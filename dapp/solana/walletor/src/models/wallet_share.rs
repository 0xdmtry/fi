use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wallet_shares")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub wallet_id: Uuid,
    pub encrypted_user_share: Vec<u8>,
    pub server_share: Vec<u8>,
    pub encryption_secret_id: Uuid,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
