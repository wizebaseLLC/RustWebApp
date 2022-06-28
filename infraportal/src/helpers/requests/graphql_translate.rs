use async_graphql::{FieldError, FieldResult};

pub fn graphql_translate<T>(res: anyhow::Result<T>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => {
            log::error!(target: "error", "{}", e.to_string());
            FieldResult::Err(FieldError::from(e))
        }
    }
}

pub fn graphql_translate_option<T>(res: anyhow::Result<T>) -> FieldResult<Option<T>> {
    match res {
        Ok(x) => Ok(Some(x)),
        Err(e) => {
            log::error!(target: "error", "{}", e.to_string());
            FieldResult::Err(FieldError::from(e))
        }
    }
}
