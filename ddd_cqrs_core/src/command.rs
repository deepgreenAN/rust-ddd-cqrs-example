use crate::Aggregate;

/// コマンドハンドラが実装すべきトレイト
#[async_trait::async_trait]
pub trait HandleCommand {
    type Command;
    type Aggregate: Aggregate;
    type Error: std::error::Error;

    /// コマンドハンドラの実装部分
    async fn handle_command(
        &self,
        command: Self::Command,
    ) -> Result<Vec<<Self::Aggregate as Aggregate>::Event>, Self::Error>;

    /// コマンドの重複を許すかどうか
    fn allow_duplicate(&self) -> bool {
        false
    }
}
