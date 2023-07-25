use uuid::Uuid;

/// Idとして同定可能なことを示すトレイト
pub trait Identity {
    /// Uuidに変換可能な型
    type IntoId: Into<Uuid>;
    /// idとして何かしらの値を取得
    fn id(&self) -> Self::IntoId;
    /// uuidを取得
    fn uuid(&self) -> Uuid {
        self.id().into()
    }
}
