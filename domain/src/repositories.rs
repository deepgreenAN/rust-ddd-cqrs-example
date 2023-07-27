use crate::aggregates::atm::AtmId;
use crate::aggregates::bank_account::BankAccountId;
use crate::aggregates::{Atm, BankAccount};

/// BankAccountのリポジトリ
#[async_trait::async_trait]
pub trait BankAccountRepository {
    type Error;
    fn save(&self, bank_account: BankAccount) -> Result<(), Self::Error>;
    fn edit(&self, bank_account: BankAccount) -> Result<(), Self::Error>;
    fn open(&self, id: BankAccountId) -> Result<(), Self::Error>;
    fn remove(&self, id: BankAccountId) -> Result<(), Self::Error>;
}

/// Atmのリポジトリ
#[async_trait::async_trait]
pub trait AtmRepository {
    type Error;
    fn save(&self, atm: Atm) -> Result<(), Self::Error>;
    fn edit(&self, atm: Atm) -> Result<(), Self::Error>;
    fn remove(&self, id: AtmId) -> Result<(), Self::Error>;
}
