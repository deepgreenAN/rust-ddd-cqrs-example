use crate::DomainEvent;

/// コマンドハンドラが実装すべきトレイト
#[async_trait::async_trait]
pub trait HandleCommand {
    type Command;
    type Event: DomainEvent;
    type Error: std::error::Error;

    /// コマンドハンドラの実装部分
    async fn handle_command(
        &mut self,
        command: Self::Command,
    ) -> Result<Vec<Self::Event>, Self::Error>;

    /// コマンドIDの重複を許すかどうか
    fn allow_duplicate(&self) -> bool {
        false
    }
}
