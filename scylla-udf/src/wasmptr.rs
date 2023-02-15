use crate::abi_exports::{_scylla_free, _scylla_malloc};

// A unique pointer to an object in the WASM memory.
// Contains the serialized size of the object in the high 32 bits, and the pointer
// to the object in the low 32 bits. A null pointer is represented by a size of u32::MAX.
// The pointer is allocated with _scylla_malloc and freed with _scylla_free.
#[repr(transparent)]
pub struct WasmPtr(u64);

impl WasmPtr {
    pub fn with_size(size: u32) -> Option<WasmPtr> {
        if size == u32::MAX {
            // u32::MAX is reserved for null
            return None;
        }

        // SAFETY: the size fits in a u32, so it's valid to allocate that much memory
        // and we do not dereference the pointer if the allocation fails
        let ptr = unsafe { _scylla_malloc(size) };
        if ptr == 0 {
            return None;
        }
        Some(WasmPtr(((size as u64) << 32) + ptr as u64))
    }

    pub const fn size(&self) -> Option<usize> {
        let size = self.0 >> 32;
        if size == u32::MAX as u64 {
            None
        } else {
            Some(size as usize)
        }
    }

    pub const fn null() -> WasmPtr {
        WasmPtr((u32::MAX as u64) << 32)
    }

    pub const fn is_null(&self) -> bool {
        self.size().is_none()
    }

    fn raw(&self) -> *mut u8 {
        (self.0 & 0xffffffff) as *mut u8
    }

    fn raw_mut(&self) -> *mut u8 {
        (self.0 & 0xffffffff) as *mut u8
    }

    pub fn as_slice<'a>(&self) -> Option<&'a [u8]> {
        // SAFETY: the `dest` pointer is a succesful result of allocating `size` bytes and it's always aligned to a u8
        self.size()
            .map(|size| unsafe { std::slice::from_raw_parts::<'a>(self.raw(), size) })
    }

    pub fn as_mut_slice<'a>(&mut self) -> Option<&'a mut [u8]> {
        if let Some(size) = self.size() {
            // SAFETY: the `dest` pointer is a succesful result of allocating `size` bytes and it's always aligned to a u8
            Some(unsafe { std::slice::from_raw_parts_mut::<'a>(self.raw_mut(), size) })
        } else {
            None
        }
    }
}

impl Drop for WasmPtr {
    fn drop(&mut self) {
        if !self.is_null() {
            // SAFETY: the `dest` pointer is a succesful result of a _scylla_malloc call, so it's valid
            unsafe { _scylla_free(self.raw() as u32) };
        }
    }
}
