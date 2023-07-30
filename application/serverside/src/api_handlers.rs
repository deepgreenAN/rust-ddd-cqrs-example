use crate::command_handlers::ApiHandleCommand;
use crate::query_handlers::QueryHandler;
use common::query_statement::QueryStatement;
use common::ApplicationError;

use axum::{
    extract::rejection::JsonRejection,
    extract::{Json, State},
};
use sea_orm::FromQueryResult;
use serde::Serialize;
use std::sync::Arc;

// -------------------------------------------------------------------------------------------------
// アグリゲイトに対するコマンドのaxumハンドラ

/// ジェネリックなコマンドに対するaxumハンドラ
pub async fn command_api_handler<C: ApiHandleCommand>(
    State(command_handler): State<Arc<C>>,
    command_res: Result<Json<C::Command>, JsonRejection>,
) -> Result<(), ApplicationError> {
    let command = command_res?.0;

    command_handler.handle_command(command).await
}

// -------------------------------------------------------------------------------------------------
// ジェネリックなクエリのaxumハンドラ

/// ジェネリックなクエリ(one)に対するaxumハンドラ
pub async fn query_one_api_handler<T: FromQueryResult + Serialize>(
    State(query_handler): State<Arc<QueryHandler<T>>>,
    query_res: Result<Json<QueryStatement>, JsonRejection>,
) -> Result<Json<Option<T>>, ApplicationError> {
    let query = query_res?.0;

    let res = query_handler.handle_query_one(query).await?;
    Ok(Json(res))
}

/// ジェネリックなクエリ(all)に対するハンドラ
pub async fn query_all_api_handler<T: FromQueryResult + Serialize>(
    State(query_handler): State<Arc<QueryHandler<T>>>,
    query_res: Result<Json<QueryStatement>, JsonRejection>,
) -> Result<Json<Vec<T>>, ApplicationError> {
    let query = query_res?.0;

    let res = query_handler.handle_query_all(query).await?;
    Ok(Json(res))
}
