fn main() {
    use migration::m20220101_000001_create_table::{
        create_atm_table_sql, create_bank_account_table_sql, drop_atm_table_sql,
        drop_bank_account_table_sql,
    };

    use sea_orm_migration::prelude::PostgresQueryBuilder;
    use sea_orm_migration::sea_orm::DatabaseBackend;
    let backend = DatabaseBackend::Postgres;

    println!(
        "create bank account: \n{}",
        create_bank_account_table_sql(backend).to_string(PostgresQueryBuilder)
    );
    println!("");
    println!(
        "create atm: \n{}",
        create_atm_table_sql(backend).to_string(PostgresQueryBuilder)
    );
    println!("");
    println!(
        "drop bank account: \n{}",
        drop_bank_account_table_sql().to_string(PostgresQueryBuilder)
    );
    println!("");
    println!(
        "drop atm: \n{}",
        drop_atm_table_sql().to_string(PostgresQueryBuilder)
    );
}
