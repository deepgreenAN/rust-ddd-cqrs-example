use crate::DomainEventList;
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// Aggregate

/// アグリゲイトが実装すべきトレイト
pub trait Aggregate: Sync + Send + PartialEq {
    type Event;
    type IntoId: Into<Uuid> + Eq + Ord;
    /// idを取得
    fn id(&self) -> Self::IntoId;
    /// ドメインイベントを共有参照として取得
    fn domain_events(&self) -> &DomainEventList<Self::Event>;
    /// ドメインイベントを可変参照として取得
    fn domain_events_mut(&mut self) -> &mut DomainEventList<Self::Event>;
}
