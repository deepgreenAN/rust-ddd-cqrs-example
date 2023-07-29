use crate::{DBTransaction, InfraError};
use domain::aggregates::bank_account::orm as bank_account_mod;
use domain::aggregates::bank_account::{BankAccount, BankAccountId};
use domain::repositories::{BankAccountRepository, Repository, Transaction};

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};

/// データベースを用いたBankAccountRepository
#[derive(Clone, Debug)]
pub struct DbBankAccountRepository {
    conn: DatabaseConnection,
}

impl DbBankAccountRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl Repository for DbBankAccountRepository {
    type Error = InfraError;
    type Aggregate = BankAccount;
    type Transaction = DBTransaction;

    async fn save<'t>(
        &self,
        bank_account: BankAccount,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let active_model = Into::<bank_account_mod::Model>::into(bank_account).into_active_model();

        match transaction {
            Some(transaction) => {
                active_model.insert(transaction.inner()).await?;
            }
            None => {
                active_model.insert(&self.conn).await?;
            }
        }

        Ok(())
    }
    async fn edit<'t>(
        &self,
        bank_account: BankAccount,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let active_model = Into::<bank_account_mod::Model>::into(bank_account)
            .into_active_model() // 全ての値を更新
            .reset_all();

        match transaction {
            Some(transaction) => {
                active_model.update(transaction.inner()).await?;
            }
            None => {
                active_model.update(&self.conn).await?;
            }
        }

        Ok(())
    }
    async fn find_by_id<'t>(
        &self,
        id: BankAccountId,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<Self::Aggregate, Self::Error> {
        let found_bank_account = {
            let select = bank_account_mod::Entity::find_by_id(id);

            match transaction {
                Some(transaction) => select.one(transaction.inner()).await?,
                None => select.one(&self.conn).await?,
            }
        };

        match found_bank_account {
            Some(res) => Ok(res.into()),
            None => Err(InfraError::RecordNotFoundError(format!(
                "Not found id: {}",
                Into::<String>::into(id)
            ))),
        }
    }
    async fn remove<'t>(
        &self,
        id: BankAccountId,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let delete = bank_account_mod::Entity::delete_by_id(id);

        match transaction {
            Some(transaction) => {
                delete.exec(transaction.inner()).await?;
            }
            None => {
                delete.exec(&self.conn).await?;
            }
        }

        Ok(())
    }
}

impl BankAccountRepository for DbBankAccountRepository {}
