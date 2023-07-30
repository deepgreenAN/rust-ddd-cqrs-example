use crate::command_handlers::bank_account_command_handlers::BankAccountCommandHandler;
use crate::query_handlers::QueryHandler;
use common::commands::bank_account_commands::BankAccountCommand;
use common::query_statement::QueryStatement;
use common::ApplicationError;

use axum::{
    extract::rejection::JsonRejection,
    extract::{Json, State},
};
use sea_orm::FromQueryResult;
use std::sync::Arc;

/// BankAccountに関するコマンドに対するaxumハンドラ
pub async fn bank_account_command_api_handler(
    State(bank_account_command_handler): State<Arc<BankAccountCommandHandler>>,
    command_res: Result<Json<BankAccountCommand>, JsonRejection>,
) -> Result<(), ApplicationError> {
    let command = command_res?.0;

    bank_account_command_handler.handle_command(command).await
}

/// ジェネリックなクエリ(one)に対するaxumハンドラ
pub async fn query_one_handler<T: FromQueryResult>(
    State(query_handler): State<Arc<QueryHandler<T>>>,
    query_res: Result<Json<QueryStatement>, JsonRejection>,
) -> Result<Option<T>, ApplicationError> {
    let query = query_res?.0;

    query_handler.handle_query_one(query).await
}

/// ジェネリックなクエリ(all)に対するハンドラ
pub async fn query_all_handler<T: FromQueryResult>(
    State(query_handler): State<Arc<QueryHandler<T>>>,
    query_res: Result<Json<QueryStatement>, JsonRejection>,
) -> Result<Vec<T>, ApplicationError> {
    let query = query_res?.0;

    query_handler.handle_query_all(query).await
}
