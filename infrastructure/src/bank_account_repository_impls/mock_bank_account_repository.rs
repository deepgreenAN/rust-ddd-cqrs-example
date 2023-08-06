use crate::transactions::MockTransaction;
use crate::InfraError;
use async_trait::async_trait;
use ddd_cqrs_core::Aggregate;
use domain::aggregates::BankAccount;
use domain::repositories::{BankAccountRepository, Repository};

use mockall::mock;

mock! {
    /// DbAtmRepositoryのモック
    #[derive(Clone, Debug)]
    pub BankAccountRepository{}

    #[async_trait]
    impl Repository for BankAccountRepository {
        type Error = InfraError;
        type Aggregate = BankAccount;
        type Transaction = MockTransaction;

        async fn save<'t>(
            &self,
            aggregate: <Self as Repository>::Aggregate,
            transaction: Option<&'t <Self as Repository>::Transaction>,
        ) -> Result<(), <Self as Repository>::Error>;

        async fn edit<'t>(
            &self,
            aggregate: <Self as Repository>::Aggregate,
            transaction: Option<&'t <Self as Repository>::Transaction>,
        ) -> Result<(), <Self as Repository>::Error>;

        async fn find_by_id<'t>(
            &self,
            id: <<Self as Repository>::Aggregate as Aggregate>::IntoId,
            transaction: Option<&'t <Self as Repository>::Transaction>,
        ) -> Result<<Self as Repository>::Aggregate, <Self as Repository>::Error>;

        async fn remove<'t>(
            &self,
            id: <<Self as Repository>::Aggregate as Aggregate>::IntoId,
            transaction: Option<&'t <Self as Repository>::Transaction>,
        ) -> Result<(), <Self as Repository>::Error>;
    }

    impl BankAccountRepository for BankAccountRepository {}
}
