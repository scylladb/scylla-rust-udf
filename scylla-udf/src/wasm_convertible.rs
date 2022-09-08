use crate::from_wasmptr::FromWasmPtr;
use crate::to_wasmptr::ToWasmPtr;
use crate::wasmptr::WasmPtr;
use scylla_cql::frame::value::{Counter, CqlDuration, Time, Timestamp};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::convert::TryFrom;

pub trait WasmConvertible {
    type WasmType;
    fn from_wasm(arg: Self::WasmType) -> Self;
    fn to_wasm(&self) -> Self::WasmType;
}

// This macro implements WasmConvertible given a Rust type and the resulting WasmType
macro_rules! impl_wasm_convertible_native {
    ($rust_type:ty) => {
        impl WasmConvertible for $rust_type {
            type WasmType = $rust_type;
            fn from_wasm(arg: Self::WasmType) -> Self {
                arg
            }
            fn to_wasm(&self) -> Self::WasmType {
                *self
            }
        }
    };
}

impl_wasm_convertible_native!(i32);
impl_wasm_convertible_native!(i64);
impl_wasm_convertible_native!(f32);
impl_wasm_convertible_native!(f64);

// This macro implements WasmConvertible given a Rust type that can be converted to the given WasmType using TryFrom
macro_rules! impl_wasm_convertible_scalar {
    ($rust_type:ty, $scalar_type:ty) => {
        impl WasmConvertible for $rust_type {
            type WasmType = $scalar_type;
            fn from_wasm(arg: Self::WasmType) -> Self {
                <$rust_type>::try_from(arg)
                    .expect("Failed to convert from wasm type to a rust type")
            }
            fn to_wasm(&self) -> Self::WasmType {
                *self as Self::WasmType
            }
        }
    };
}

impl_wasm_convertible_scalar!(i8, i32);
impl_wasm_convertible_scalar!(i16, i32);

// Can't convert bool to i32 using TryFrom, so we need a special implementation
impl WasmConvertible for bool {
    type WasmType = i32;
    fn from_wasm(arg: Self::WasmType) -> Self {
        arg != 0
    }
    fn to_wasm(&self) -> Self::WasmType {
        i32::from(*self)
    }
}

// This macro implements WasmConvertible given a Rust type that can be (de)serialized using FromWasmPtr and ToWasmPtr
macro_rules! impl_wasm_convertible_serialized {
    ($rust_type:ty) => {
        impl WasmConvertible for $rust_type {
            type WasmType = WasmPtr;
            fn from_wasm(arg: Self::WasmType) -> Self {
                <Self as FromWasmPtr>::from_wasmptr(arg)
            }
            fn to_wasm(&self) -> Self::WasmType {
                <Self as ToWasmPtr>::to_wasmptr(self)
            }
        }
    };
}

impl_wasm_convertible_serialized!(Counter);
impl_wasm_convertible_serialized!(chrono::NaiveDate);
impl_wasm_convertible_serialized!(bigdecimal::BigDecimal);
impl_wasm_convertible_serialized!(CqlDuration);
impl_wasm_convertible_serialized!(String);
impl_wasm_convertible_serialized!(Timestamp);
impl_wasm_convertible_serialized!(std::net::IpAddr);
impl_wasm_convertible_serialized!(Time);
impl_wasm_convertible_serialized!(uuid::Uuid);
impl_wasm_convertible_serialized!(num_bigint::BigInt);

// This macro implements WasmConvertible given a Rust type with a generic parameter T that can be (de)serialized using FromWasmPtr and ToWasmPtr
macro_rules! impl_wasm_convertible_serialized_generic {
    ($rust_type:ty) => {
        impl<T> WasmConvertible for $rust_type
        where
            $rust_type: FromWasmPtr + ToWasmPtr,
        {
            type WasmType = WasmPtr;
            fn from_wasm(arg: Self::WasmType) -> Self {
                <Self as FromWasmPtr>::from_wasmptr(arg)
            }
            fn to_wasm(&self) -> Self::WasmType {
                <Self as ToWasmPtr>::to_wasmptr(self)
            }
        }
    };
}

