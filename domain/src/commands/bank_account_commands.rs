use crate::aggregates::bank_account::{AccountName, BankAccountId};

use serde::{Deserialize, Serialize};

/// アカウント開設のコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAccountCommand {
    pub id: BankAccountId,
    pub account_name: AccountName,
}

/// 預金するコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepositMoneyCommand {
    pub id: BankAccountId,
    pub amount: f64,
}

/// 引き出しを行うコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawMoneyCommand {
    pub id: BankAccountId,
    pub amount: f64,
    /// 外部マイクロサービスについての処理であるため，プリミティブな型
    pub atm_id: String,
}

/// 小切手の発行を行うコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WriteCheckCommand {
    pub id: BankAccountId,
    pub amount: f64,
    /// 外部マイクロサービスについての処理であるため，プリミティブな型
    pub check_number: String,
}

/// bank_accountアグリゲイトに関わるコマンド全体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BankAccountCommand {
    OpenAccountCommand(OpenAccountCommand),
    DepositMoneyCommand(DepositMoneyCommand),
    WithdrawMoneyCommand(WithdrawMoneyCommand),
    WriteCheckCommand(WriteCheckCommand),
}

// impl Fromを生成
crate::generate_enum_from!(
    BankAccountCommand,
    OpenAccountCommand,
    DepositMoneyCommand,
    WithdrawMoneyCommand,
    WriteCheckCommand
);
