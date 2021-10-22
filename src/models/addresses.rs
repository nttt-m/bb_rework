//! SeaORM Entity. Generated by sea-orm-codegen 0.3.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "addresses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub address_id: i32,
    pub house_name_number: String,
    pub street: Option<String>,
    pub town_city: Option<String>,
    pub region: Option<String>,
    pub postal_code: String,
    pub country: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::employees::Entity")]
    Employees,
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employees.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