impl_wasm_convertible_serialized_generic!(Option<T>);
// Implements both lists and blobs
impl_wasm_convertible_serialized_generic!(Vec<T>);
impl_wasm_convertible_serialized_generic!(BTreeSet<T>);
impl_wasm_convertible_serialized_generic!(HashSet<T>);

// This macro implements WasmConvertible given a Rust type with generic parameters K and V that can be (de)serialized using FromWasmPtr and ToWasmPtr
macro_rules! impl_wasm_convertible_serialized_double_generic {
    ($rust_type:ty) => {
        impl<K, V> WasmConvertible for $rust_type
        where
            $rust_type: FromWasmPtr + ToWasmPtr,
        {
            type WasmType = WasmPtr;
            fn from_wasm(arg: Self::WasmType) -> Self {
                <Self as FromWasmPtr>::from_wasmptr(arg)
            }
            fn to_wasm(&self) -> Self::WasmType {
                <Self as ToWasmPtr>::to_wasmptr(self)
            }
        }
    };
}

impl_wasm_convertible_serialized_double_generic!(BTreeMap<K, V>);
impl_wasm_convertible_serialized_double_generic!(HashMap<K, V>);

// This macro implements WasmConvertible for tuples of types that can be (de)serialized using FromWasmPtr and ToWasmPtr
macro_rules! impl_wasm_convertible_serialized_tuple {
    ( $( $types:ident )* ) => {
        impl<$($types),*> WasmConvertible for ($($types,)*)
        where
            ($($types,)*): FromWasmPtr + ToWasmPtr
        {
            type WasmType = WasmPtr;
            fn from_wasm(arg: Self::WasmType) -> Self {
                <Self as FromWasmPtr>::from_wasmptr(arg)
            }
            fn to_wasm(&self) -> Self::WasmType {
                <Self as ToWasmPtr>::to_wasmptr(self)
            }
        }
    };
}

