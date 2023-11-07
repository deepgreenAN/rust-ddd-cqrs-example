use super::CommandId;
use domain::aggregates::atm::AtmId;
use domain::aggregates::bank_account::{AccountName, BankAccountId, EmailAddress};

use serde::{Deserialize, Serialize};

/// アカウント開設のコマンド
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "fake", derive(fake::Dummy))]
pub struct OpenAccountCommand {
    pub account_name: AccountName,
    pub email_address: EmailAddress,
}

/// 預金するコマンド
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "fake", derive(fake::Dummy))]
pub struct DepositMoneyCommand {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub atm_id: AtmId,
}

/// 引き出しを行うコマンド
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "fake", derive(fake::Dummy))]
pub struct WithdrawMoneyCommand {
    pub account_id: BankAccountId,
    pub amount: f64,
    pub atm_id: AtmId,
}

/// 小切手の発行を行うコマンド
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "fake", derive(fake::Dummy))]
pub struct WriteCheckCommand {
    pub account_id: BankAccountId,
    pub amount: f64,
    /// 外部マイクロサービスについての処理であるため，プリミティブな型
    pub check_number: String,
}

// -------------------------------------------------------------------------------------------------
// 以下参照バージョン

/// アカウント開設のコマンド(参照)
#[cfg(feature = "frontend")]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct OpenAccountRefCommand<'a> {
    pub account_name: &'a AccountName,
    pub email_address: &'a EmailAddress,
}

/// 小切手の発行を行うコマンド(参照)
#[cfg(feature = "frontend")]
#[derive(Debug, PartialEq, Clone, Serialize)]
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
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum BankAccountCommand {
    OpenAccountCommand(OpenAccountCommand, CommandId),
    DepositMoneyCommand(DepositMoneyCommand, CommandId),
    WithdrawMoneyCommand(WithdrawMoneyCommand, CommandId),
    WriteCheckCommand(WriteCheckCommand, CommandId),
}

// -------------------------------------------------------------------------------------------------
// BankAccountRefCommand

/// bank_accountアグリゲイトに関わる参照コマンド全体(フロントエンド側で利用)
#[cfg(feature = "frontend")]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum BankAccountRefCommand<'a> {
    OpenAccountCommand(OpenAccountRefCommand<'a>, CommandId),
    DepositMoneyCommand(DepositMoneyCommand, CommandId),
    WithdrawMoneyCommand(WithdrawMoneyCommand, CommandId),
    WriteCheckCommand(WriteCheckRefCommand<'a>, CommandId),
}

#[cfg(all(test, all(feature = "server", feature = "frontend")))]
mod serde_test {
    use fake::{Fake, Faker};

    #[test]
    fn bank_account_test() {
        use super::{
            BankAccountCommand, BankAccountRefCommand, OpenAccountCommand, OpenAccountRefCommand,
        };
        use crate::commands::CommandId;
        use domain::aggregates::bank_account::{AccountName, EmailAddress};

        let account_name: AccountName = Faker.fake();
        let email_address: EmailAddress = Faker.fake();
        let command_id = CommandId::generate();

        let open_account_ref_command = BankAccountRefCommand::OpenAccountCommand(
            OpenAccountRefCommand {
                account_name: &account_name,
                email_address: &email_address,
            },
            command_id,
        );

        let open_account_command_from_json: BankAccountCommand = serde_json::from_str(
            serde_json::to_string(&open_account_ref_command)
                .unwrap()
                .as_str(),
        )
        .unwrap();

        let open_account_command = BankAccountCommand::OpenAccountCommand(
            OpenAccountCommand {
                account_name,
                email_address,
            },
            command_id,
        );

        assert_eq!(open_account_command_from_json, open_account_command)
    }
}
