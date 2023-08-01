use crate::aggregates::atm::AtmId;
use crate::aggregates::bank_account::{BankAccountId, EmailAddress};

use serde::{Deserialize, Serialize};

/// アカウントが開設される時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(event_bus::Event))]
pub struct AccountOpenedEvent {
    pub account_id: BankAccountId,
    pub email_address: EmailAddress,
}

/// 預金する時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(event_bus::Event))]
pub struct CustomerDepositedMoneyEvent {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub balance: f64,
    pub atm_id: AtmId,
}

/// 引き出した時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(event_bus::Event))]
pub struct CustomerWithdrewCashEvent {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub balance: f64,
    pub atm_id: AtmId,
}

/// 小切手を発行したときにレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(event_bus::Event))]
pub struct CustomerWroteCheckEvent {
    pub account_id: BankAccountId,
    /// 外部マイクロサービスを用いるため，プリミティブな型
    pub check_number: String,
    pub amount: f64,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpenedEvent(AccountOpenedEvent),
    CustomerDepositedMoneyEvent(CustomerDepositedMoneyEvent),
    CustomerWithdrewCashEvent(CustomerWithdrewCashEvent),
    CustomerWroteCheckEvent(CustomerWroteCheckEvent),
}

crate::generate_enum_from!(
    BankAccountEvent,
    AccountOpenedEvent,
    CustomerDepositedMoneyEvent,
    CustomerWithdrewCashEvent,
    CustomerWroteCheckEvent
);
