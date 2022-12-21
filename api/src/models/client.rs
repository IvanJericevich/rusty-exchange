use database::ClientModel;

use utoipa::{IntoParams, ToSchema};

use serde::Deserialize;

// ----------------------------------------------------------------------

pub struct Client(ClientModel);

impl ToSchema for Client {
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
                "email",
                utoipa::openapi::Object::with_type(utoipa::openapi::SchemaType::String),
            )
            .required("email")
            .property(
                "created_at",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::DateTime,
                    ))),
            )
            .required("created_at")
            .example(Some(serde_json::json!({
                "id": 1,
                "email": "example@gmail.com",
                "created_at": "2022-01-01T00:00:00"
            })))
            .into()
    }
}

#[derive(Deserialize, IntoParams)]
pub struct GetRequest {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct PutRequest {
    pub new_email: String,
}
