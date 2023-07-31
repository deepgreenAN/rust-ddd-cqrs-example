#![cfg(all(feature = "server", feature = "frontend"))]

fn main() {
    use common::commands::bank_account_commands::{
        BankAccountCommand, BankAccountRefCommand, OpenAccountCommand, OpenAccountRefCommand,
    };
    use common::commands::CommandId;
    use domain::aggregates::bank_account::{AccountName, EmailAddress};

    let account_name =
        AccountName::from_primitives("山田".to_string(), "太郎".to_string()).unwrap();
    let email_address = EmailAddress::try_from("xxxyyyzzz@gmail.com".to_string()).unwrap();
    let command_id = CommandId::generate();

    let open_account_command = BankAccountRefCommand::OpenAccountCommand(
        OpenAccountRefCommand {
            account_name: &account_name,
            email_address: &email_address,
        },
        command_id,
    );

    let open_account_command_from_json: BankAccountCommand = serde_json::from_str(
        serde_json::to_string(&open_account_command)
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let open_account_command_2 = BankAccountCommand::OpenAccountCommand(
        OpenAccountCommand {
            account_name,
            email_address,
        },
        command_id,
    );

    assert_eq!(open_account_command_from_json, open_account_command_2)
}
