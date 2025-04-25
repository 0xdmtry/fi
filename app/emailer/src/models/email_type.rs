use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum EmailType {
    #[sea_orm(string_value = "passcode")]
    Passcode,

    #[sea_orm(string_value = "success_passcode")]
    SuccessPasscode,

    #[sea_orm(string_value = "failed_passcode")]
    FailedPasscode,
}
