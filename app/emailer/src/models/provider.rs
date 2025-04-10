use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type="String", db_type="Text")]
pub enum Provider {
    #[sea_orm(string_value="mailhog")]
    Mailhog
}