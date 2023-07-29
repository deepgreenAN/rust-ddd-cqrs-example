use crate::Event;

use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;

// -------------------------------------------------------------------------------------------------
// Subscribe

/// イベントハンドラが実装すべきトレイト．
#[async_trait]
pub trait Subscribe: Send + Sync {
    type InputEvent: Event;
    type Error: std::error::Error;

    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), Self::Error>;
}

// -------------------------------------------------------------------------------------------------
// FuncSubscriber

/// 関数をSubscribeを実装した型にする
/// Pin<Box<dyn Future>>をeventと同じライフタイムにするため高階トレイト境界を使っている
pub(crate) struct AsyncFuncSubscriber<Ev: Event, Er: std::error::Error> {
    inner_func: Box<
        dyn for<'event> Fn(
                &'event Ev,
            )
                -> Pin<Box<dyn Future<Output = Result<(), Er>> + Send + 'event>>
            + Send
            + Sync,
    >,
}

impl<Ev: Event, Er: std::error::Error> AsyncFuncSubscriber<Ev, Er> {
    pub(crate) fn from_pinned_fn<F>(func: F) -> Self
    where
        F: for<'event> Fn(
                &'event Ev,
            )
                -> Pin<Box<dyn Future<Output = Result<(), Er>> + Send + 'event>>
            + Send
            + Sync
            + 'static,
    {
        Self {
            inner_func: Box::new(func)
                as Box<
                    dyn for<'event> Fn(
                            &'event Ev,
                        ) -> Pin<
                            Box<dyn Future<Output = Result<(), Er>> + Send + 'event>,
                        > + Send
                        + Sync,
                >,
        }
    }
}

#[async_trait]
impl<Ev: Event, Er: std::error::Error> Subscribe for AsyncFuncSubscriber<Ev, Er> {
    type InputEvent = Ev;
    type Error = Er;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), Self::Error> {
        (self.inner_func)(event).await
    }
}
