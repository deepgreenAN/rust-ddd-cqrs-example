use crate::{transactions::DbTransaction, InfraError};
use domain::aggregates::bank_account::{self, BankAccount, BankAccountId};
use domain::repositories::{BankAccountRepository, Repository};

use derive_new::new;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};

/// データベースを用いたBankAccountRepository
#[derive(Clone, Debug, new)]
pub struct DbBankAccountRepository {
    conn: DatabaseConnection,
}

#[async_trait::async_trait]
impl Repository for DbBankAccountRepository {
    type Error = InfraError;
    type Aggregate = BankAccount;
    type Transaction = DbTransaction;

    async fn save<'t>(
        &self,
        bank_account: BankAccount,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let active_model = Into::<bank_account::orm::Model>::into(bank_account).into_active_model();

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
        let active_model = Into::<bank_account::orm::Model>::into(bank_account)
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
            let select = bank_account::orm::Entity::find_by_id(id);

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
        let delete = bank_account::orm::Entity::delete_by_id(id);

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

// -------------------------------------------------------------------------------------------------
// test

#[cfg(test)]
mod test {
    use super::{DbBankAccountRepository, DbTransaction};
    use crate::{test_utils::assert_aggregates_eq, InfraError};
    use domain::repositories::Repository;
    use domain::{
        aggregates::{bank_account, BankAccount},
        repositories::Transaction,
    };

    use rand::seq::SliceRandom;
    use rand::Rng;
    use rstest::{fixture, rstest};
    use sea_orm::{Database, EntityTrait};

    #[fixture]
    async fn save_bank_accounts(
    ) -> Result<(DbBankAccountRepository, DbTransaction, Vec<BankAccount>), InfraError> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL envvar is not set.");
        let db_connection = Database::connect(db_url).await?;

        let transaction = DbTransaction::begin(&db_connection).await?;

        let repo = DbBankAccountRepository::new(db_connection);

        let bank_accounts = fake::vec![BankAccount; 50];

        for bank_account in bank_accounts.iter().cloned() {
            repo.save(bank_account, Some(&transaction)).await?;
        }

        Ok((repo, transaction, bank_accounts))
    }

    async fn query_all_bank_account(
        transaction: &DbTransaction,
    ) -> Result<Vec<BankAccount>, InfraError> {
        let models = bank_account::orm::Entity::find()
            .all(transaction.inner())
            .await?;

        Ok(models
            .into_iter()
            .map(Into::<BankAccount>::into)
            .collect::<Vec<_>>())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_edit_bank_accounts(
        #[future] save_bank_accounts: Result<
            (DbBankAccountRepository, DbTransaction, Vec<BankAccount>),
            InfraError,
        >,
    ) {
        let (repo, transaction, mut bank_accounts) = save_bank_accounts.await.unwrap();

        let mut rng = rand::thread_rng();

        // シャッフル
        bank_accounts.shuffle(&mut rng);

        for bank_account in bank_accounts.iter_mut().take(20) {
            bank_account
                .deposit_money(rng.gen_range(0.0..100_000.0))
                .unwrap();
            repo.edit(bank_account.clone(), Some(&transaction))
                .await
                .unwrap();
        }

        // データを取得して比較
        let mut actual_bank_accounts = query_all_bank_account(&transaction).await.unwrap();
        assert_aggregates_eq(&mut actual_bank_accounts, &mut bank_accounts);
    }
}
