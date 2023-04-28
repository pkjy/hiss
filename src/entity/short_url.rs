// src/entity/short_url.rs
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "short_url")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: String,
    pub company_id: Option<String>,
    pub short_domain: String,
    pub short_code: String,
    pub original_url: String,
    pub expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {
    ShortUrl,
}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        match self {
            Self::ShortUrl => Entity::has_many(super::short_url::Entity).into(),
        }
        // panic!("没有定义关系")
    }
}

impl Related<super::short_url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShortUrl.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
