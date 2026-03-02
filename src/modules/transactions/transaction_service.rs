use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::transactions::{
    transaction_command::*,
    transaction_dto::*,
    transaction_model::Transaction,
    transaction_repo::{TransactionRepository, TransactionRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};
use crate::shared::response::PaginatedResponse;

#[async_trait]
pub trait TransactionInterface {

    async fn get(&self, command: TransactionGetCommand) -> Result<Option<TransactionResponse>, AppError>;

    async fn create(&self, command: TransactionCreateCommand) -> Result<TransactionResponse, AppError>;

    async fn update(&self, command: TransactionUpdateCommand) -> Result<Option<TransactionResponse>, AppError>;

    async fn delete(&self, command: TransactionDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: TransactionListByUserCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;
    
    async fn get_by_user_filter(&self, command: TransactionListFilterByUserCommand) -> Result<Vec<TransactionResponse>, AppError>;

    async fn get_by_account(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;

    async fn get_by_category(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;

    async fn get_by_payee(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;

    async fn get_by_person(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;

    async fn get_by_location(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;

    async fn get_by_project(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;

    async fn get_by_goal(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError>;
    
    
    
    async fn get_12_months_cash_flow(&self, command: TransactionGetStatCommand) -> Result<Vec<MonthlyFlowResponse>, AppError>;
    
    async fn get_12_months_category_expenses(&self, command: TransactionGetStatCommand) -> Result<Vec<MonthlyCategoryExpenseResponse>, AppError>;

}

#[derive(Clone)]
pub struct TransactionService {
    transaction_repo: TransactionRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for TransactionService {
    fn from(app_state: &AppState) -> Self {
        Self {
            transaction_repo: TransactionRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl TransactionService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 10) }

    fn form_redis_key_transaction(&self, key: &Uuid) -> String { format!("transaction:{}", key) }

    async fn cache_transaction(&self, transaction: &TransactionResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_transaction(&transaction.transaction_id).as_str(),
                &transaction,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_transaction(&self, key: &Uuid) -> Result<Option<TransactionResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let transaction_cache: Option<TransactionResponse> = get_key(
                &redis_pool,
                self.form_redis_key_transaction(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(transaction_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, _user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_transaction(key).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_transaction(&self, transaction: Result<Option<Transaction>>, _user: &Uuid) -> Result<Option<TransactionResponse>, AppError> {
        let transaction = transaction.map_err(AppError::Internal)?;

        if let Some(tx) = transaction {
            let response = TransactionResponse::from(tx);
            self.cache_transaction(&response).await?;

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl TransactionInterface for TransactionService {
    async fn get(&self, command: TransactionGetCommand) -> Result<Option<TransactionResponse>, AppError> {
        let cache = self.get_cache_transaction(&command.transaction_id).await?;
        if let Some(transaction) = cache {
            return Ok(Some(transaction));
        }

        let transaction = self.transaction_repo.get(command.transaction_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_transaction(transaction, &command.auth_user.user_id).await
    }

    async fn create(&self, command: TransactionCreateCommand) -> Result<TransactionResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let transaction_create = Transaction::from(command);

        let transaction = self.transaction_repo.create(transaction_create, Some(meta_user)).await
            .map_err(AppError::Internal)?;
        let response = TransactionResponse::from(transaction);

        self.cache_transaction(&response).await?;

        Ok(response)
    }

    async fn update(&self, command: TransactionUpdateCommand) -> Result<Option<TransactionResponse>, AppError> {
        let transaction = self.transaction_repo.update(
            command.transaction_id, command.account_id, command.occurred_at, command.amount_minor,
            command.currency_code, command.base_amount_minor, command.base_currency_code,
            command.fx_rate_id, command.category_id, command.payee_id, command.person_id,
            command.location_id, command.note, command.project_id, command.goal_id, command.status,
            Some(command.auth_user.user_id)
        ).await;

        self.handle_res_opt_transaction(transaction, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: TransactionDeleteCommand) -> Result<(), AppError> {
        self.transaction_repo.delete(command.transaction_id.clone(), Some(command.auth_user.user_id)).await
            .map_err(AppError::Internal)?;
        self.delete_cache(&command.transaction_id, &command.auth_user.user_id).await?;
        Ok(())
    }

    async fn get_by_user(&self, command: TransactionListByUserCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_user(
            command.user_id, limit, offset
        ).await.map_err(AppError::Internal)?;
        
        let total = self.transaction_repo.count_by_user(command.user_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();
        
        let response = PaginatedResponse { items: data, total, page: 0, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_user_filter(&self, command: TransactionListFilterByUserCommand) -> Result<Vec<TransactionResponse>, AppError> {
        let transactions = self.transaction_repo.get_by_user_filter(command.user_id, Some(command.year), Some(command.month)).await
            .map_err(AppError::Internal)?;

        let response: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();
        
        Ok(response)
    }

    async fn get_by_account(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }
        
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_account(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_account(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_category(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }

        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_category(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_category(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_payee(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }

        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_payee(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_payee(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_person(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }
        
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_person(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_person(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_location(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }

        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_location(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_location(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_project(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }

        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_project(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_project(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_by_goal(&self, command: TransactionListByCommand) -> Result<PaginatedResponse<TransactionResponse>, AppError> {
        let mut page: i64 = 0;

        if let Some(pagination) = command.pagination.clone() {
            if let Some(page_request) = pagination.page {
                page = page_request as i64;
            }
        }

        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let transactions = self.transaction_repo.get_by_goal(
            command.user_id, command.person_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let total = self.transaction_repo.count_by_goal(command.user_id, command.person_id).await
            .map_err(AppError::Internal)?;

        let data: Vec<TransactionResponse> = transactions.into_iter().map(TransactionResponse::from).collect();

        let response = PaginatedResponse { items: data, total, page, page_size: limit.unwrap_or(100) as i64 };

        Ok(response)
    }

    async fn get_12_months_cash_flow(&self, command: TransactionGetStatCommand) -> Result<Vec<MonthlyFlowResponse>, AppError> {
        let records = self.transaction_repo.get_12_months_cash_flow(command.user_id).await?;
        
        let data: Vec<MonthlyFlowResponse> = records.into_iter()
            .filter(|c| c.month.is_some())
            .map(MonthlyFlowResponse::from)
            .collect();
        
        Ok(data)
    }

    async fn get_12_months_category_expenses(&self, command: TransactionGetStatCommand) -> Result<Vec<MonthlyCategoryExpenseResponse>, AppError> {
        let records = self.transaction_repo.get_12_months_category_expenses(command.user_id).await?;

        let data: Vec<MonthlyCategoryExpenseResponse> = records.into_iter()
            .filter(|c| c.month.is_some())
            .map(MonthlyCategoryExpenseResponse::from)
            .collect();

        Ok(data)
    }
}
