use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "encryption_secrets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub user_id: Uuid,
    pub encrypted_secret: Vec<u8>,
    pub salt: Vec<u8>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
