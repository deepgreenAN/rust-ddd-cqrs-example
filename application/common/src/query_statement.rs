use sea_orm::query::{QueryTrait, Select};
use sea_orm::EntityTrait;
pub use sea_orm::{DatabaseBackend, Statement, Value};

use serde::{Deserialize, Serialize};

use std::borrow::Cow;

pub const DEFAULT_DB_BACKEND: DatabaseBackend = DatabaseBackend::Postgres;

/// SQL文を表す型
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub struct QueryStatement {
    statement: Statement,
}

#[cfg(feature = "server")]
impl QueryStatement {
    /// statementを取得
    pub fn statement(self) -> Statement {
        self.statement
    }
}

#[cfg(feature = "frontend")]
impl QueryStatement {
    /// SQL文から直接作成
    pub fn from_string<'s, S: Into<Cow<'s, str>>>(db_backend: DatabaseBackend, stmt: S) -> Self {
        let stmt: Cow<'s, str> = stmt.into();

        Self {
            statement: Statement::from_string(db_backend, stmt.into_owned()),
        }
    }
    /// SQL文＋Valueから作成
    pub fn from_sql_and_values<VI: IntoIterator<Item = Value>>(
        db_backend: DatabaseBackend,
        sql: &str,
        values: VI,
    ) -> Self {
        Self {
            statement: Statement::from_sql_and_values(db_backend, sql, values),
        }
    }

    /// sea_orm::Selectから作成
    pub fn from_select<E: EntityTrait>(db_backend: DatabaseBackend, select: Select<E>) -> Self {
        Self {
            statement: select.build(db_backend),
        }
    }
}

impl From<QueryStatement> for String {
    fn from(value: QueryStatement) -> Self {
        value.statement.to_string()
    }
}

impl From<String> for QueryStatement {
    fn from(value: String) -> Self {
        QueryStatement {
            statement: Statement::from_string(DEFAULT_DB_BACKEND, value),
        }
    }
}
