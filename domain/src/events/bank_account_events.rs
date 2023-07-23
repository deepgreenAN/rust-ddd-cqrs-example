use crate::aggregates::bank_account::{BankAccountId, EmailAddress};

use serde::{Deserialize, Serialize};

/// アカウントが開設される時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountOpenedEvent {
    pub id: BankAccountId,
    pub email_address: EmailAddress,
}

/// 預金する時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerDepositedMoneyEvent {
    pub id: BankAccountId,
    pub amount: f64,
    pub balance: f64,
}

/// 引き出した時にレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerWithdrewCashEvent {
    pub id: BankAccountId,
    pub amount: f64,
    pub balance: f64,
}

/// 小切手を発行したときにレイズされるイベント
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerWroteCheckEvent {
    pub id: BankAccountId,
    /// 外部マイクロサービスを用いるため，プリミティブな型
    pub check_number: String,
    pub amount: f64,
    pub balance: f64,
}

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
