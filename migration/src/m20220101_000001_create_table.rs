use domain::aggregates::atm::orm::Entity as AtmEntity;
use domain::aggregates::bank_account::orm::Entity as BankAccountEntity;

use sea_orm::{DbBackend, EntityName};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::schema::Schema;
use sea_orm_migration::sea_query::{TableCreateStatement, TableDropStatement};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// BankAccountのテーブルを作成するSQLを作成
pub fn create_bank_account_table_sql(backend: DbBackend) -> TableCreateStatement {
    Schema::new(backend).create_table_from_entity(BankAccountEntity)
}
/// Atmのテーブルを作成するSQLを作成
pub fn create_atm_table_sql(backend: DbBackend) -> TableCreateStatement {
    Schema::new(backend).create_table_from_entity(AtmEntity)
}

/// BankAccountのテーブルを削除するSQLを作成
pub fn drop_bank_account_table_sql() -> TableDropStatement {
    Table::drop()
        .table(BankAccountEntity.table_ref())
        .to_owned()
}

/// Atmのテーブルを削除するSQを作成
pub fn drop_atm_table_sql() -> TableDropStatement {
    Table::drop().table(AtmEntity.table_ref()).to_owned()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(create_bank_account_table_sql(
                manager.get_database_backend(),
            ))
            .await?;

        manager
            .create_table(create_atm_table_sql(manager.get_database_backend()))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(drop_bank_account_table_sql()).await?;

        manager.drop_table(drop_atm_table_sql()).await?;

        Ok(())
    }
}
