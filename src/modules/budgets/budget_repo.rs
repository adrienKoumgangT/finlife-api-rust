use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::budgets::budget_model::{Budget, BudgetEnvelope, BudgetStatus, RolloverRule};
use crate::shared::state::AppState;


#[async_trait]
pub trait BudgetRepositoryInterface {

    async fn get_budget(&self, budget_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Budget>, Error>;

    async fn create_budget(&self, budget: Budget, user_id: Option<Uuid>) -> Result<Budget, Error>;

    async fn update_budget(&self, budget_id: Uuid, base_currency_code: String, person_id: Option<Uuid>, status: BudgetStatus, user_id: Option<Uuid>) -> Result<Option<Budget>, Error>;

    async fn delete_budget(&self, budget_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_budgets_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Budget>, Error>;



    async fn get_envelope(&self, envelope_id: Uuid, user_id: Option<Uuid>) -> Result<Option<BudgetEnvelope>, Error>;

    async fn create_envelope(&self, envelope: BudgetEnvelope, user_id: Option<Uuid>) -> Result<BudgetEnvelope, Error>;

    async fn update_envelope(&self, envelope_id: Uuid, planned: i64, carryover: i64, rule: RolloverRule, user_id: Option<Uuid>) -> Result<Option<BudgetEnvelope>, Error>;

    async fn delete_envelope(&self, envelope_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_envelopes_by_budget(&self, budget_id: Uuid, limit: Option<u32>, offset: Option<u32>, user_id: Option<Uuid>) -> Result<Vec<BudgetEnvelope>, Error>;

}


#[derive(Clone)]
pub struct BudgetRepository {
    pool: MySqlPool,
}

impl From<&AppState> for BudgetRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl BudgetRepositoryInterface for BudgetRepository {

    // ==========================================
    //                 BUDGETS
    // ==========================================
    async fn get_budget(&self, budget_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Budget>, Error> {
        let budget = sqlx::query_as!(
            Budget,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                month, base_currency_code, person_id AS "person_id: _",
                status AS "status: String", created_at, updated_at
            FROM budgets
            WHERE id = ? AND user_id = ?
            "#,
            budget_id, user_id
        ).fetch_optional(&self.pool).await?;
        Ok(budget)
    }

    async fn create_budget(&self, budget: Budget, user_id: Option<Uuid>) -> Result<Budget, Error> {
        let new_id = Uuid::new_v4();
        let status_str = budget.status.as_str();

        sqlx::query!(
            "INSERT INTO budgets (id, user_id, month, base_currency_code, person_id, status) VALUES (?, ?, ?, ?, ?, ?)",
            new_id, budget.user_id, budget.month, budget.base_currency_code, budget.person_id, status_str
        ).execute(&self.pool).await?;

        self.get_budget(new_id, user_id).await?.ok_or_else(|| Error::msg("Budget not found"))
    }

    async fn update_budget(&self, budget_id: Uuid, base_currency_code: String, person_id: Option<Uuid>, status: BudgetStatus, user_id: Option<Uuid>) -> Result<Option<Budget>, Error> {
        let status_str = status.as_str();
        sqlx::query!(
            "UPDATE budgets SET base_currency_code = ?, person_id = ?, status = ? WHERE id = ? AND user_id = ?",
            base_currency_code, person_id, status_str, budget_id, user_id
        ).execute(&self.pool).await?;

        self.get_budget(budget_id, user_id).await
    }

    async fn delete_budget(&self, budget_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM budgets WHERE id = ? AND user_id = ?", budget_id, user_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_budgets_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Budget>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let budgets = sqlx::query_as!(
            Budget,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", month, base_currency_code,
                person_id AS "person_id: _", status AS "status: String", created_at, updated_at
            FROM budgets WHERE user_id = ? ORDER BY month DESC LIMIT ? OFFSET ?
            "#,
            user_id, limit_val, offset_val
        ).fetch_all(&self.pool).await?;
        Ok(budgets)
    }

    // ==========================================
    //            BUDGET ENVELOPES
    // ==========================================
    async fn get_envelope(&self, envelope_id: Uuid, user_id: Option<Uuid>) -> Result<Option<BudgetEnvelope>, Error> {
        let env = sqlx::query_as!(
            BudgetEnvelope,
            r#"
            SELECT
                e.id AS "id: _", e.budget_id AS "budget_id: _", e.category_id AS "category_id: _",
                e.planned_base_minor, e.carryover_base_minor, e.rollover_rule AS "rollover_rule: String",
                e.created_at, e.updated_at
            FROM budget_envelopes e
            JOIN budgets b ON b.id = e.budget_id
            WHERE e.id = ? AND b.user_id = ?
            "#,
            envelope_id, user_id
        ).fetch_optional(&self.pool).await?;
        Ok(env)
    }

    async fn create_envelope(&self, envelope: BudgetEnvelope, user_id: Option<Uuid>) -> Result<BudgetEnvelope, Error> {
        // Security check
        let budget_exists = sqlx::query!("SELECT id FROM budgets WHERE id = ? AND user_id = ?", envelope.budget_id, user_id).fetch_optional(&self.pool).await?;
        if budget_exists.is_none() { return Err(Error::msg("Unauthorized or budget not found")); }

        let new_id = Uuid::new_v4();
        let rule_str = envelope.rollover_rule.as_str();

        sqlx::query!(
            "INSERT INTO budget_envelopes (id, budget_id, category_id, planned_base_minor, carryover_base_minor, rollover_rule) VALUES (?, ?, ?, ?, ?, ?)",
            new_id, envelope.budget_id, envelope.category_id, envelope.planned_base_minor, envelope.carryover_base_minor, rule_str
        ).execute(&self.pool).await?;

        self.get_envelope(new_id, user_id).await?.ok_or_else(|| Error::msg("Envelope not found"))
    }

    async fn update_envelope(&self, envelope_id: Uuid, planned: i64, carryover: i64, rule: RolloverRule, user_id: Option<Uuid>) -> Result<Option<BudgetEnvelope>, Error> {
        let rule_str = rule.as_str();
        sqlx::query!(
            r#"
            UPDATE budget_envelopes e JOIN budgets b ON b.id = e.budget_id
            SET e.planned_base_minor = ?, e.carryover_base_minor = ?, e.rollover_rule = ?
            WHERE e.id = ? AND b.user_id = ?
            "#,
            planned, carryover, rule_str, envelope_id, user_id
        ).execute(&self.pool).await?;

        self.get_envelope(envelope_id, user_id).await
    }

    async fn delete_envelope(&self, envelope_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE e FROM budget_envelopes e JOIN budgets b ON b.id = e.budget_id WHERE e.id = ? AND b.user_id = ?", envelope_id, user_id)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_envelopes_by_budget(&self, budget_id: Uuid, limit: Option<u32>, offset: Option<u32>, user_id: Option<Uuid>) -> Result<Vec<BudgetEnvelope>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let envs = sqlx::query_as!(
            BudgetEnvelope,
            r#"
            SELECT
                e.id AS "id: _", e.budget_id AS "budget_id: _", e.category_id AS "category_id: _",
                e.planned_base_minor, e.carryover_base_minor, e.rollover_rule AS "rollover_rule: String",
                e.created_at, e.updated_at
            FROM budget_envelopes e
            JOIN budgets b ON b.id = e.budget_id
            WHERE e.budget_id = ? AND b.user_id = ?
            LIMIT ? OFFSET ?
            "#,
            budget_id, user_id, limit_val, offset_val
        ).fetch_all(&self.pool).await?;
        Ok(envs)
    }
}
