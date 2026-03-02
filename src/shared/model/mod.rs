use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CountRow {
    
    pub n: i64
    
}
