use crate::aggregates::atm::AtmId;
use crate::aggregates::bank_account::{BankAccountId, EmailAddress};
use config::CONFIG;
use ddd_cqrs_core::DomainEvent;

use serde::{Deserialize, Serialize};

/// アカウントが開設される時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountOpenedEvent {
    pub account_id: BankAccountId,
    pub email_address: EmailAddress,
}

#[cfg(feature = "server")]
impl event_bus::Event for AccountOpenedEvent {}

/// 預金する時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerDepositedMoneyEvent {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub balance: f64,
    pub atm_id: AtmId,
}

#[cfg(feature = "server")]
impl event_bus::Event for CustomerDepositedMoneyEvent {}

/// 引き出した時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerWithdrewCashEvent {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub balance: f64,
    pub atm_id: AtmId,
}

#[cfg(feature = "server")]
impl event_bus::Event for CustomerWithdrewCashEvent {}

/// 小切手を発行したときにレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerWroteCheckEvent {
    pub account_id: BankAccountId,
    /// 外部マイクロサービスを用いるため，プリミティブな型
    pub check_number: String,
    pub amount: f64,
    pub balance: f64,
}

#[cfg(feature = "server")]
impl event_bus::Event for CustomerWroteCheckEvent {}

/// BankAccountに関するイベント全体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpenedEvent(AccountOpenedEvent),
    CustomerDepositedMoneyEvent(CustomerDepositedMoneyEvent),
    CustomerWithdrewCashEvent(CustomerWithdrewCashEvent),
    CustomerWroteCheckEvent(CustomerWroteCheckEvent),
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let type_name = match self {
            Self::AccountOpenedEvent(_) => "BankAccountEvent::AccountOpenedEvent",
            Self::CustomerDepositedMoneyEvent(_) => "BankAccountEvent::CustomerDepositedMoneyEvent",
            Self::CustomerWithdrewCashEvent(_) => "BankAccountEvent::CustomerWithdrewCashEvent",
            Self::CustomerWroteCheckEvent(_) => "BankAccountEvent::CustomerWroteCheckEvent",
        };
        type_name.to_string()
    }
    fn event_version() -> String {
        CONFIG.EVENT_VERSION.to_string()
    }
}

crate::generate_enum_from!(
    BankAccountEvent,
    AccountOpenedEvent,
    CustomerDepositedMoneyEvent,
    CustomerWithdrewCashEvent,
    CustomerWroteCheckEvent
);
