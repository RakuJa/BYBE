use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::{Object, ObjectBuilder, Type};
pub fn ordered_float_to_schema() -> Object {
    ObjectBuilder::new()
        .schema_type(SchemaType::Type(Type::Number))
        .build()
}
