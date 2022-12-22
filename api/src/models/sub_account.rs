use database::{SubAccountModel, SubAccountStatus};

use utoipa::{IntoParams, ToSchema};

use serde::Deserialize;

// ----------------------------------------------------------------------

pub struct SubAccount(SubAccountModel);

impl ToSchema for SubAccount {
    fn schema() -> utoipa::openapi::schema::Schema {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("id")
            .property(
                "name",
                utoipa::openapi::Object::with_type(utoipa::openapi::SchemaType::String),
            )
            .required("name")
            .property(
                "created_at",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::DateTime,
                    ))),
            )
            .required("created_at")
            .property(
                "client_id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("client_id")
            .example(Some(serde_json::json!({
                "id": 1,
                "name": "Test",
                "created_at": "2022-01-01T00:00:00",
                "client_id": 1
            })))
            .into()
    }
}

#[derive(Deserialize, IntoParams)]
pub struct GetRequest {
    pub status: Option<SubAccountStatus>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct PostRequest {
    pub status: Option<SubAccountStatus>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct PutRequest {
    pub status: Option<SubAccountStatus>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
