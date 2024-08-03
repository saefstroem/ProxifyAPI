use serde::{Deserialize, Serialize};

pub mod crud;

#[derive(Debug, Deserialize, Serialize)]
pub struct SanitizedAPI {
    pub identifier: String,
    pub metadata: String,
}
