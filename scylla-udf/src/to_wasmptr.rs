use crate::wasmptr::WasmPtr;
use core::convert::TryInto;
use scylla_cql::frame::value::Value;

pub trait ToWasmPtr {
    fn to_wasmptr(&self) -> WasmPtr;
}

impl<T: Value> ToWasmPtr for T {
    fn to_wasmptr(&self) -> WasmPtr {
        let mut bytes = Vec::<u8>::new();
        self.serialize(&mut bytes).expect("Error serializing value");
        let size = u32::from_be_bytes(bytes[..4].try_into().expect("slice with incorrect length"));
        if size == u32::MAX {
            return WasmPtr::null();
        }
        let mut dest = WasmPtr::with_size(size).expect("Failed to allocate memory");
        let dest_slice = dest
            .as_mut_slice()
            .expect("WasmPtr::as_mut_slice returned None");
        dest_slice.copy_from_slice(&bytes[4..]);
        dest
    }
}
