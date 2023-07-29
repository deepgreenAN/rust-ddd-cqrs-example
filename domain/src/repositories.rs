use ddd_cqrs_core::Aggregate;

use crate::aggregates::{Atm, BankAccount};

use std::future::Future;
use std::pin::Pin;

// -------------------------------------------------------------------------------------------------
// Transaction

/// トランザクション用のトレイト．ネストはできない．
#[async_trait::async_trait]
pub trait Transaction: Sized + Send + Sync {
    type Inner;
    type Error;
    type Pool: Send + Sync + Clone;
    // 内部のコネクション・トランザクション等を取得
    fn inner(&self) -> &Self::Inner;
    // トランザクションのコンストラクタ
    async fn begin(pool: &Self::Pool) -> Result<Self, Self::Error>;
    // コミット
    async fn commit(self) -> Result<(), Self::Error>;
    // ロールバック
    async fn rollback(self) -> Result<(), Self::Error>;
    // クロージャーを与え、Okが返った場合はコミット，Errが返った場合はロールバックを行う．
    async fn transaction<F, T>(&self, func: F) -> Result<T, Self::Error>
    where
        F: FnOnce() -> Pin<Box<dyn Future<Output = Result<T, Self::Error>> + Send>> + Send,
        T: Send;
}

// -------------------------------------------------------------------------------------------------
// 各種Repository

/// ベースリポジトリ
#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    type Error: std::error::Error;
    type Aggregate: Aggregate;
    type Transaction: Transaction<Error = <Self as Repository>::Error>;

    /// アグリゲイトを一つ保存(インサート)
    async fn save<'t>(
        &self,
        aggregate: Self::Aggregate,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error>;
    /// アグリゲイトを一つアップデート
    async fn edit<'t>(
        &self,
        aggregate: Self::Aggregate,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error>;
    /// アグリゲイトをidから取得
    async fn find_by_id<'t>(
        &self,
        id: <Self::Aggregate as Aggregate>::IntoId,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<Self::Aggregate, Self::Error>;
    /// 指定したidのアグリゲイトを削除
    async fn remove<'t>(
        &self,
        id: <Self::Aggregate as Aggregate>::IntoId,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error>;
}

/// BankAccountのリポジトリ(追加の処理を記述する)
pub trait BankAccountRepository: Repository<Aggregate = BankAccount> {}

/// Atmのリポジトリ(追加の処理を記述する)
pub trait AtmRepository: Repository<Aggregate = Atm> {}
