use common::query_statement::QueryStatement;
use common::ApplicationError;
use infrastructure::InfraError;

use sea_orm::{DatabaseConnection, FromQueryResult};
use std::marker::PhantomData;

/// クエリ結果を取得するジェネリックなクエリハンドラ
pub struct QueryHandler<T: FromQueryResult> {
    data_type: PhantomData<T>,
    conn: DatabaseConnection,
}

impl<T: FromQueryResult> QueryHandler<T> {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {
            data_type: PhantomData,
            conn,
        }
    }
    /// クエリの結果を一つ取得．
    pub async fn handle_query_one(
        &self,
        query_stmt: QueryStatement,
    ) -> Result<Option<T>, ApplicationError> {
        let res_opt = T::find_by_statement(query_stmt.statement())
            .one(&self.conn)
            .await
            .map_err(Into::<InfraError>::into)?;

        Ok(res_opt)
    }
    /// クエリの結果を複数取得
    pub async fn handle_query_all(
        &self,
        query_stmt: QueryStatement,
    ) -> Result<Vec<T>, ApplicationError> {
        let res = T::find_by_statement(query_stmt.statement())
            .all(&self.conn)
            .await
            .map_err(Into::<InfraError>::into)?;

        Ok(res)
    }
}
