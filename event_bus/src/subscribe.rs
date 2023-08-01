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
    type Output: Send;

    async fn handle_event<'event>(&self, event: &'event Self::InputEvent) -> Self::Output;
}

// -------------------------------------------------------------------------------------------------
// FuncSubscriber

/// 関数をSubscribeを実装した型にする
/// Pin<Box<dyn Future>>をeventと同じライフタイムにするため高階トレイト境界を使っている
pub struct AsyncFuncSubscriber<E: Event, O: Send> {
    inner_func: Box<
        dyn for<'event> Fn(&'event E) -> Pin<Box<dyn Future<Output = O> + Send + 'event>>
            + Send
            + Sync,
    >,
}

impl<E: Event, O: Send> AsyncFuncSubscriber<E, O> {
    pub fn from_pinned_fn<F>(func: F) -> Self
    where
        F: for<'event> Fn(&'event E) -> Pin<Box<dyn Future<Output = O> + Send + 'event>>
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
                            -> Pin<Box<dyn Future<Output = O> + Send + 'event>>
                        + Send
                        + Sync,
                >,
        }
    }
}

#[async_trait]
impl<E: Event, O: Send> Subscribe for AsyncFuncSubscriber<E, O> {
    type InputEvent = E;
    type Output = O;
    async fn handle_event<'event>(&self, event: &'event Self::InputEvent) -> Self::Output {
        (self.inner_func)(event).await
    }
}
