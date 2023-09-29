//! Reqwest wrappers

use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    error::{VelocityAPIError, VelocityError},
    Velocity,
};

/// A JSON response, containing a status code and a response structure
#[allow(dead_code)]
pub struct JSONResponse<T: DeserializeOwned> {
    /// The status the method exited with
    pub status: StatusCode,
    /// The response structure provided by the API
    pub response: T,
}

impl Velocity {
    /// Generates a full URL from the internal base url and the endpoint
    fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }

    /// Executes a request to the remote API and expects a JSON response
    /// # Arguments
    /// * `method` - The method to use for the request
    /// * `endpoint` - The endpoint to route the request to: e.g. `/u/auth`
    /// * `request` - The request structure to provide to the API
    /// # Returns
    /// A `JSONResponse` struct containing the status code and the expected response structure
    pub async fn request_json<REQUEST: Serialize, RESPONSE: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        request: &REQUEST,
    ) -> Result<JSONResponse<RESPONSE>, VelocityError> {
        let r_response = self
            .http_client
            .request(method, self.url(endpoint))
            .json(&request)
            .send()
            .await?;

        let status = r_response.status();

        match status {
            StatusCode::OK => {
                let response = r_response.json::<RESPONSE>().await?;

                Ok(JSONResponse { status, response })
            }
            StatusCode::BAD_REQUEST => {
                #[derive(Deserialize)]
                struct In {
                    reason: String,
                }

                let status: u32 = r_response.status().as_u16() as u32;
                let err = r_response.json::<In>().await?;

                Err(VelocityError::APIError(VelocityAPIError {
                    code: status,
                    message: err.reason,
                }))
            }
            _ => Err(VelocityError::APIError(
                r_response.json::<VelocityAPIError>().await?,
            )),
        }
    }

    /// Executes a request to the remote API without any body
    /// # Arguments
    /// * `method` - The method to use for the request
    /// * `endpoint` - The endpoint to route the request to: e.g. `/u/auth`
    /// * `request` - The request structure to provide to the API
    pub async fn request<T: Serialize>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        request: &T,
    ) -> Result<StatusCode, VelocityError> {
        let r_response = self
            .http_client
            .request(method, self.url(endpoint))
            .json(&request)
            .send()
            .await?;

        let status = r_response.status();

        match status {
            StatusCode::OK => Ok(status),
            StatusCode::BAD_REQUEST => {
                #[derive(Deserialize)]
                struct In {
                    reason: String,
                }

                let status: u32 = r_response.status().as_u16() as u32;
                let err = r_response.json::<In>().await?;

                Err(VelocityError::APIError(VelocityAPIError {
                    code: status,
                    message: err.reason,
                }))
            }
            _ => Err(VelocityError::APIError(
                r_response.json::<VelocityAPIError>().await?,
            )),
        }
    }
}
