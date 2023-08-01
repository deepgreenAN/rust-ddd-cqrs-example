/// イベントが実装するトレイト
pub trait Event: Send + Sync + 'static {
    fn event_type() -> String
    where
        Self: Sized;
}
