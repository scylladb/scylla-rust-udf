mod abi_exports;
mod from_wasmptr;
mod to_columntype;
mod to_wasmptr;
mod wasm_convertible;
mod wasmptr;

/// Not a part of public API. May change in minor releases.
/// Contains all the items used by the scylla_udf macros.
#[doc(hidden)]
pub mod _macro_internal {
    pub use crate::from_wasmptr::FromWasmPtr;
    pub use crate::to_columntype::ToColumnType;
    pub use crate::to_wasmptr::ToWasmPtr;
    pub use crate::wasm_convertible::WasmConvertible;
    pub use crate::wasmptr::WasmPtr;
    pub use scylla_cql::_macro_internal::*;
    pub use scylla_cql::frame::response::result::ColumnType;
}

/// This macro allows using a Rust function as a Scylla UDF.
///
/// The function must have arguments and return value of Rust types that can be mapped to CQL types,
/// the macro takes care of converting the arguments from CQL types to Rust types and back.
/// The function must not have the `#[no_mangle]` attribute, it will be added by the macro.
///
/// For example, for a function:
/// ```
/// #[scylla_udf::export_udf]
/// fn foo(arg: i32) -> i32 {
///    arg + 1
/// }
/// ```
/// you can use the compiled binary in its text format as a UDF in Scylla:
/// ```text
/// CREATE FUNCTION foo(arg int) RETURNS NULL ON NULL INPUT RETURNS int LANGUAGE rust AS '(module ...)`;
/// ```
pub use scylla_udf_macros::export_udf;

/// This macro allows mapping a Rust struct to a UDT from Scylla, and using in a scylla_udf function.
///
/// To use it, you need to define a struct with the same fields as the Scylla UDT.
/// For example, for a UDT defined as:
/// ```text
/// CREATE TYPE udt (
///     a int,
///     b double,
///     c text,
/// );
/// ```
/// you need to define a struct:
/// ```
/// #[scylla_udf::export_udt]
/// struct Udt {
///     a: i32,
///     b: f64,
///     c: String,
/// }
/// ```
pub use scylla_udf_macros::export_udt;

/// This macro allows (de)serializing a cql type to/from a Rust "newtype" struct.
///
/// The macro takes a "newtype" struct (tuple struct with only one field) and generates all implementations for (de)serialization
/// traits used in the scylla_udf macros by treating the struct as the inner type itself.
///
/// This allows overriding the impls for the inner type, while still being able to use it in the types of parameters or return
/// values of scylla_udf functions.
///
/// For example, for a function using a newtype struct:
/// ```
/// #[scylla_udf::export_newtype]
/// struct MyInt(i32);
///
/// #[scylla_udf::export_udf]
/// fn foo(arg: MyInt) -> MyInt {
///     ...
/// }
/// ```
/// and a table:
/// ```text
/// CREATE TABLE table (x int PRIMARY KEY);
/// ```
/// you can use the function in a query:
/// ```text
/// SELECT foo(x) FROM table;
/// ```
pub use scylla_udf_macros::export_newtype;

pub use scylla_cql::frame::value::{Counter, CqlDuration, Time, Timestamp};
