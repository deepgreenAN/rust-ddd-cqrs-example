use crate::InfraError;
use domain::repositories::Transaction;

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::future::Future;
use std::pin::Pin;

/// データベースのトランザクション
pub struct DbTransaction {
    inner: DatabaseTransaction,
}

impl DbTransaction {
    pub fn inner(&self) -> &DatabaseTransaction {
        &self.inner
    }
}

#[async_trait::async_trait]
impl Transaction for DbTransaction {
    type Error = InfraError;
    type Pool = DatabaseConnection;

    async fn begin(pool: &Self::Pool) -> Result<Self, Self::Error> {
        let inner = pool.begin().await?;
        Ok(Self { inner })
    }
    async fn commit(self) -> Result<(), Self::Error> {
        self.inner.commit().await?;
        Ok(())
    }
    async fn rollback(self) -> Result<(), Self::Error> {
        self.inner.rollback().await?;
        Ok(())
    }
    async fn transaction<F, T>(&self, func: F) -> Result<T, Self::Error>
    where
        F: FnOnce() -> Pin<Box<dyn Future<Output = Result<T, Self::Error>> + Send>> + Send,
        T: Send,
    {
        let transaction = self.inner.begin().await?;
        match func().await {
            Ok(res) => {
                transaction.commit().await?;
                Ok(res)
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }
}
