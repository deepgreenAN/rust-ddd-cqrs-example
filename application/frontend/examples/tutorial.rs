#[tokio::main]
async fn main() {
    use frontend::aggregates::{atm, bank_account};
    use frontend::commands::atm_commands::RegisterAtmRefCommand;
    use frontend::commands::bank_account_commands::{DepositMoneyCommand, OpenAccountRefCommand};
    use frontend::CommandId;
    use frontend::{execute_atm_command, execute_bank_account_command};
    use frontend::{AtmCommand, BankAccountCommand};

    // Atmの登録
    {
        let location = atm::AtmLocation::new("東京都");
        execute_atm_command(AtmCommand::RegisterAtmCommand(
            RegisterAtmRefCommand {
                location: &location,
                total_cash: 100_000_000.0,
            },
            CommandId::generate(),
        ))
        .await
        .unwrap();
    }
    let atm = {
        let location = atm::AtmLocation::new("東京都");
        frontend::queries::atm_queries::atm_from_location(&location)
            .await
            .unwrap()
            .unwrap()
    };
    println!("atm: {atm:?}");

    // 口座の開設
    {
        let account_name =
            bank_account::AccountName::from_primitives("山田".to_string(), "太郎".to_string())
                .unwrap();
        let email_address =
            bank_account::EmailAddress::try_from("aaabbbccc@gmail.com".to_string()).unwrap();

        execute_bank_account_command(BankAccountCommand::OpenAccountCommand(
            OpenAccountRefCommand {
                account_name: &account_name,
                email_address: &email_address,
            },
            CommandId::generate(),
        ))
        .await
        .unwrap();
    }
    {
        let account_name =
            bank_account::AccountName::from_primitives("斎藤".to_string(), "健二".to_string())
                .unwrap();
        let email_address =
            bank_account::EmailAddress::try_from("eeefffggg@gmail.com".to_string()).unwrap();

        execute_bank_account_command(BankAccountCommand::OpenAccountCommand(
            OpenAccountRefCommand {
                account_name: &account_name,
                email_address: &email_address,
            },
            CommandId::generate(),
        ))
        .await
        .unwrap();
    }
    let bank_account = {
        let email_address =
            bank_account::EmailAddress::try_from("aaabbbccc@gmail.com".to_string()).unwrap();
        frontend::queries::bank_account_queries::bank_account_from_email(&email_address)
            .await
            .unwrap()
            .unwrap()
    };
    println!("bank_account: {bank_account:?}");

    // 口座に入金
    {
        execute_bank_account_command(BankAccountCommand::DepositMoneyCommand(
            DepositMoneyCommand {
                account_id: bank_account.id(),
                amount: 100_000.0,
                atm_id: atm.id(),
            },
            CommandId::generate(),
        ))
        .await
        .unwrap();
    }
    // 口座から引き出し
    {
        execute_bank_account_command(BankAccountCommand::DepositMoneyCommand(
            DepositMoneyCommand {
                account_id: bank_account.id(),
                amount: 10_000.0,
                atm_id: atm.id(),
            },
            CommandId::generate(),
        ))
        .await
        .unwrap();
    }

    let updated_atm = {
        let location = atm::AtmLocation::new("東京都");
        frontend::queries::atm_queries::atm_from_location(&location)
            .await
            .unwrap()
            .unwrap()
    };
    println!("updated_atm: {updated_atm:?}");
}
