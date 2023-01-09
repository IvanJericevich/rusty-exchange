//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "clients")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[schema(example = 1)]
    pub id: i32,
    #[sea_orm(unique)]
    #[schema(example = "example@gmail.com")]
    pub email: String,
    #[schema(example = "1970-01-01T00:00:00")]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sub_accounts::Entity")]
    SubAccounts,
}

impl Related<super::sub_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubAccounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// ----------------------------------------------------------------------

#[derive(Deserialize, IntoParams)]
pub struct GetRequest {
    #[param(example = 0)]
    pub page: Option<u64>,
    #[param(example = 1000)]
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct PutRequest {
    #[param(example = "example@gmail.com")]
    pub new_email: String,
}