use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "guest")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "first_name", column_type = "Text")]
    pub first_name: String,

    #[sea_orm(column_name = "first_name", column_type = "Text")]
    pub last_name: String,

    #[sea_orm(column_name = "first_name", column_type = "Text", unique)]
    pub phone: String,

    #[sea_orm(column_name = "first_name", column_type = "Text", nullable, optional)]
    pub email: String,

    #[sea_orm(column_name = "address", column_type = "Text", nullable, optional)]
    pub address: String,

    #[sea_orm(column_name = "created_at", column_type = "TimestampWithTimeZone", optional)]
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
