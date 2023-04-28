// src/entity/visit.rs
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "visit")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub ua: String,
    pub ip: String,
    pub short_url_id: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {
}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        panic!("没有定义关系")
    }
}


impl ActiveModelBehavior for ActiveModel {}
