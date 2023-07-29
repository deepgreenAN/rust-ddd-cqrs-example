use crate::InfraError;
use domain::repositories::Transaction;

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::future::Future;
use std::pin::Pin;

/// データベースのトランザクション
pub struct DBTransaction {
    inner: DatabaseTransaction,
}

#[async_trait::async_trait]
impl Transaction for DBTransaction {
    type Error = InfraError;
    type Inner = DatabaseTransaction;
    type Pool = DatabaseConnection;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
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
