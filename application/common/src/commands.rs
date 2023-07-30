pub mod atm_commands;
pub mod bank_account_commands;

use domain::Id;

// -------------------------------------------------------------------------------------------------
// CommandId

/// リトライなどで重複したコマンドの実行を防ぐためのID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandIdType;

pub type CommandId = Id<CommandIdType>;
