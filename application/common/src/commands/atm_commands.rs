use super::CommandId;
use domain::aggregates::atm::AtmLocation;

use serde::{Deserialize, Serialize};

/// Atm登録のコマンド
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterAtmCommand {
    pub location: AtmLocation,
    pub total_cash: f64,
}

// -------------------------------------------------------------------------------------------------
// 以下参照バージョン

/// Atm登録のコマンド(参照)
#[derive(Debug, Clone, Serialize)]
pub struct RegisterAtmRefCommand<'a> {
    pub location: &'a AtmLocation,
    pub total_cash: f64,
}

// -------------------------------------------------------------------------------------------------
// AtmCommand

/// Atmに関するコマンド(サーバーサイドで利用する)
#[cfg(feature = "server")]
#[derive(Debug, Clone, Deserialize)]
pub enum AtmCommand {
    RegisterAtmCommand(RegisterAtmCommand, CommandId),
}

// -------------------------------------------------------------------------------------------------
// AtmRefCommand

#[cfg(feature = "frontend")]
#[derive(Debug, Clone, Serialize)]
pub enum AtmCommand<'a> {
    RegisterAtmCommand(RegisterAtmRefCommand<'a>, CommandId),
}
