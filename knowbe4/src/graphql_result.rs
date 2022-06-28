use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GrapqhqlResult<T> {
    pub query: String,
    pub variables: T,
}

impl<T> GrapqhqlResult<T> {
    pub fn new(query: String, variables: T) -> Self
    where
        T: Serialize + DeserializeOwned,
    {
        Self { query, variables }
    }
}
