use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use chrono::Datelike;
use uuid::Uuid;

use crate::modules::budgets::{
    budget_command::*,
    budget_dto::*,
    budget_model::{Budget, BudgetEnvelope},
    budget_repo::{BudgetRepository, BudgetRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
};

#[async_trait]
pub trait BudgetInterface {
    // --- Budgets ---

    async fn get_budget(&self, command: BudgetGetCommand) -> Result<Option<BudgetResponse>, AppError>;

    async fn create_budget(&self, command: BudgetCreateCommand) -> Result<BudgetResponse, AppError>;

    async fn update_budget(&self, command: BudgetUpdateCommand) -> Result<Option<BudgetResponse>, AppError>;

    async fn delete_budget(&self, command: BudgetDeleteCommand) -> Result<(), AppError>;

    async fn get_budgets_by_user(&self, command: BudgetListByUserCommand) -> Result<Vec<BudgetResponse>, AppError>;


    // --- Budget Envelopes ---

    async fn get_envelope(&self, command: BudgetEnvelopeGetCommand) -> Result<Option<BudgetEnvelopeResponse>, AppError>;

    async fn create_envelope(&self, command: BudgetEnvelopeCreateCommand) -> Result<BudgetEnvelopeResponse, AppError>;

    async fn update_envelope(&self, command: BudgetEnvelopeUpdateCommand) -> Result<Option<BudgetEnvelopeResponse>, AppError>;

    async fn delete_envelope(&self, command: BudgetEnvelopeDeleteCommand) -> Result<(), AppError>;

    async fn get_envelopes_by_budget(&self, command: BudgetEnvelopeListByBudgetCommand) -> Result<Vec<BudgetEnvelopeResponse>, AppError>;

}

#[derive(Clone)]
pub struct BudgetService {
    budget_repo: BudgetRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for BudgetService {
    fn from(app_state: &AppState) -> Self {
        Self {
            budget_repo: BudgetRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl BudgetService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    // --- Redis Keys for Budgets ---
    fn form_redis_key_budget(&self, key: &Uuid) -> String { format!("budget:{}", key) }
    fn form_redis_key_list_budgets_by_user(&self, user: &Uuid, year: &u32) -> String { format!("user:{}:budgets:year:{}", user, year) }

    // --- Redis Keys for Envelopes ---
    fn form_redis_key_envelope(&self, key: &Uuid) -> String { format!("envelope:{}", key) }
    fn form_redis_key_list_envelopes_by_budget(&self, budget_id: &Uuid) -> String { format!("budget:{}:envelopes", budget_id) }

    // ==========================================
    //            BUDGET CACHING LOGIC
    // ==========================================

    async fn cache_budget(&self, budget: &BudgetResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                redis_pool,
                self.form_redis_key_budget(&budget.budget_id).as_str(),
                &budget,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_budgets_by_user(&self, user_id: &Uuid, year: &u32, budgets: &Vec<BudgetResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                redis_pool,
                self.form_redis_key_list_budgets_by_user(user_id, year).as_str(),
                &budgets,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_budget(&self, key: &Uuid) -> Result<Option<BudgetResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<BudgetResponse> = get_key(
                redis_pool,
                self.form_redis_key_budget(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(cache);
        }
        Ok(None)
    }

    async fn get_cache_list_budgets_by_user(&self, user_id: &Uuid, year: &u32) -> Result<Option<Vec<BudgetResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<BudgetResponse>> = get_key(
                redis_pool, self.form_redis_key_list_budgets_by_user(user_id, year).as_str()
            ).await.map_err(AppError::Internal)?;

            return Ok(cache);
        }
        Ok(None)
    }

    async fn delete_budget_cache(&self, budget_id: &Uuid, user_id: &Uuid, year: &u32) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_budget(budget_id).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(redis_pool, self.form_redis_key_list_budgets_by_user(user_id, year).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn delete_list_budget_cache(&self, user_id: &Uuid, year: &u32) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_list_budgets_by_user(user_id, year).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(())
    }

    async fn handle_res_opt_budget(&self, budget: Option<Budget>, delete_cache_list: bool, user: &Uuid) -> Result<Option<BudgetResponse>, AppError> {
        if let Some(b) = budget {
            let response = BudgetResponse::from(b);
            self.cache_budget(&response).await?;
            if delete_cache_list { self.delete_list_budget_cache(user, &(response.month.year() as u32)).await?; }

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }

    // ==========================================
    //           ENVELOPE CACHING LOGIC
    // ==========================================

    async fn cache_envelope(&self, envelope: &BudgetEnvelopeResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                redis_pool,
                self.form_redis_key_envelope(&envelope.envelope_id).as_str(),
                &envelope,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_envelopes_by_budget(&self, budget_id: &Uuid, envelopes: &Vec<BudgetEnvelopeResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                redis_pool,
                self.form_redis_key_list_envelopes_by_budget(budget_id).as_str(),
                &envelopes,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_envelope(&self, key: &Uuid) -> Result<Option<BudgetEnvelopeResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<BudgetEnvelopeResponse> = get_key(
                redis_pool,
                self.form_redis_key_envelope(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(cache);
        }
        Ok(None)
    }

    async fn get_cache_envelopes_by_budget(&self, budget_id: &Uuid) -> Result<Option<Vec<BudgetEnvelopeResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<BudgetEnvelopeResponse>> = get_key(
                redis_pool, self.form_redis_key_list_envelopes_by_budget(budget_id).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(cache);
        }

        Ok(None)
    }

    async fn delete_envelope_cache(&self, envelope_id: &Uuid, budget_id: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_envelope(envelope_id).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(redis_pool, self.form_redis_key_list_envelopes_by_budget(budget_id).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn delete_list_envelopes_by_budget(&self, budget_id: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_list_envelopes_by_budget(budget_id).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(())
    }

    async fn handle_res_opt_envelope(&self, envelope: anyhow::Result<Option<BudgetEnvelope>>) -> Result<Option<BudgetEnvelopeResponse>, AppError> {
        let envelope = envelope.map_err(AppError::Internal)?;

        if let Some(e) = envelope {
            let response = BudgetEnvelopeResponse::from(e);
            self.cache_envelope(&response).await?;
            self.delete_list_envelopes_by_budget(&response.budget_id).await?;

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl BudgetInterface for BudgetService {

    // ==========================================
    //                 BUDGETS
    // ==========================================

    async fn get_budget(&self, command: BudgetGetCommand) -> Result<Option<BudgetResponse>, AppError> {
        if let Some(budget) = self.get_cache_budget(&command.budget_id).await? {
            return Ok(Some(budget));
        }

        let budget = self.budget_repo.get_budget(command.budget_id, command.auth_user.user_id).await?;
        self.handle_res_opt_budget(budget, false, &command.auth_user.user_id).await
    }

    async fn create_budget(&self, command: BudgetCreateCommand) -> Result<BudgetResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let budget_create = Budget::from(command);

        let budget = self.budget_repo.create_budget(budget_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = BudgetResponse::from(budget);

        self.cache_budget(&response).await?;
        self.delete_list_budget_cache(&meta_user, &(response.month.year() as u32)).await?;

        Ok(response)
    }

    async fn update_budget(&self, command: BudgetUpdateCommand) -> Result<Option<BudgetResponse>, AppError> {
        let budget = self.budget_repo.update_budget(
            command.budget_id, command.base_currency_code, command.person_id,
            command.status, command.auth_user.user_id
        ).await?;

        self.handle_res_opt_budget(budget, true, &command.auth_user.user_id).await
    }

    async fn delete_budget(&self, command: BudgetDeleteCommand) -> Result<(), AppError> {
        let budget_opt = self.budget_repo.get_budget(command.budget_id.clone(), command.auth_user.user_id.clone()).await
            .map_err(AppError::Internal)?;

        if let Some(budget) = budget_opt {
            self.budget_repo.delete_budget(command.budget_id.clone(), command.auth_user.user_id).await
                .map_err(AppError::Internal)?;
            self.delete_budget_cache(&command.budget_id, &budget.user_id, &(budget.month.year() as u32)).await?;
        }
        Ok(())
    }

    async fn get_budgets_by_user(&self, command: BudgetListByUserCommand) -> Result<Vec<BudgetResponse>, AppError> {
        if let Some(budgets) = self.get_cache_list_budgets_by_user(&command.user_id, &command.year).await? { return Ok(budgets); }

        let budgets = self.budget_repo.get_budgets_by_user(command.user_id, command.year).await
            .map_err(AppError::Internal)?;

        let response: Vec<BudgetResponse> = budgets.into_iter().map(BudgetResponse::from).collect();
        self.cache_budgets_by_user(&command.user_id, &command.year, &response).await?;

        Ok(response)
    }

    // ==========================================
    //            BUDGET ENVELOPES
    // ==========================================

    async fn get_envelope(&self, command: BudgetEnvelopeGetCommand) -> Result<Option<BudgetEnvelopeResponse>, AppError> {
        if let Some(envelope) = self.get_cache_envelope(&command.envelope_id).await? {
            return Ok(Some(envelope));
        }

        let envelope = self.budget_repo.get_envelope(command.envelope_id, command.auth_user.user_id).await;
        self.handle_res_opt_envelope(envelope).await
    }

    async fn create_envelope(&self, command: BudgetEnvelopeCreateCommand) -> Result<BudgetEnvelopeResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let envelope_create = BudgetEnvelope::from(command);

        let envelope = self.budget_repo.create_envelope(envelope_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = BudgetEnvelopeResponse::from(envelope);

        self.cache_envelope(&response).await?;
        self.delete_list_envelopes_by_budget(&response.budget_id).await?;

        Ok(response)
    }

    async fn update_envelope(&self, command: BudgetEnvelopeUpdateCommand) -> Result<Option<BudgetEnvelopeResponse>, AppError> {
        let envelope = self.budget_repo.update_envelope(
            command.envelope_id, command.planned_base_minor, command.carryover_base_minor,
            command.rollover_rule, command.auth_user.user_id
        ).await;

        self.handle_res_opt_envelope(envelope).await
    }

    async fn delete_envelope(&self, command: BudgetEnvelopeDeleteCommand) -> Result<(), AppError> {
        let envelope_opt = self.budget_repo.get_envelope(command.envelope_id.clone(), command.auth_user.user_id.clone()).await
            .map_err(AppError::Internal)?;

        if let Some(envelope) = envelope_opt {
            self.budget_repo.delete_envelope(command.envelope_id.clone(), command.auth_user.user_id).await
                .map_err(AppError::Internal)?;
            self.delete_envelope_cache(&command.envelope_id, &envelope.budget_id).await?;
        }
        Ok(())
    }

    async fn get_envelopes_by_budget(&self, command: BudgetEnvelopeListByBudgetCommand) -> Result<Vec<BudgetEnvelopeResponse>, AppError> {
        let cache = self.get_cache_envelopes_by_budget(&command.budget_id).await?;
        if let Some(envelopes) = cache { return Ok(envelopes); }

        let envelopes = self.budget_repo.get_envelopes_by_budget(command.budget_id, command.auth_user.user_id).await
            .map_err(AppError::Internal)?;

        let response: Vec<BudgetEnvelopeResponse> = envelopes.into_iter().map(BudgetEnvelopeResponse::from).collect();
        self.cache_envelopes_by_budget(&command.budget_id, &response).await?;

        Ok(response)
    }

}
