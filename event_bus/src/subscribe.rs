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

    async fn handle_event<'event>(&self, event: &'event Self::InputEvent);
}

// -------------------------------------------------------------------------------------------------
// FuncSubscriber

/// 関数をSubscribeを実装した型にする
/// Pin<Box<dyn Future>>をeventと同じライフタイムにするため高階トレイト境界を使っている
pub(crate) struct AsyncFuncSubscriber<E: Event> {
    inner_func: Box<
        dyn for<'event> Fn(&'event E) -> Pin<Box<dyn Future<Output = ()> + Send + 'event>>
            + Send
            + Sync,
    >,
}

impl<E: Event> AsyncFuncSubscriber<E> {
    pub(crate) fn from_pinned_fn<F>(func: F) -> Self
    where
        F: for<'event> Fn(&'event E) -> Pin<Box<dyn Future<Output = ()> + Send + 'event>>
            + Send
            + Sync
            + 'static,
    {
        Self {
            inner_func: Box::new(func)
                as Box<
                    dyn for<'event> Fn(
                            &'event E,
                        )
                            -> Pin<Box<dyn Future<Output = ()> + Send + 'event>>
                        + Send
                        + Sync,
                >,
        }
    }
}

#[async_trait]
impl<E: Event> Subscribe for AsyncFuncSubscriber<E> {
    type InputEvent = E;
    async fn handle_event<'event>(&self, event: &'event Self::InputEvent) {
        (self.inner_func)(event).await;
    }
}
