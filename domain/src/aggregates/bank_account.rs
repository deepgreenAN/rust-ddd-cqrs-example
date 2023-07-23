mod email_address;
mod name;

pub use self::email_address::EmailAddress;
use crate::error::DomainError;
use crate::id::Id;
pub use name::AccountName;

use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------------------------------
// BankAccountId

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BankAccountIdType;

pub type BankAccountId = Id<BankAccountIdType>;

// -------------------------------------------------------------------------------------------------
// BankAccount

/// アグリゲイトとなる銀行アカウント
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BankAccount {
    /// id
    id: BankAccountId,
    /// 残高
    balance: f64,
    /// メールアドレス
    email_address: EmailAddress,
    /// 口座名
    account_name: AccountName,
}

impl BankAccount {
    pub fn from_domains(email_address: EmailAddress, account_name: AccountName) -> Self {
        BankAccount {
            id: BankAccountId::generate(),
            balance: 0_f64,
            email_address,
            account_name,
        }
    }
    pub fn from_primitives(
        email_address: String,
        first_name: String,
        last_name: String,
    ) -> Result<Self, DomainError> {
        Ok(BankAccount {
            id: BankAccountId::generate(),
            balance: 0_f64,
            email_address: email_address.try_into()?,
            account_name: AccountName::from_primitives(first_name, last_name)?,
        })
    }
    pub fn id(&self) -> BankAccountId {
        self.id
    }
    pub fn balance(&self) -> f64 {
        self.balance
    }
    pub fn balance_mut(&mut self) -> &mut f64 {
        &mut self.balance
    }
    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
    pub fn email_address_mut(&mut self) -> &mut EmailAddress {
        &mut self.email_address
    }
    pub fn account_name(&self) -> &AccountName {
        &self.account_name
    }
    pub fn account_name_mut(&mut self) -> &mut AccountName {
        &mut self.account_name
    }
}
