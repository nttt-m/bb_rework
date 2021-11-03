//! SeaORM Entity. Generated by sea-orm-codegen 0.3.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "authentication")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub auth_id: i32,
    pub username: String,
    pub salt: String,
    #[sea_orm(column_type = "Custom(\"VARBINARY(128)\".to_owned())", nullable)]
    pub verifier: Option<Vec<u8>>,
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
