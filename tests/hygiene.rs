#![no_implicit_prelude]

extern crate scylla_udf as _scylla_udf;

#[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::std::marker::Copy, ::std::clone::Clone)]
#[::_scylla_udf::export_udt(crate = "_scylla_udf")]
struct TestStruct {
    a: ::core::primitive::i32,
}
#[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::std::marker::Copy, ::std::clone::Clone)]
#[::_scylla_udf::export_newtype(crate = "_scylla_udf")]
struct TestNewtype(::core::primitive::i32);

// Macro can only be expanded if TestStruct and TestNewtype were
// properly expanded.
#[::_scylla_udf::export_udf(crate = "_scylla_udf")]
fn test_fn(arg1: TestNewtype, arg2: TestStruct) -> (TestNewtype, TestStruct) {
    (arg1, arg2)
}

#[test]
fn test_renamed() {
    use ::_scylla_udf::_macro_internal::WasmConvertible;
    let arg1 = TestNewtype(16);
    let arg2 = TestStruct { a: 16 };
    let rets = _scylla_internal_test_fn(arg1.to_wasm(), arg2.to_wasm());
    let (ret1, ret2) = <(TestNewtype, TestStruct)>::from_wasm(rets);
    ::std::assert_eq!(arg1, ret1);
    ::std::assert_eq!(arg2, ret2);
}
