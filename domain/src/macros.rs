/// バリアントとと同じ名前の型からのFromトレイトを列挙体に実装する．
#[macro_export]
macro_rules! generate_enum_from {
    ($enum_ty:ty, $( $variant_ty:ident ),*) => {
        $(
            impl From<$variant_ty> for $enum_ty {
                fn from(value: $variant_ty) -> Self {
                    <$enum_ty>::$variant_ty(value)
                }
            }
        )*

    };
}
