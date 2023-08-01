/// モックテスト用にurlを引数とする関数を定義するモジュール
use crate::utils::{deserialize_response, deserialize_response_unit};
use crate::{AtmCommand, BankAccountCommand};

use common::{query_statement::QueryStatement, ApplicationError};
use domain::aggregates::{Atm, BankAccount};

use reqwest::Client;
use serde::de::DeserializeOwned;

pub async fn execute_bank_account_command<'a>(
    base_url: &str,
    command: BankAccountCommand<'a>,
) -> Result<(), ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/command/bank_account"))
        .json(&command);

    let response = request.send().await?;

    deserialize_response_unit(response).await
}

pub async fn execute_atm_command<'a>(
    base_url: &str,
    command: AtmCommand<'a>,
) -> Result<(), ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/command/atm"))
        .json(&command);

    let response = request.send().await?;

    deserialize_response_unit(response).await
}

pub async fn query_one_bank_account(
    base_url: &str,
    query_stmt: QueryStatement,
) -> Result<Option<BankAccount>, ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/query_one/bank_account"))
        .json(&query_stmt);

    let response = request.send().await?;

    deserialize_response(response).await
}

pub async fn query_all_bank_account(
    base_url: &str,
    query_stmt: QueryStatement,
) -> Result<Vec<BankAccount>, ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/query_all/bank_account"))
        .json(&query_stmt);

    let response = request.send().await?;

    deserialize_response(response).await
}

pub async fn query_one_atm(
    base_url: &str,
    query_stmt: QueryStatement,
) -> Result<Option<Atm>, ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/query_one/atm"))
        .json(&query_stmt);

    let response = request.send().await?;

    deserialize_response(response).await
}

pub async fn query_all_atm(
    base_url: &str,
    query_stmt: QueryStatement,
) -> Result<Vec<Atm>, ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/query_all/atm"))
        .json(&query_stmt);

    let response = request.send().await?;

    deserialize_response(response).await
}

pub async fn query_one_custom<T: DeserializeOwned>(
    base_url: &str,
    query_stmt: QueryStatement,
) -> Result<Option<T>, ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/query_one/custom"))
        .json(&query_stmt);

    let response = request.send().await?;

    deserialize_response(response).await
}

pub async fn query_all_custom<T: DeserializeOwned>(
    base_url: &str,
    query_stmt: QueryStatement,
) -> Result<Vec<T>, ApplicationError> {
    let request = Client::new()
        .post(&format!("{base_url}/query_all/custom"))
        .json(&query_stmt);

    let response = request.send().await?;

    deserialize_response(response).await
}
