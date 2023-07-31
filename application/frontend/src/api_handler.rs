/// モックテスト用にurlを引数とする関数を定義するモジュール
pub(crate) mod inner;

use crate::API_BASE_URL;

use common::commands::atm_commands::AtmRefCommand;
use common::commands::bank_account_commands::BankAccountRefCommand;
use common::{query_statement::QueryStatement, ApplicationError};
use domain::aggregates::{Atm, BankAccount};

use serde::de::DeserializeOwned;

/// BankAccountCommandを実行
pub async fn execute_bank_account_command<'a>(
    command: BankAccountRefCommand<'a>,
) -> Result<(), ApplicationError> {
    inner::execute_bank_account_command(API_BASE_URL, command).await
}

/// AtmCommandを実行
pub async fn execute_atm_command<'a>(command: AtmRefCommand<'a>) -> Result<(), ApplicationError> {
    inner::execute_atm_command(API_BASE_URL, command).await
}

/// BankAccountに関するクエリを実行して結果を一つ取得する．
pub async fn query_one_bank_account(
    query_stmt: QueryStatement,
) -> Result<Option<BankAccount>, ApplicationError> {
    inner::query_one_bank_account(API_BASE_URL, query_stmt).await
}

/// BankAccountに関するクエリを実行して結果を複数取得する．
pub async fn query_all_bank_account(
    query_stmt: QueryStatement,
) -> Result<Vec<BankAccount>, ApplicationError> {
    inner::query_all_bank_account(API_BASE_URL, query_stmt).await
}

/// Atmに関するクエリを実行して結果を一つ取得する．
pub async fn query_one_atm(query_stmt: QueryStatement) -> Result<Option<Atm>, ApplicationError> {
    inner::query_one_atm(API_BASE_URL, query_stmt).await
}

/// Atmに関するクエリを実行して結果を複数取得する．
pub async fn query_all_atm(query_stmt: QueryStatement) -> Result<Vec<Atm>, ApplicationError> {
    inner::query_all_atm(API_BASE_URL, query_stmt).await
}

/// カスタムクエリを実行して結果を一つ取得する．
pub async fn query_one_custom<T: DeserializeOwned>(
    query_stmt: QueryStatement,
) -> Result<Option<T>, ApplicationError> {
    inner::query_one_custom(API_BASE_URL, query_stmt).await
}

/// カスタムクエリを実行して結果を複数取得する．
pub async fn query_all_custom<T: DeserializeOwned>(
    query_stmt: QueryStatement,
) -> Result<Vec<T>, ApplicationError> {
    inner::query_all_custom(API_BASE_URL, query_stmt).await
}
