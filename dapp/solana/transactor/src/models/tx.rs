use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tx")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub signature: String,

    pub user_id: Uuid,
    pub wallet_id: Option<Uuid>,

    pub slot: u64,
    pub block_time: Option<i64>,
    pub success: bool,

    pub fee_payer: String,
    pub signer_addresses: Json,
    pub instructions: Option<Json>,
    pub log_messages: Option<Json>,

    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}