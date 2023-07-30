pub mod atm_command_handlers;
pub mod bank_account_command_handlers;

use common::ApplicationError;
use serde::de::DeserializeOwned;

/// 統合コマンドが実装すべきトレイト．api_handlerで利用する．
#[async_trait::async_trait]
pub trait ApiHandleCommand {
    type Command: DeserializeOwned;

    async fn handle_command(&self, command: Self::Command) -> Result<(), ApplicationError>;
}
