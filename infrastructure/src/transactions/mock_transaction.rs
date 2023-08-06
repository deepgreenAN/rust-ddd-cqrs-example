use crate::InfraError;
use domain::repositories::Transaction;

use std::future::Future;
use std::pin::Pin;

/// モック用のトランザクション
pub struct MockTransaction;

/// モック用のプール
#[derive(Clone)]
pub struct MockPool;

#[async_trait::async_trait]
impl Transaction for MockTransaction {
    type Error = InfraError;
    type Pool = MockPool;

    async fn begin(_: &Self::Pool) -> Result<Self, Self::Error> {
        Ok(MockTransaction)
    }
    async fn commit(self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn rollback(self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn transaction<F, T>(&self, func: F) -> Result<T, Self::Error>
    where
        F: FnOnce() -> Pin<Box<dyn Future<Output = Result<T, Self::Error>> + Send>> + Send,
        T: Send,
    {
        func().await
    }
}
