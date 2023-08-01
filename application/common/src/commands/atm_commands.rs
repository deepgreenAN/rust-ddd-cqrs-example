use super::CommandId;
use domain::aggregates::atm::AtmLocation;

#[cfg(feature = "server")]
use serde::Deserialize;

#[cfg(feature = "frontend")]
use serde::Serialize;

/// Atm登録のコマンド
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterAtmCommand {
    pub location: AtmLocation,
    pub total_cash: f64,
}

// -------------------------------------------------------------------------------------------------
// 以下参照バージョン

/// Atm登録のコマンド(参照)
#[cfg(feature = "frontend")]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RegisterAtmRefCommand<'a> {
    pub location: &'a AtmLocation,
    pub total_cash: f64,
}

// -------------------------------------------------------------------------------------------------
// AtmCommand

/// Atmに関するコマンド(サーバーサイドで利用する)
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum AtmCommand {
    RegisterAtmCommand(RegisterAtmCommand, CommandId),
}

// -------------------------------------------------------------------------------------------------
// AtmRefCommand

#[cfg(feature = "frontend")]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AtmRefCommand<'a> {
    RegisterAtmCommand(RegisterAtmRefCommand<'a>, CommandId),
}

#[cfg(all(test, all(feature = "server", feature = "frontend")))]
mod serde_test {
    #[test]
    fn atm_command() {
        use super::{AtmCommand, AtmRefCommand, RegisterAtmCommand, RegisterAtmRefCommand};
        use crate::commands::CommandId;

        use domain::aggregates::atm::AtmLocation;

        let location = AtmLocation::from("東京".to_string());
        let total_cash = 100_000_000.0;
        let command_id = CommandId::generate();

        let atm_ref_command = AtmRefCommand::RegisterAtmCommand(
            RegisterAtmRefCommand {
                location: &location,
                total_cash,
            },
            command_id,
        );

        let atm_command_from_json: AtmCommand =
            serde_json::from_str(serde_json::to_string(&atm_ref_command).unwrap().as_str())
                .unwrap();

        let atm_command = AtmCommand::RegisterAtmCommand(
            RegisterAtmCommand {
                location,
                total_cash,
            },
            command_id,
        );

        assert_eq!(atm_command_from_json, atm_command);
    }
}
