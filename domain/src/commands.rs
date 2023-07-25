pub mod bank_account_commands;

use crate::id::Id;

// -------------------------------------------------------------------------------------------------
// CommandId

/// リトライなどで重複したコマンドの実行を防ぐためのID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandIdType;

pub type CommandId = Id<CommandIdType>;
