use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

// -------------------------------------------------------------------------------------------------
// DomainEvent

/// ドメインイベントが実装べきトレイト
pub trait DomainEvent:
    Serialize + DeserializeOwned + Clone + PartialEq + Debug + Sync + Send
{
    /// イベントのバージョンを取得
    fn event_version() -> String;

    /// イベントの名前を取得．
    fn event_type(&self) -> String;
}

// -------------------------------------------------------------------------------------------------
// EventList

/// DomainEventのリスト(Vec<E: DomainEvent>)の代用になるようにOptionを隠匿する．必ずSomeとなるようにする．
#[derive(Debug, Clone, PartialEq)]
pub struct DomainEventList<E: DomainEvent> {
    events_opt: Option<Vec<E>>,
}

impl<E: DomainEvent> Default for DomainEventList<E> {
    fn default() -> Self {
        Self {
            events_opt: Some(Vec::new()),
        }
    }
}

impl<E: DomainEvent> DomainEventList<E> {
    pub const fn new() -> Self {
        Self {
            events_opt: Some(Vec::new()),
        }
    }
    /// ドメインイベントを追加
    pub fn push(&mut self, event: E) {
        if let Some(events) = self.events_opt.as_mut() {
            events.push(event);
        } else {
            self.events_opt = Some(vec![event]);
        }
    }
    /// ドメインイベント全てを取得．
    pub fn take(&mut self) -> Vec<E> {
        let events = self.events_opt.take().unwrap(); // 必ずSomeであるため安全
        self.events_opt = Some(Vec::new()); // Someであることを保証
        events
    }
}
