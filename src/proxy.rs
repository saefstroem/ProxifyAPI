use std::collections::HashMap;

use axum::{
    extract::State,
    http::{HeaderMap, HeaderName, HeaderValue},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::API;

#[derive(Deserialize, Serialize, Debug)]
pub enum Method {
    Get,
    Post(String),
    Put,
    Delete,
}

/**
 * KeyTransport is an enum that represents the different ways that an API key can be sent to the
 * external API. The Header variant is used when the API key is sent as a header, and the Replace
 * variant is used when the API key is sent as a part of the URI.
 */
#[derive(Deserialize, Serialize, Debug)]
pub enum KeyTransport {
    Header(String),
    Replace(String),
}

/**
 * ProxyRequest is a struct that represents the request that is sent to the proxy endpoint. It
 * contains the identifier of the exchange, the URI of the external API, the method of the request,
 * and the transport method of the API key.
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct ProxyRequest {
    identifier: String,
    uri: String,
    method: Method,
    transport: Option<KeyTransport>,
}

/**
 * ProxyResponse is a struct that represents the response that is sent back from the proxy endpoint.
 * It may contain the body of the response and the status code of the response.
 */
#[derive(Serialize, Debug)]
pub struct ProxyResponse {
    pub body: Option<String>,
    pub status: u16,
}

/**
 * proxy_request is a function that is called when a request is made to the proxy endpoint. It
 * retrieves the API information from the database, constructs the request to the external API, and
 * sends the request. It then returns the response from the external API.
 */
pub async fn proxy_request(
    State(state): State<HashMap<String, API>>,
    Json(request): Json<ProxyRequest>,
) -> Json<ProxyResponse> {
    match state.get(&request.identifier) {
        Some(api) => {
            let mut uri = request.uri;
            let mut headers = HeaderMap::new();

            if let Some(transport) = request.transport {
                if let Some(api) = &api.api_key {
                    match transport {
                        KeyTransport::Header(header_identifier) => {
                            headers.insert(
                                HeaderName::from_bytes(header_identifier.as_bytes()).unwrap(),
                                HeaderValue::from_str(api).unwrap(),
                            );
                        }
                        KeyTransport::Replace(replacement) => {
                            uri = uri.replace(&replacement, api);
                        }
                    }
                }
            }

            let client = reqwest::Client::new();
            let response = match request.method {
                Method::Get => client.get(&uri).headers(headers).send().await,
                Method::Post(body) => client.post(&uri).headers(headers).body(body).send().await,
                Method::Put => client.put(&uri).headers(headers).send().await,
                Method::Delete => client.delete(&uri).headers(headers).send().await,
            };

            match response {
                Ok(response) => {
                    let status = response.status();

                    // Check if the response has a body
                    if let Ok(body) = response.text().await {
                        Json(ProxyResponse {
                            body: Some(body),
                            status: status.as_u16(),
                        })
                    } else {
                        Json(ProxyResponse {
                            body: None,
                            status: status.as_u16(),
                        })
                    }
                }
                Err(e) => {
                    log::error!("{}", e);
                    Json(ProxyResponse {
                        body: None,
                        status: 500,
                    })
                }
            }
        }
        None => {
            Json(ProxyResponse {
                body: None,
                status: 404,
            })
        }
    }
}
