use crate::utils::normalize::normalize_str;
use sea_orm::entity::prelude::*;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Provider {
    #[sea_orm(string_value = "mailhog")]
    #[default]
    Mailhog,
}

impl FromStr for Provider {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = normalize_str(s);

        match p.as_str() {
            "mailhog" => Ok(Provider::Mailhog),
            _ => Err(()),
        }
    }
}