impl_wasm_convertible_serialized_tuple! { A }
impl_wasm_convertible_serialized_tuple! { A B }
impl_wasm_convertible_serialized_tuple! { A B C }
impl_wasm_convertible_serialized_tuple! { A B C D }
impl_wasm_convertible_serialized_tuple! { A B C D E }
impl_wasm_convertible_serialized_tuple! { A B C D E F }
impl_wasm_convertible_serialized_tuple! { A B C D E F G }
impl_wasm_convertible_serialized_tuple! { A B C D E F G H }
impl_wasm_convertible_serialized_tuple! { A B C D E F G H I }
impl_wasm_convertible_serialized_tuple! { A B C D E F G H I J }
impl_wasm_convertible_serialized_tuple! { A B C D E F G H I J K }
impl_wasm_convertible_serialized_tuple! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::WasmConvertible;
    use crate::*;

    #[test]
    fn i32_convert() {
        assert_eq!(i32::from_wasm(42_i32.to_wasm()), 42_i32);
        assert_eq!(i32::from_wasm(-42_i32), -42_i32);
        assert_eq!((-42_i32).to_wasm(), -42_i32);
    }
    #[test]
    fn i64_convert() {
        assert_eq!(i64::from_wasm(42_i64.to_wasm()), 42_i64);
        assert_eq!(i64::from_wasm(-42_i64), -42_i64);
        assert_eq!((-42_i64).to_wasm(), -42_i64);
    }
    #[test]
    fn f32_convert() {
        assert_eq!(f32::from_wasm(0.42_f32.to_wasm()), 0.42_f32);
        assert_eq!(f32::from_wasm(-0.42_f32), -0.42_f32);
        assert_eq!((-0.42_f32).to_wasm(), -0.42_f32);
    }
    #[test]
    fn f64_convert() {
        assert_eq!(f64::from_wasm(0.42_f64.to_wasm()), 0.42_f64);
        assert_eq!(f64::from_wasm(-0.42_f64), -0.42_f64);
        assert_eq!((-0.42_f64).to_wasm(), -0.42_f64);
    }
    #[test]
    fn i8_convert() {
        assert_eq!(i8::from_wasm(42_i8.to_wasm()), 42_i8);
        assert_eq!(i8::from_wasm(-42_i32), -42_i8);
        assert_eq!((-42_i8).to_wasm(), -42_i32);
    }
    #[test]
    fn i16_convert() {
        assert_eq!(i64::from_wasm(42_i64.to_wasm()), 42_i64);
        assert_eq!(i64::from_wasm(-42_i64), -42_i64);
        assert_eq!((-42_i64).to_wasm(), -42_i64);
    }
    #[test]
    fn bool_convert() {
        assert!(bool::from_wasm(true.to_wasm()));
        assert!(bool::from_wasm(1_i32));
        assert_eq!(bool::to_wasm(&false), 0_i32);
    }
    #[test]
    fn blob_convert() {
        let blob: Vec<u8> = vec![1, 2, 3, 4, 5];
        assert_eq!(Vec::<u8>::from_wasm(blob.to_wasm()), blob);
    }
    #[test]
    fn counter_convert() {
        assert_eq!(Counter::from_wasm(Counter(13).to_wasm()), Counter(13));
    }
    #[test]
    fn naive_date_convert() {
        use chrono::NaiveDate;
        let date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        assert_eq!(NaiveDate::from_wasm(date.to_wasm()), date);
    }
    #[test]
    fn big_decimal_convert() {
        use bigdecimal::BigDecimal;
        use std::str::FromStr;
        let bigd = BigDecimal::from_str("547318970434573134570").unwrap();
        assert_eq!(BigDecimal::from_wasm(bigd.to_wasm()), bigd);
    }
    #[test]
    fn cql_duration_convert() {
        let dur = CqlDuration {
            months: 1,
            days: 2,
            nanoseconds: 3,
        };
        assert_eq!(CqlDuration::from_wasm(dur.to_wasm()), dur);
    }
    #[test]
    fn timestamp_convert() {
        use chrono::Duration;
        let ts = Timestamp(Duration::weeks(2));
        assert_eq!(Timestamp::from_wasm(ts.to_wasm()), ts);
    }
    #[test]
    fn string_convert() {
        let s = String::from("abc");
        assert_eq!(String::from_wasm(s.to_wasm()), s);
    }
    #[test]
    fn inet_convert() {
        use std::net::IpAddr;
        let ip = IpAddr::from([127, 0, 0, 1]);
        assert_eq!(IpAddr::from_wasm(ip.to_wasm()), ip);
    }
    #[test]
    fn time_convert() {
        use chrono::Duration;
        let t = Time(Duration::hours(3));
        assert_eq!(Time::from_wasm(t.to_wasm()), t);
    }
    #[test]
    fn uuid_convert() {
        use uuid::Uuid;
        let uuid = Uuid::NAMESPACE_OID;
        assert_eq!(Uuid::from_wasm(uuid.to_wasm()), uuid);
    }
    #[test]
    fn big_int_convert() {
        use num_bigint::BigInt;
        use std::str::FromStr;
        let bi = BigInt::from_str("420000000000000000").unwrap();
        assert_eq!(BigInt::from_wasm(bi.to_wasm()), bi);
    }

    #[test]
    fn vec_convert() {
        // convert vec of strings
        let vec = vec![String::from("a"), String::from("b")];
        assert_eq!(Vec::<String>::from_wasm(vec.to_wasm()), vec);
    }
    #[test]
    fn hashset_convert() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(String::from("a"));
        set.insert(String::from("b"));
        assert_eq!(HashSet::<String>::from_wasm(set.to_wasm()), set);
    }
    #[test]
    fn btreeset_convert() {
        use std::collections::BTreeSet;
        let mut set = BTreeSet::new();
        set.insert((1, String::from("a")));
        set.insert((3, String::from("b")));
        assert_eq!(BTreeSet::<(i32, String)>::from_wasm(set.to_wasm()), set);
    }
    #[test]
    fn hashmap_convert() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(String::from("a"), 5_i16);
        map.insert(String::from("b"), 55_i16);
        assert_eq!(HashMap::<String, i16>::from_wasm(map.to_wasm()), map);
    }
    #[test]
    fn btreemap_convert() {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert((1, 2), String::from("a"));
        map.insert((3, 4), String::from("b"));
        assert_eq!(
            BTreeMap::<(i32, i32), String>::from_wasm(map.to_wasm()),
            map
        );
    }
    #[test]
    fn tuple_convert() {
        let tup = (String::from("a"), 5_i8);
        assert_eq!(<(String, i8)>::from_wasm(tup.to_wasm()), tup);
    }
}
