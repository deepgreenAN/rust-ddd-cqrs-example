use crate::{Event, Subscribe};

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_global_executor::Task;

// -------------------------------------------------------------------------------------------------
// EventBs

/// イベントバス．
pub struct EventBus<E: Event> {
    subscribers: Vec<Arc<dyn Subscribe<InputEvent = E>>>,
}

impl<E: Event> EventBus<E> {
    pub const fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
    fn subscribe_arc(&mut self, subscriber: Arc<dyn Subscribe<InputEvent = E>>) {
        self.subscribers.push(subscriber);
    }
    /// サブスクライバーを追加する．
    pub fn subscribe<S: Subscribe<InputEvent = E> + 'static>(&mut self, subscriber: S) {
        self.subscribe_arc(Arc::new(subscriber));
    }
    /// Pin<Box<dyn Future<Output = ()>>>を返す関数をサブスクライバーとして追加する．
    pub fn subscribe_pinned_fn<F>(&mut self, func: F)
    where
        F: for<'a> Fn(&'a E) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    {
        self.subscribe(crate::subscribe::AsyncFuncSubscriber::from_pinned_fn(func));
    }
    /// イベントをサブスクライバーに通知して非同期実行しハンドルを返す．
    pub fn dispatch_event(&mut self, event: E) -> Vec<Task<()>> {
        let mut tasks = Vec::new();
        let event = Arc::new(event);

        for subscriber in self.subscribers.iter() {
            let subscriber = Arc::clone(subscriber);
            let event = Arc::clone(&event);

            let task = async_global_executor::spawn(async move {
                subscriber.handle_event(&event).await;
            });

            tasks.push(task);
        }
        tasks
    }
}
