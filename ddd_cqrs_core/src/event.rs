// -------------------------------------------------------------------------------------------------
// EventList

use std::fmt::Debug;

/// DomainEventのリスト(Vec<E>)の代用になるようにOptionを隠匿する．必ずSomeとなるようにする．

#[derive(Clone, PartialEq)]
pub struct DomainEventList<E> {
    events_opt: Option<Vec<E>>,
}

impl<E> Default for DomainEventList<E> {
    fn default() -> Self {
        Self {
            events_opt: Some(Vec::new()),
        }
    }
}

impl<E: Debug> Debug for DomainEventList<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.events_opt.as_ref() {
            Some(events) => f.debug_tuple("DomainEventList").field(events).finish(),
            None => f
                .debug_tuple("DomainEventList")
                .field(&Vec::<E>::new())
                .finish(),
        }
    }
}

impl<E> DomainEventList<E> {
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
