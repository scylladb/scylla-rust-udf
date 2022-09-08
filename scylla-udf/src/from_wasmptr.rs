use crate::to_columntype::ToColumnType;
use crate::wasmptr::WasmPtr;
use scylla_cql::cql_to_rust::FromCqlVal;
use scylla_cql::frame::response::result::{deser_cql_value, CqlValue};

pub trait FromWasmPtr {
    fn from_wasmptr(wasmptr: WasmPtr) -> Self;
}

impl<T> FromWasmPtr for T
where
    T: FromCqlVal<Option<CqlValue>> + ToColumnType,
{
    fn from_wasmptr(wasmptr: WasmPtr) -> Self {
        if wasmptr.is_null() {
            return T::from_cql(None).unwrap();
        }
        let mut slice = wasmptr.as_slice().expect("WasmPtr::as_slice returned None");
        T::from_cql(Some(
            deser_cql_value(&T::to_column_type(), &mut slice).unwrap(),
        ))
        .unwrap()
    }
}
