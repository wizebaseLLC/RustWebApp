use async_graphql::SimpleObject;
use query_tiberius_derive::Queryable;

#[allow(non_snake_case)]
#[derive(Debug, SimpleObject, Clone, Queryable, serde::Serialize, serde::Deserialize)]
pub struct QtrakUser {
    Fullname: Option<String>,
    Firstname: Option<String>,
    Lastname: Option<String>,
    Email: Option<String>,
    Phone: Option<String>,
    Phone2: Option<String>,
    Location01: Option<String>,
    Location02: Option<String>,
    Location03: Option<String>,
    ContactId: Option<String>,
}
