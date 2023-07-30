use crate::{Event, Subscribe};

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_global_executor::Task;

// -------------------------------------------------------------------------------------------------
// EventBus

/// イベントバス．
pub struct EventBus<E: Event, O: Send> {
    subscribers: Vec<Arc<dyn Subscribe<InputEvent = E, Output = O>>>,
}

impl<E: Event, O: Send + 'static> EventBus<E, O> {
    pub const fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
    fn subscribe_arc(&mut self, subscriber: Arc<dyn Subscribe<InputEvent = E, Output = O>>) {
        self.subscribers.push(subscriber);
    }
    /// サブスクライバーを追加する．
    pub fn subscribe<S: Subscribe<InputEvent = E, Output = O> + 'static>(&mut self, subscriber: S) {
        self.subscribe_arc(Arc::new(subscriber));
    }
    /// Pin<Box<dyn Future<Output = ()>>>を返す関数をサブスクライバーとして追加する．
    pub fn subscribe_pinned_fn<F>(&mut self, func: F)
    where
        F: for<'a> Fn(&'a E) -> Pin<Box<dyn Future<Output = O> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    {
        self.subscribe(crate::subscribe::AsyncFuncSubscriber::from_pinned_fn(func));
    }
    /// イベントをサブスクライバーに通知して非同期実行しハンドルを返す．
    pub fn dispatch_event(&self, event: E) -> Vec<Task<O>> {
        let mut tasks = Vec::new();
        let event = Arc::new(event);

        for subscriber in self.subscribers.iter() {
            let subscriber = Arc::clone(subscriber);
            let event = Arc::clone(&event);

            let task =
                async_global_executor::spawn(async move { subscriber.handle_event(&event).await });

            tasks.push(task);
        }
        tasks
    }
    /// EventBusのextend
    pub fn extend(&mut self, other: EventBus<E, O>) {
        let EventBus { subscribers } = other;
        self.subscribers.extend(subscribers);
    }
}

// -------------------------------------------------------------------------------------------------
// EventBus生成用のマクロ

#[macro_export]
macro_rules! event_bus_from_subscribes {
    ($($subscriber:expr),*) => {
        {
            let mut bus = $crate::EventBus::new();
            $(
                bus.subscribe($subscriber);
            )*
            bus
        }
    };
}

#[macro_export]
macro_rules! event_bus_from_subscriber_pinned_fns {
    ($($subscriber_fn:expr),*) => {
        {
            let mut bus = $crate::EventBus::new();
            $(
                bus.subscribe_pinned_fn($subscriber_fn);
            )*
            bus
        }
    };
}
