use domain::aggregates::atm::orm::Entity as AtmEntity;
use domain::aggregates::bank_account::orm::Entity as BankAccountEntity;

use sea_orm::EntityName;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::schema::Schema;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Schema::new(manager.get_database_backend())
                    .create_table_from_entity(BankAccountEntity),
            )
            .await?;

        manager
            .create_table(
                Schema::new(manager.get_database_backend()).create_table_from_entity(AtmEntity),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(BankAccountEntity.table_ref())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AtmEntity.table_ref()).to_owned())
            .await?;

        Ok(())
    }
}
