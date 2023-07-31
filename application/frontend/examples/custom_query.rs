use frontend::ApplicationError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct QueryResult {
    account_name: String,
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    use frontend::aggregates::bank_account;
    use frontend::query_statement::{DatabaseBackend, QueryStatement};
    use sea_orm::{EntityTrait, QuerySelect};

    let query = QueryStatement::from_string(
        DatabaseBackend::Postgres,
        r#"
SELECT "account_name" FROM "bank_account"
    "#,
    );

    let query_res: Vec<QueryResult> = frontend::query_all_custom::<QueryResult>(query).await?;

    println!("query_res: {query_res:?}");

    let query_res_2 = frontend::query_all_custom::<QueryResult>(QueryStatement::from_select(
        DatabaseBackend::Postgres,
        bank_account::orm::Entity::find()
            .select_only()
            .columns([bank_account::orm::Column::AccountName]),
    ))
    .await?;

    println!("query_res_2: {query_res_2:?}");

    Ok(())
}
