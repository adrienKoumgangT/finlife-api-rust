use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::investments::{
    investment_command::*,
    investment_dto::*,
    investment_model::{Portfolio, Position, Trade},
    investment_repo::{InvestmentRepository, InvestmentRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};


#[async_trait]
pub trait InvestmentInterface {
    
    async fn create_portfolio(&self, cmd: PortfolioCreateCommand) -> Result<PortfolioResponse, AppError>;
    
    async fn update_portfolio(&self, cmd: PortfolioUpdateCommand) -> Result<Option<PortfolioResponse>, AppError>;
    
    async fn get_portfolio(&self, cmd: PortfolioGetCommand) -> Result<Option<PortfolioResponse>, AppError>;
    
    async fn delete_portfolio(&self, cmd: PortfolioDeleteCommand) -> Result<(), AppError>;
    
    async fn list_portfolios(&self, cmd: PortfolioListByUserCommand) -> Result<Vec<PortfolioResponse>, AppError>;
    
    

    async fn create_position(&self, cmd: PositionCreateCommand) -> Result<PositionResponse, AppError>;
    
    async fn update_position(&self, cmd: PositionUpdateCommand) -> Result<Option<PositionResponse>, AppError>;
    
    async fn get_position(&self, cmd: PositionGetCommand) -> Result<Option<PositionResponse>, AppError>;
    
    async fn delete_position(&self, cmd: PositionDeleteCommand) -> Result<(), AppError>;
    
    async fn list_positions(&self, cmd: PositionListByPortfolioCommand) -> Result<Vec<PositionResponse>, AppError>;
    
    

    async fn create_trade(&self, cmd: TradeCreateCommand) -> Result<TradeResponse, AppError>;
    
    async fn update_trade(&self, cmd: TradeUpdateCommand) -> Result<Option<TradeResponse>, AppError>;
    
    async fn get_trade(&self, cmd: TradeGetCommand) -> Result<Option<TradeResponse>, AppError>;
    
    async fn delete_trade(&self, cmd: TradeDeleteCommand) -> Result<(), AppError>;
    
    async fn list_trades(&self, cmd: TradeListByPositionCommand) -> Result<Vec<TradeResponse>, AppError>;
    
}

#[derive(Clone)]
pub struct InvestmentService {
    repo: InvestmentRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for InvestmentService {
    fn from(app_state: &AppState) -> Self {
        Self {
            repo: InvestmentRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl InvestmentService {
    fn ttl(&self) -> Option<u64> { Some(60 * 60) }

    // Keys
    fn key_portfolio(&self, id: &Uuid) -> String { format!("portfolio:{}", id) }
    fn key_portfolios_list(&self, uid: &Uuid) -> String { format!("user:{}:portfolios", uid) }
    fn key_position(&self, id: &Uuid) -> String { format!("position:{}", id) }
    fn key_positions_list(&self, pid: &Uuid) -> String { format!("portfolio:{}:positions", pid) }
    fn key_trade(&self, id: &Uuid) -> String { format!("trade:{}", id) }
    fn key_trades_list(&self, pid: &Uuid) -> String { format!("position:{}:trades", pid) }

    
    async fn clear_portfolio_cache(&self, pid: &Uuid, uid: &Uuid) -> Result<(), AppError> {
        if let Some(r) = &self.redis_pool {
            let _ = delete_key(r, &self.key_portfolio(pid)).await
                .map_err(AppError::Internal)?;
            let _ = delete_key(r, &self.key_portfolios_list(uid)).await
                .map_err(AppError::Internal)?;
        }
        
        Ok(())
    }
    async fn clear_position_cache(&self, pos_id: &Uuid, port_id: &Uuid) -> Result<(), AppError> {
        if let Some(r) = &self.redis_pool {
            let _ = delete_key(r, &self.key_position(pos_id)).await
                .map_err(AppError::Internal)?;
            let _ = delete_key(r, &self.key_positions_list(port_id)).await
                .map_err(AppError::Internal)?;
        }

        Ok(())
    }
    async fn clear_trade_cache(&self, trade_id: &Uuid, pos_id: &Uuid) -> Result<(), AppError> {
        if let Some(r) = &self.redis_pool {
            let _ = delete_key(r, &self.key_trade(trade_id)).await
                .map_err(AppError::Internal)?;
            let _ = delete_key(r, &self.key_trades_list(pos_id)).await
                .map_err(AppError::Internal)?;
        }

        Ok(())
    }
}

#[async_trait]
impl InvestmentInterface for InvestmentService {

    // PORTFOLIOS
    async fn create_portfolio(&self, cmd: PortfolioCreateCommand) -> Result<PortfolioResponse, AppError> {
        let uid = cmd.user_id.clone();
        
        let p = self.repo.create_portfolio(Portfolio::from(cmd)).await
            .map_err(AppError::Internal)?;
        
        self.clear_portfolio_cache(&p.id.unwrap(), &uid).await?;
        
        Ok(PortfolioResponse::from(p))
    }
    async fn update_portfolio(&self, cmd: PortfolioUpdateCommand) -> Result<Option<PortfolioResponse>, AppError> {
        let p = self.repo.update_portfolio(cmd.portfolio_id, cmd.name, cmd.base_currency_code, cmd.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        if let Some(ref port) = p {
            self.clear_portfolio_cache(&port.id.unwrap(), &port.user_id).await?;
        }
        
        Ok(p.map(PortfolioResponse::from))
    }
    async fn get_portfolio(&self, cmd: PortfolioGetCommand) -> Result<Option<PortfolioResponse>, AppError> {
        if let Some(r) = &self.redis_pool {
            if let Ok(Some(c)) = get_key::<PortfolioResponse>(r, &self.key_portfolio(&cmd.portfolio_id)).await { return Ok(Some(c)); }
        }
        
        let p = self.repo.get_portfolio(cmd.portfolio_id, cmd.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        Ok(p.map(PortfolioResponse::from))
    }
    async fn delete_portfolio(&self, cmd: PortfolioDeleteCommand) -> Result<(), AppError> {
        self.repo.delete_portfolio(cmd.portfolio_id, cmd.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        self.clear_portfolio_cache(&cmd.portfolio_id, &cmd.auth_user.user_id).await?;
        
        Ok(())
    }
    async fn list_portfolios(&self, cmd: PortfolioListByUserCommand) -> Result<Vec<PortfolioResponse>, AppError> {
        let (limit, offset, _) = extract_pagination_data(cmd.pagination);
        
        if let Some(r) = &self.redis_pool {
            if let Ok(Some(c)) = get_key::<Vec<PortfolioResponse>>(r, &self.key_portfolios_list(&cmd.user_id)).await { return Ok(c); }
        }
        
        let p = self.repo.list_portfolios(cmd.user_id, limit, offset).await
            .map_err(AppError::Internal)?;
        let res: Vec<PortfolioResponse> = p.into_iter().map(PortfolioResponse::from).collect();
        
        if let Some(r) = &self.redis_pool {
            let _ = set_key(r, &self.key_portfolios_list(&cmd.user_id), &res, self.ttl()).await
                .map_err(AppError::Internal)?;
        }
        
        Ok(res)
    }
    
    

    // POSITIONS
    async fn create_position(&self, cmd: PositionCreateCommand) -> Result<PositionResponse, AppError> {
        let user_id = cmd.auth_user.user_id.clone();
        let pid = cmd.portfolio_id.clone();
        
        let p = self.repo.create_position(Position::from(cmd), user_id).await
            .map_err(AppError::Internal)?;
        
        self.clear_position_cache(&p.id.unwrap(), &pid).await?;
        
        Ok(PositionResponse::from(p))
    }
    async fn update_position(&self, cmd: PositionUpdateCommand) -> Result<Option<PositionResponse>, AppError> {
        let p = self.repo.update_position(cmd.position_id, cmd.symbol, cmd.name, cmd.status, cmd.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        if let Some(ref pos) = p {
            self.clear_position_cache(&pos.id.unwrap(), &pos.portfolio_id).await?;
        }
        
        Ok(p.map(PositionResponse::from))
    }
    async fn get_position(&self, cmd: PositionGetCommand) -> Result<Option<PositionResponse>, AppError> {
        let p = self.repo.get_position(cmd.position_id, cmd.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        Ok(p.map(PositionResponse::from))
    }
    async fn delete_position(&self, cmd: PositionDeleteCommand) -> Result<(), AppError> {
        if let Ok(Some(p)) = self.repo.get_position(cmd.position_id, cmd.auth_user.user_id.clone()).await {
            self.repo.delete_position(cmd.position_id, cmd.auth_user.user_id).await
                .map_err(AppError::Internal)?;
            
            self.clear_position_cache(&cmd.position_id, &p.portfolio_id).await?;
        }
        Ok(())
    }
    async fn list_positions(&self, cmd: PositionListByPortfolioCommand) -> Result<Vec<PositionResponse>, AppError> {
        let (limit, offset, _) = extract_pagination_data(cmd.pagination);
        
        let p = self.repo.list_positions(cmd.portfolio_id, cmd.auth_user.user_id, limit, offset).await
            .map_err(AppError::Internal)?;
        
        Ok(p.into_iter().map(PositionResponse::from).collect())
    }

    
    
    // TRADES
    async fn create_trade(&self, cmd: TradeCreateCommand) -> Result<TradeResponse, AppError> {
        let user_id = cmd.auth_user.user_id.clone();
        let pid = cmd.position_id.clone();
        
        let t = self.repo.create_trade(Trade::from(cmd), user_id).await
            .map_err(AppError::Internal)?;
        
        self.clear_trade_cache(&t.id.unwrap(), &pid).await?;
        
        Ok(TradeResponse::from(t))
    }
    async fn update_trade(&self, cmd: TradeUpdateCommand) -> Result<Option<TradeResponse>, AppError> {
        let t = self.repo.update_trade(
            cmd.trade_id, cmd.trade_type, 
            cmd.trade_date, cmd.quantity, cmd.price_minor, cmd.fees_minor, cmd.currency_code, 
            cmd.auth_user.user_id
        ).await.map_err(AppError::Internal)?;
        
        if let Some(ref tr) = t {
            self.clear_trade_cache(&tr.id.unwrap(), &tr.position_id).await?;
        }
        
        Ok(t.map(TradeResponse::from))
    }
    async fn get_trade(&self, cmd: TradeGetCommand) -> Result<Option<TradeResponse>, AppError> {
        let t = self.repo.get_trade(cmd.trade_id, cmd.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        Ok(t.map(TradeResponse::from))
    }
    async fn delete_trade(&self, cmd: TradeDeleteCommand) -> Result<(), AppError> {
        if let Ok(Some(t)) = self.repo.get_trade(cmd.trade_id, cmd.auth_user.user_id.clone()).await {
            self.repo.delete_trade(cmd.trade_id, cmd.auth_user.user_id).await
                .map_err(AppError::Internal)?;
            
            self.clear_trade_cache(&cmd.trade_id, &t.position_id).await?;
        }
        
        Ok(())
    }
    async fn list_trades(&self, cmd: TradeListByPositionCommand) -> Result<Vec<TradeResponse>, AppError> {
        let (limit, offset, _) = extract_pagination_data(cmd.pagination);
        
        let t = self.repo.list_trades(cmd.position_id, cmd.auth_user.user_id, limit, offset).await
            .map_err(AppError::Internal)?;
        
        Ok(t.into_iter().map(TradeResponse::from).collect())
    }
}
