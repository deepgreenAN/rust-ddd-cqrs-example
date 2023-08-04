use crate::{Event, Subscribe};

use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use async_global_executor::Task;

// -------------------------------------------------------------------------------------------------
// EventBus

type Subscribers<E, O> = Vec<Arc<dyn Subscribe<InputEvent = E, Output = O>>>;

/// イベントバス．
pub struct EventBus<O: Send> {
    subscribers_map: HashMap<String, Box<dyn Any + Send + Sync>>, // E, Subscribers<E, O>を保持できるハッシュマップ
    _output_type: PhantomData<O>,
}

impl<O: Send + 'static> EventBus<O> {
    pub fn new() -> Self {
        Self {
            subscribers_map: HashMap::new(),
            _output_type: PhantomData,
        }
    }
    fn subscribe_arc<E: Event>(
        &mut self,
        subscriber: Arc<dyn Subscribe<InputEvent = E, Output = O>>,
    ) {
        use std::collections::hash_map::Entry::*;

        let event_type = E::event_type();

        match self.subscribers_map.entry(event_type) {
            Occupied(mut o) => {
                let subscribers = o.get_mut().downcast_mut::<Subscribers<E, O>>().unwrap(); // ダウンキャスト結果がNoneとなるのはバグかイベント名が重複する場合である．
                subscribers.push(subscriber);
            }
            Vacant(v) => {
                let subscribers_any = Box::new(vec![subscriber]) as Box<dyn Any + Send + Sync>;
                v.insert(subscribers_any);
            }
        }
    }
    /// サブスクライバーを追加する．
    pub fn subscribe<S, E>(&mut self, subscriber: S)
    where
        S: Subscribe<InputEvent = E, Output = O> + 'static,
        E: Event,
    {
        self.subscribe_arc(Arc::new(subscriber));
    }
    /// Pin<Box<dyn Future<Output = ()>>>を返す関数をサブスクライバーとして追加する．
    pub fn subscribe_pinned_fn<F, E>(&mut self, func: F)
    where
        F: for<'a> Fn(&'a E) -> Pin<Box<dyn Future<Output = O> + Send + 'a>>
            + Send
            + Sync
            + 'static,
        E: Event,
    {
        self.subscribe(crate::subscribe::AsyncFuncSubscriber::from_pinned_fn(func));
    }
    /// イベントをサブスクライバーに通知して非同期実行しハンドルを返す．
    pub fn dispatch_event<E: Event>(&self, event: E) -> Vec<Task<O>> {
        let mut tasks = Vec::new();
        let event = Arc::new(event);
        let event_type = E::event_type();

        if let Some(subscribers_any) = self.subscribers_map.get(&event_type) {
            let subscribers = subscribers_any.downcast_ref::<Subscribers<E, O>>().unwrap(); // ダウンキャスト結果がNoneとなるのはバグである

            for subscriber in subscribers.iter() {
                let subscriber = Arc::clone(subscriber);
                let event = Arc::clone(&event);

                let task =
                    async_global_executor::spawn(
                        async move { subscriber.handle_event(&event).await },
                    );

                tasks.push(task);
            }
        }
        tasks
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
