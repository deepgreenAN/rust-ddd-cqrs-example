use crate::{Event, Subscribe};

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_global_executor::Task;

// -------------------------------------------------------------------------------------------------
// EventBs

/// イベントバス．
pub struct EventBus<Ev: Event, Er: std::error::Error + Send> {
    subscribers: Vec<Arc<dyn Subscribe<InputEvent = Ev, Error = Er>>>,
}

impl<Ev: Event, Er: std::error::Error + Send + 'static> EventBus<Ev, Er> {
    pub const fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
    fn subscribe_arc(&mut self, subscriber: Arc<dyn Subscribe<InputEvent = Ev, Error = Er>>) {
        self.subscribers.push(subscriber);
    }
    /// サブスクライバーを追加する．
    pub fn subscribe<S: Subscribe<InputEvent = Ev, Error = Er> + 'static>(
        &mut self,
        subscriber: S,
    ) {
        self.subscribe_arc(Arc::new(subscriber));
    }
    /// Pin<Box<dyn Future<Output = ()>>>を返す関数をサブスクライバーとして追加する．
    pub fn subscribe_pinned_fn<F>(&mut self, func: F)
    where
        F: for<'a> Fn(&'a Ev) -> Pin<Box<dyn Future<Output = Result<(), Er>> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    {
        self.subscribe(crate::subscribe::AsyncFuncSubscriber::from_pinned_fn(func));
    }
    /// イベントをサブスクライバーに通知して非同期実行しハンドルを返す．
    pub fn dispatch_event(&self, event: Ev) -> Vec<Task<Result<(), Er>>> {
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
    pub fn extend(&mut self, other: EventBus<Ev, Er>) {
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
            (
                bus.subscribe(subscriber);
            )*
            bus
        }
    };
}

#[macro_export]
macro_rules! event_bus_from_subscriber_fns {
    ($($subscriber_fn:expr),*) => {
        {
            let mut bus = $crate::EventBus::new();
            (
                bus.subscribe_fn(subscriber_fn);
            )*
            bus
        }
    };
}
