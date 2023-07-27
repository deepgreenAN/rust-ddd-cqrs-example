use super::CommandId;
use crate::aggregates::atm::AtmId;
use crate::aggregates::bank_account::{AccountName, BankAccountId};

use serde::{Deserialize, Serialize};

/// アカウント開設のコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAccountCommand {
    pub command_id: CommandId,
    pub account_id: BankAccountId,
    pub account_name: AccountName,
}

/// 預金するコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepositMoneyCommand {
    pub command_id: CommandId,
    pub account_id: BankAccountId,
    pub amount: f64,
}

/// 引き出しを行うコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawMoneyCommand {
    pub command_id: CommandId,
    pub account_id: BankAccountId,
    pub amount: f64,
    pub atm_id: AtmId,
}

/// 小切手の発行を行うコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WriteCheckCommand {
    pub command_id: CommandId,
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
    pub command_id: CommandId,
    pub account_id: BankAccountId,
    pub account_name: &'a AccountName,
}

/// 小切手の発行を行うコマンド(参照)
#[derive(Debug, Clone, Serialize)]
pub struct WriteCheckRefCommand<'a> {
    pub command_id: CommandId,
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
    OpenAccountCommand(OpenAccountCommand),
    DepositMoneyCommand(DepositMoneyCommand),
    WithdrawMoneyCommand(WithdrawMoneyCommand),
    WriteCheckCommand(WriteCheckCommand),
}

// // impl Fromを生成
// crate::generate_enum_from!(
//     BankAccountCommand,
//     OpenAccountCommand,
//     DepositMoneyCommand,
//     WithdrawMoneyCommand,
//     WriteCheckCommand
// );

// -------------------------------------------------------------------------------------------------
// BankAccountRefCommand

/// bank_accountアグリゲイトに関わる参照コマンド全体(フロントエンド側で利用)
#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, Serialize)]
pub enum BankAccountRefCommand<'a> {
    OpenAccountCommand(OpenAccountRefCommand<'a>),
    DepositMoneyCommand(DepositMoneyCommand),
    WithdrawMoneyCommand(WithdrawMoneyCommand),
    WriteCheckCommand(WriteCheckRefCommand<'a>),
}
