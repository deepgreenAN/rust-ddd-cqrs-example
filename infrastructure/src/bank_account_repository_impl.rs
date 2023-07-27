use crate::InfraError;
use domain::aggregates::bank_account::orm as bank_account_mod;
use domain::aggregates::bank_account::{BankAccount, BankAccountId};
use domain::repositories::BankAccountRepository;

use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, EntityTrait, IntoActiveModel};

/// データベースを用いたBankAccountRepository
pub struct DbBankAccountRepository<T: ConnectionTrait + Send> {
    conn: T,
}

impl<T: ConnectionTrait + Send> DbBankAccountRepository<T> {
    pub fn new(conn: T) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl<T: ConnectionTrait + Send> BankAccountRepository for DbBankAccountRepository<T> {
    type Error = InfraError;
    async fn save(&self, bank_account: BankAccount) -> Result<(), Self::Error> {
        let active_model = Into::<bank_account_mod::Model>::into(bank_account).into_active_model();
        active_model.insert(&self.conn).await?;

        Ok(())
    }
    async fn edit(&self, bank_account: BankAccount) -> Result<(), Self::Error> {
        let active_model = Into::<bank_account_mod::Model>::into(bank_account)
            .into_active_model() // 全ての値を更新
            .reset_all();
        active_model.update(&self.conn).await?;

        Ok(())
    }
    async fn open(&self, id: BankAccountId) -> Result<(), Self::Error> {
        let active_model = bank_account_mod::ActiveModel {
            id: ActiveValue::Unchanged(id.into()),
            opened: ActiveValue::Set(true),
            ..Default::default()
        };
        active_model.update(&self.conn).await?;

        Ok(())
    }
    async fn remove(&self, id: BankAccountId) -> Result<(), Self::Error> {
        bank_account_mod::Entity::delete_by_id(id)
            .exec(&self.conn)
            .await?;

        Ok(())
    }
}
