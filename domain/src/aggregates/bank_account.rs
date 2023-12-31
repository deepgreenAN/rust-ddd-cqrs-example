mod email_address;
mod name;

pub use self::email_address::EmailAddress;
use crate::error::{BankAccountError, DomainError};
use crate::events::bank_account_events::{self, BankAccountEvent};
use crate::id::Id;
use config::CONFIG;
use ddd_cqrs_core::{Aggregate, DomainEventList};
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
    /// 口座が有効かどうか
    opened: bool,
    /// 残高
    balance: f64,
    /// メールアドレス
    email_address: EmailAddress,
    /// 口座名
    account_name: AccountName,
    /// イベントのリスト
    #[serde(skip)]
    events_list: DomainEventList<BankAccountEvent>,
}

impl BankAccount {
    pub fn from_domains(email_address: EmailAddress, account_name: AccountName) -> Self {
        BankAccount {
            id: BankAccountId::generate(),
            opened: false,
            balance: 0_f64,
            email_address,
            account_name,
            events_list: DomainEventList::new(),
        }
    }
    pub fn from_primitives(
        email_address: String,
        first_name: String,
        last_name: String,
    ) -> Result<Self, DomainError> {
        Ok(BankAccount {
            id: BankAccountId::generate(),
            opened: false,
            balance: 0_f64,
            email_address: email_address.try_into()?,
            account_name: AccountName::from_primitives(first_name, last_name)?,
            events_list: DomainEventList::new(),
        })
    }
    pub fn id(&self) -> BankAccountId {
        self.id
    }
    pub fn opened(&self) -> bool {
        self.opened
    }
    pub fn balance(&self) -> f64 {
        self.balance
    }
    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
    pub fn account_name(&self) -> &AccountName {
        &self.account_name
    }

    // -------------------------------------------------------------------------------------------------
    // 以下はドメインロジック

    /// アカウントの利用を可能にする．
    pub fn open_account(&mut self) {
        let event = bank_account_events::AccountOpenedEvent {
            account_id: self.id,
            email_address: self.email_address.clone(),
        };

        self.opened = true;
        self.domain_events_mut().push(event.into());
    }
    /// 預金を行う
    pub fn deposit_money(&mut self, amount: f64) -> Result<(), DomainError> {
        if self.balance + amount > CONFIG.BALANCE_UPPER_LIM {
            Err(BankAccountError::DepositExceedLimitError {
                limit: CONFIG.BALANCE_UPPER_LIM,
                amount,
                exceed_balance: self.balance + amount,
            }
            .into())
        } else {
            self.balance += amount;
            Ok(())
        }
    }
    /// 引き出しを行う
    pub fn withdraw_money(&mut self, amount: f64) -> Result<(), DomainError> {
        if self.balance - amount > 0.0 {
            self.balance -= amount;
            Ok(())
        } else {
            Err(BankAccountError::WithdrawExceedBalanceError {
                amount,
                balance: self.balance,
            }
            .into())
        }
    }
    /// 小切手を利用する
    pub fn write_check(&mut self, amount: f64, check_number: String) -> Result<(), DomainError> {
        if self.balance - amount > 0.0 {
            let event = bank_account_events::CustomerWroteCheckEvent {
                account_id: self.id,
                amount,
                check_number,
                balance: self.balance,
            };

            self.balance -= amount;
            self.domain_events_mut().push(event.into());
            Ok(())
        } else {
            Err(BankAccountError::CheckExceedBalanceError {
                amount,
                balance: self.balance,
            }
            .into())
        }
    }
}

impl Aggregate for BankAccount {
    type Event = BankAccountEvent;
    type IntoId = BankAccountId;
    fn id(&self) -> Self::IntoId {
        self.id
    }
    fn domain_events(&self) -> &DomainEventList<Self::Event> {
        &self.events_list
    }
    fn domain_events_mut(&mut self) -> &mut DomainEventList<Self::Event> {
        &mut self.events_list
    }
}

// -------------------------------------------------------------------------------------------------
// sea_orm用Model

#[cfg(feature = "orm")]
pub mod orm {
    use super::*;
    use sea_orm::entity::prelude::*;
    use serde::Serialize;

    /// BankAccountに対するORMモデル．
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "bank_account")]
    pub struct Model {
        /// id
        #[sea_orm(primary_key, auto_increment = false, unique)]
        id: BankAccountId,
        /// 口座が有効かどうか
        opened: bool,
        /// 残高
        balance: f64,
        /// メールアドレス
        email_address: EmailAddress,
        /// 口座名
        account_name: AccountName,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

    /// 双方のFromを実装することで，フィールド対応のバグを減らすことができる．
    impl From<Model> for BankAccount {
        fn from(value: Model) -> Self {
            let Model {
                id,
                opened,
                balance,
                email_address,
                account_name,
            } = value;

            Self {
                id,
                opened,
                balance,
                email_address,
                account_name,
                events_list: Default::default(),
            }
        }
    }

    impl From<BankAccount> for Model {
        fn from(value: BankAccount) -> Self {
            let BankAccount {
                id,
                opened,
                balance,
                email_address,
                account_name,
                events_list: _,
            } = value;

            Self {
                id,
                opened,
                balance,
                email_address,
                account_name,
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// impl Dummy

#[cfg(any(test, feature = "fake"))]
impl fake::Dummy<fake::Faker> for BankAccount {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use fake::{Fake, Faker};

        Self {
            id: Faker.fake_with_rng(rng),
            opened: Faker.fake_with_rng(rng),
            balance: (0.0..CONFIG.BALANCE_UPPER_LIM).fake_with_rng(rng),
            email_address: Faker.fake_with_rng(rng),
            account_name: Faker.fake_with_rng(rng),
            events_list: DomainEventList::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// test

#[cfg(test)]
mod test {
    use super::{orm, BankAccount};

    #[cfg(feature = "orm")]
    mod orm_test {
        use super::{orm, BankAccount};
        use fake::{Fake, Faker};

        #[test]
        fn aggregate_model_serde() {
            let bank_account = Faker.fake::<BankAccount>();

            let json_from_model =
                serde_json::to_string(&(Into::<orm::Model>::into(bank_account.clone()))).unwrap();

            assert_eq!(
                bank_account,
                serde_json::from_str(&json_from_model).unwrap()
            );
        }
    }
}
