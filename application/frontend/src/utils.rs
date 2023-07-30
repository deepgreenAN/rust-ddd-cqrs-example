use common::ApplicationError;

use reqwest::Response;
use serde::de::DeserializeOwned;

/// レスポンスのResultを特定の型とエラーにデシリアライズ
pub async fn deserialize_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, ApplicationError> {
    match response.status().is_success() {
        true => Ok(response.json::<T>().await?),
        false => Err(response.json::<ApplicationError>().await?),
    }
}

/// レスポンスのResultを()型とエラーにデシリアライズ
pub async fn deserialize_response_unit(response: Response) -> Result<(), ApplicationError> {
    match response.status().is_success() {
        true => Ok(()),
        false => Err(response.json::<ApplicationError>().await?),
    }
}
