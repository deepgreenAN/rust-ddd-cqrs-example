use super::CommandId;
use crate::aggregates::atm::AtmId;
use crate::aggregates::bank_account::{AccountName, BankAccountId, EmailAddress};

use serde::{Deserialize, Serialize};

/// アカウント開設のコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAccountCommand {
    pub account_name: AccountName,
    pub email_address: EmailAddress,
}

/// 預金するコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepositMoneyCommand {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub atm_id: AtmId,
}

/// 引き出しを行うコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawMoneyCommand {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub atm_id: AtmId,
}

/// 小切手の発行を行うコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WriteCheckCommand {
    pub account_id: BankAccountId,
    pub amount: f64,
    /// 外部マイクロサービスについての処理であるため，プリミティブな型
    pub check_number: String,
}

// -------------------------------------------------------------------------------------------------
// 以下参照バージョン

/// アカウント開設のコマンド(参照)
#[derive(Debug, Clone, Serialize)]
pub struct OpenAccountRefCommand<'a> {
    pub account_name: &'a AccountName,
    pub email_address: &'a EmailAddress,
}

/// 小切手の発行を行うコマンド(参照)
#[derive(Debug, Clone, Serialize)]
pub struct WriteCheckRefCommand<'a> {
    pub account_id: BankAccountId,
    pub amount: f64,
    /// 外部マイクロサービスについての処理であるため，プリミティブな型
    pub check_number: &'a String,
}

// -------------------------------------------------------------------------------------------------
// BankAccountCommand

/// bank_accountアグリゲイトに関わるコマンド全体(サーバーサイド側で利用)
#[cfg(feature = "server")]
#[derive(Debug, Clone, Deserialize)]
pub enum BankAccountCommand {
    OpenAccountCommand(OpenAccountCommand, CommandId),
    DepositMoneyCommand(DepositMoneyCommand, CommandId),
    WithdrawMoneyCommand(WithdrawMoneyCommand, CommandId),
    WriteCheckCommand(WriteCheckCommand, CommandId),
}

#[cfg(feature = "server")]
impl BankAccountCommand {
    pub fn id(&self) -> CommandId {
        match self {
            Self::OpenAccountCommand(_, id) => *id,
            Self::DepositMoneyCommand(_, id) => *id,
            Self::WithdrawMoneyCommand(_, id) => *id,
            Self::WriteCheckCommand(_, id) => *id,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// BankAccountRefCommand

/// bank_accountアグリゲイトに関わる参照コマンド全体(フロントエンド側で利用)
#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, Serialize)]
pub enum BankAccountRefCommand<'a> {
    OpenAccountCommand(OpenAccountRefCommand<'a>, CommandId),
    DepositMoneyCommand(DepositMoneyCommand, CommandId),
    WithdrawMoneyCommand(WithdrawMoneyCommand, CommandId),
    WriteCheckCommand(WriteCheckRefCommand<'a>, CommandId),
}
