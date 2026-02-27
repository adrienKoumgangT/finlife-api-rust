use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::modules::investments::investment_model::*;
use crate::shared::state::AppState;


#[async_trait]
pub trait InvestmentRepositoryInterface {

    // Portfolios
    async fn create_portfolio(&self, p: Portfolio) -> Result<Portfolio, Error>;

    async fn get_portfolio(&self, id: Uuid, user_id: Uuid) -> Result<Option<Portfolio>, Error>;

    async fn update_portfolio(&self, id: Uuid, name: String, currency: String, user_id: Uuid) -> Result<Option<Portfolio>, Error>;

    async fn delete_portfolio(&self, id: Uuid, user_id: Uuid) -> Result<(), Error>;

    async fn list_portfolios(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Portfolio>, Error>;


    // Positions
    async fn create_position(&self, pos: Position, user_id: Uuid) -> Result<Position, Error>;

    async fn get_position(&self, id: Uuid, user_id: Uuid) -> Result<Option<Position>, Error>;

    async fn update_position(&self, id: Uuid, symbol: String, name: String, status: PositionStatus, user_id: Uuid) -> Result<Option<Position>, Error>;

    async fn delete_position(&self, id: Uuid, user_id: Uuid) -> Result<(), Error>;

    async fn list_positions(&self, portfolio_id: Uuid, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Position>, Error>;


    // Trades
    async fn create_trade(&self, t: Trade, user_id: Uuid) -> Result<Trade, Error>;

    async fn get_trade(&self, id: Uuid, user_id: Uuid) -> Result<Option<Trade>, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update_trade(&self, id: Uuid, t_type: TradeType, date: NaiveDateTime, qty: Decimal, price: i64, fees: i64, curr: String, user_id: Uuid) -> Result<Option<Trade>, Error>;

    async fn delete_trade(&self, id: Uuid, user_id: Uuid) -> Result<(), Error>;

    async fn list_trades(&self, position_id: Uuid, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Trade>, Error>;

}

#[derive(Clone)]
pub struct InvestmentRepository {
    pool: MySqlPool,
}

impl From<&AppState> for InvestmentRepository {
    fn from(app_state: &AppState) -> Self { Self { pool: app_state.mysql_pool.clone() } }
}

#[async_trait]
impl InvestmentRepositoryInterface for InvestmentRepository {
    // --- PORTFOLIOS ---
    async fn create_portfolio(&self, p: Portfolio) -> Result<Portfolio, Error> {
        let new_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO investment_portfolios (id, user_id, name, base_currency_code) VALUES (?, ?, ?, ?)",
            new_id, p.user_id, p.name, p.base_currency_code
        ).execute(&self.pool).await?;
        self.get_portfolio(new_id, p.user_id).await?.ok_or_else(|| Error::msg("Not found"))
    }

    async fn get_portfolio(&self, id: Uuid, user_id: Uuid) -> Result<Option<Portfolio>, Error> {
        let p = sqlx::query_as!(
            Portfolio,
            r#"SELECT id AS "id: _",
                    user_id AS "user_id: _",
                    name, base_currency_code,
                    created_at, updated_at
            FROM investment_portfolios
            WHERE id = ? AND user_id = ?"#,
            id, user_id
        ).fetch_optional(&self.pool).await?;
        Ok(p)
    }

    async fn update_portfolio(&self, id: Uuid, name: String, currency: String, user_id: Uuid) -> Result<Option<Portfolio>, Error> {
        sqlx::query!(
            "UPDATE investment_portfolios SET name = ?, base_currency_code = ? WHERE id = ? AND user_id = ?",
            name, currency, id, user_id
        ).execute(&self.pool).await?;

        self.get_portfolio(id, user_id).await
    }

    async fn delete_portfolio(&self, id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM investment_portfolios WHERE id = ? AND user_id = ?",
            id, user_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn list_portfolios(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Portfolio>, Error> {
        let l = limit.unwrap_or(100) as i64; let o = offset.unwrap_or(0) as i64;

        let res = sqlx::query_as!(
            Portfolio,
            r#"SELECT id AS "id: _",
                    user_id AS "user_id: _",
                    name, base_currency_code,
                    created_at, updated_at
            FROM investment_portfolios
            WHERE user_id = ?
            ORDER BY name ASC
            LIMIT ? OFFSET ?"#,
            user_id, l, o
        ).fetch_all(&self.pool).await?;

        Ok(res)
    }



    // --- POSITIONS ---
    async fn create_position(&self, pos: Position, user_id: Uuid) -> Result<Position, Error> {
        let port_check = sqlx::query!(
            "SELECT id FROM investment_portfolios WHERE id = ? AND user_id = ?",
            pos.portfolio_id, user_id
        ).fetch_optional(&self.pool).await?;
        if port_check.is_none() { return Err(Error::msg("Unauthorized")); }

        let new_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO investment_positions (id, portfolio_id, symbol, name, status) VALUES (?, ?, ?, ?, ?)",
            new_id, pos.portfolio_id, pos.symbol, pos.name, pos.status.as_str()
        ).execute(&self.pool).await?;

        self.get_position(new_id, user_id).await?.ok_or_else(|| Error::msg("Not found"))
    }

    async fn get_position(&self, id: Uuid, user_id: Uuid) -> Result<Option<Position>, Error> {
        let pos = sqlx::query_as!(
            Position,
            r#"SELECT pos.id AS "id: _",
                    pos.portfolio_id AS "portfolio_id: _",
                    pos.symbol, pos.name,
                    pos.status AS "status: String",
                    pos.created_at, pos.updated_at
               FROM investment_positions pos JOIN investment_portfolios p ON p.id = pos.portfolio_id
               WHERE pos.id = ? AND p.user_id = ?"#,
            id, user_id
        ).fetch_optional(&self.pool).await?;

        Ok(pos)
    }

    async fn update_position(&self, id: Uuid, symbol: String, name: String, status: PositionStatus, user_id: Uuid) -> Result<Option<Position>, Error> {
        sqlx::query!(
            "UPDATE investment_positions pos
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            SET pos.symbol = ?, pos.name = ?, pos.status = ?
            WHERE pos.id = ? AND p.user_id = ?",
            symbol, name, status.as_str(), id, user_id
        ).execute(&self.pool).await?;

        self.get_position(id, user_id).await
    }

    async fn delete_position(&self, id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "DELETE pos
            FROM investment_positions pos
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            WHERE pos.id = ? AND p.user_id = ?",
            id, user_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn list_positions(&self, portfolio_id: Uuid, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Position>, Error> {
        let l = limit.unwrap_or(100) as i64; let o = offset.unwrap_or(0) as i64;

        let res = sqlx::query_as!(
            Position,
            r#"SELECT pos.id AS "id: _",
                    pos.portfolio_id AS "portfolio_id: _",
                    pos.symbol, pos.name,
                    pos.status AS "status: String",
                    pos.created_at, pos.updated_at
            FROM investment_positions pos
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            WHERE pos.portfolio_id = ? AND p.user_id = ?
            ORDER BY pos.symbol ASC
            LIMIT ? OFFSET ?"#,
            portfolio_id, user_id, l, o
        ).fetch_all(&self.pool).await?;

        Ok(res)
    }



    // --- TRADES ---
    async fn create_trade(&self, t: Trade, user_id: Uuid) -> Result<Trade, Error> {
        let pos_check = sqlx::query!(
            "SELECT pos.id
            FROM investment_positions pos
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            WHERE pos.id = ? AND p.user_id = ?", t.position_id,
            user_id
        ).fetch_optional(&self.pool).await?;
        if pos_check.is_none() { return Err(Error::msg("Unauthorized")); }

        let new_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO investment_trades (id, position_id, trade_type, trade_date, quantity, price_minor, fees_minor, currency_code) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            new_id, t.position_id, t.trade_type.as_str(), t.trade_date, t.quantity, t.price_minor, t.fees_minor, t.currency_code
        ).execute(&self.pool).await?;

        self.get_trade(new_id, user_id).await?.ok_or_else(|| Error::msg("Not found"))
    }

    async fn get_trade(&self, id: Uuid, user_id: Uuid) -> Result<Option<Trade>, Error> {
        let res = sqlx::query_as!(
            Trade,
            r#"SELECT t.id AS "id: _",
                    t.position_id AS "position_id: _",
                    t.trade_type AS "trade_type: String",
                    t.trade_date, t.quantity, t.price_minor, t.fees_minor, t.currency_code,
                    t.created_at
            FROM investment_trades t
                JOIN investment_positions pos ON pos.id = t.position_id
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            WHERE t.id = ? AND p.user_id = ?"#,
            id, user_id
        ).fetch_optional(&self.pool).await?;

        Ok(res)
    }

    async fn update_trade(&self, id: Uuid, t_type: TradeType, date: NaiveDateTime, qty: Decimal, price: i64, fees: i64, curr: String, user_id: Uuid) -> Result<Option<Trade>, Error> {
        sqlx::query!(
            "UPDATE investment_trades t
                JOIN investment_positions pos ON pos.id = t.position_id
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            SET t.trade_type = ?, t.trade_date = ?, t.quantity = ?, t.price_minor = ?, t.fees_minor = ?, t.currency_code = ?
            WHERE t.id = ? AND p.user_id = ?",
            t_type.as_str(), date, qty, price, fees, curr,
            id, user_id
        ).execute(&self.pool).await?;

        self.get_trade(id, user_id).await
    }

    async fn delete_trade(&self, id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "DELETE t
            FROM investment_trades t
                JOIN investment_positions pos ON pos.id = t.position_id
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            WHERE t.id = ? AND p.user_id = ?",
            id, user_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn list_trades(&self, position_id: Uuid, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Trade>, Error> {
        let l = limit.unwrap_or(100) as i64; let o = offset.unwrap_or(0) as i64;

        let res = sqlx::query_as!(
            Trade,
            r#"SELECT t.id AS "id: _",
                    t.position_id AS "position_id: _",
                    t.trade_type AS "trade_type: String",
                    t.trade_date, t.quantity, t.price_minor, t.fees_minor, t.currency_code,
                    t.created_at
            FROM investment_trades t
                JOIN investment_positions pos ON pos.id = t.position_id
                JOIN investment_portfolios p ON p.id = pos.portfolio_id
            WHERE t.position_id = ? AND p.user_id = ?
            ORDER BY t.trade_date DESC
            LIMIT ? OFFSET ?"#,
            position_id, user_id, l, o
        ).fetch_all(&self.pool).await?;

        Ok(res)
    }
}
