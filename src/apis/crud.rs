use std::collections::HashMap;

use super::SanitizedAPI;
use crate::API;
use axum::{extract::State, Json};

/**
 * get_apis is a function that is called when a request is made to the get endpoint. It retrieves all
 * the apis from the database, deserializes the apis, and returns a vector of sanitized apis.
 * Sanitized apis are apis that have been stripped of sensitive information.
 */
pub async fn get_apis(State(state): State<HashMap<String, API>>) -> Json<Vec<SanitizedAPI>> {
    let apis = state
        .values()
        .map(|api| SanitizedAPI {
            identifier: api.identifier.clone(),
            metadata: api.metadata.clone(),
        })
        .collect();
    Json(apis)
}
