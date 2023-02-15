pub use scylla_cql::frame::response::result::ColumnType;
use scylla_cql::frame::value::{Counter, CqlDuration, Time, Timestamp};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub trait ToColumnType {
    fn to_column_type() -> ColumnType;
}

// This macro implements ToColumnType given a Rust type and the resulting ColumnType
macro_rules! impl_to_col_type {
    ($rust_type:ty, $col_type:expr) => {
        impl ToColumnType for $rust_type {
            fn to_column_type() -> ColumnType {
                $col_type
            }
        }
    };
}

impl_to_col_type!(bool, ColumnType::Boolean);
impl_to_col_type!(Vec<u8>, ColumnType::Blob);
impl_to_col_type!(Counter, ColumnType::Counter);
impl_to_col_type!(chrono::NaiveDate, ColumnType::Date);
impl_to_col_type!(bigdecimal::BigDecimal, ColumnType::Decimal);
impl_to_col_type!(f64, ColumnType::Double);
impl_to_col_type!(CqlDuration, ColumnType::Duration);
impl_to_col_type!(f32, ColumnType::Float);
impl_to_col_type!(i32, ColumnType::Int);
impl_to_col_type!(i64, ColumnType::BigInt);
impl_to_col_type!(String, ColumnType::Text);
impl_to_col_type!(Timestamp, ColumnType::Timestamp);
impl_to_col_type!(std::net::IpAddr, ColumnType::Inet);
impl_to_col_type!(i16, ColumnType::SmallInt);
impl_to_col_type!(i8, ColumnType::TinyInt);
impl_to_col_type!(Time, ColumnType::Time);
impl_to_col_type!(uuid::Uuid, ColumnType::Uuid);
impl_to_col_type!(num_bigint::BigInt, ColumnType::Varint);

impl<T: ToColumnType> ToColumnType for Vec<T> {
    fn to_column_type() -> ColumnType {
        ColumnType::List(Box::new(T::to_column_type()))
    }
}

impl<K: ToColumnType, V: ToColumnType> ToColumnType for BTreeMap<K, V> {
    fn to_column_type() -> ColumnType {
        ColumnType::Map(Box::new(K::to_column_type()), Box::new(V::to_column_type()))
    }
}

impl<K: ToColumnType + Eq + std::hash::Hash, V: ToColumnType> ToColumnType for HashMap<K, V> {
    fn to_column_type() -> ColumnType {
        ColumnType::Map(Box::new(K::to_column_type()), Box::new(V::to_column_type()))
    }
}

impl<T: ToColumnType> ToColumnType for BTreeSet<T> {
    fn to_column_type() -> ColumnType {
        ColumnType::Set(Box::new(T::to_column_type()))
    }
}

impl<T: ToColumnType + Eq + std::hash::Hash> ToColumnType for HashSet<T> {
    fn to_column_type() -> ColumnType {
        ColumnType::Set(Box::new(T::to_column_type()))
    }
}

macro_rules! tuple_impls {
    ( $( $types:ident )* ) => {
        impl<$($types: ToColumnType),*> ToColumnType for ($($types,)*) {
            fn to_column_type() -> ColumnType {
                ColumnType::Tuple(vec![$($types::to_column_type()),*])
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }

impl<T: ToColumnType> ToColumnType for Option<T> {
    fn to_column_type() -> ColumnType {
        T::to_column_type()
    }
}
