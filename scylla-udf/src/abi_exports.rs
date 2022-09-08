extern "C" {
    fn malloc(size: u32) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// # Safety
/// - caller must ensure that the size is valid, if the allocation fails
/// - the caller must not dereference the returned pointer
#[no_mangle]
#[doc(hidden)]
pub(crate) unsafe extern "C" fn _scylla_malloc(size: u32) -> u32 {
    malloc(size) as u32
}

/// # Safety
/// - caller must ensure that the pointer is valid
#[no_mangle]
#[doc(hidden)]
pub(crate) unsafe extern "C" fn _scylla_free(ptr: u32) {
    free(ptr as *mut u8)
}

#[no_mangle]
#[doc(hidden)]
static _scylla_abi: u32 = 2;
