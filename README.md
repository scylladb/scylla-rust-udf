# Rust helper library for Scylla UDFs

This crate allows writing pure Rust functions that can be used as Scylla UDFs.

**Note: this crate is officially supported and ready to use. However, UDFs are still an experimental feature in ScyllaDB, and the crate has not been widely used, which is why it's still in beta and its API is subject to change. We appreciate bug reports and pull requests!**

## Usage

### Prerequisites

To use this helper library in Scylla you'll need:
* Standard library for Rust `wasm32-wasi`
  * Can be added in rustup installations using `rustup target add wasm32-wasi`
  * For non rustup setups, you can try following the steps at https://rustwasm.github.io/docs/wasm-pack/prerequisites/non-rustup-setups.html
  * Also available as an rpm: `rust-std-static-wasm32-wasi`
* `wasm2wat` parser
  * Available in many distributions in the `wabt` package

### Compilation

We recommend a setup with cargo.

1. Start with a library package
```
cargo new --lib
```
2. Add the following lines to the Cargo.toml to set the crate-type to cdylib
```
[lib]
crate-type = ["cdylib"]
```
3. Implement your package, exporting Scylla UDFs using the `scylla_udf::export_udf` macro.
4. Build the package using the wasm32-wasi target:
```
RUSTFLAGS="-C link-args=-zstack-size=131072" cargo build --target=wasm32-wasi
```
> **_NOTE:_** The default size of the stack in WASI (1MB) causes warnings about oversized allocations in Scylla, so we recommend setting the stack size to a lower value. This is done using the `RUSTFLAGS` environmental variable in the command above for a new size of 128KB, which should be enough for most use cases.

5. Find the compiled `.wasm` binary. Let's assume it's `target/wasm32-wasi/debug/abc.wasm`.
6. (optional) Optimize the binary using `wasm-opt -O3 target/wasm32-wasi/debug/abc.wasm` (can be combined with using `cargo build --release`  profile)
7. Translate the binary into `wat`:
```
wasm2wat target/wasm32-wasi/debug/abc.wasm > target/wasm32/wasi/debug/abc.wat
```

### CQL Statement

The resulting `target/wasm32/wasi/debug/abc.wat` code can now be used directly in a `CREATE FUNCTION` statement. The resulting code will most likely
contain `'` characters, so it may be necessary to first replace them with `''`, so that they're usable in a CQL string.

For example, if you have an [Rust UDF](examples/commas.rs) that joins a list of words using commas, you can create a Scylla UDF using the following statement:
```
CREATE FUNCTION commas(string list<text>) CALLED ON NULL INPUT RETURNS text AS ' (module ...) '
```


## CQL Type Mapping

The argument and return value types used in functions annotated with `#[export_udf]` must all map to CQL types used in the `CREATE FUNCTION` statements used in Scylla, according to the tables below.

If the Scylla function is created with types that do not match the types used in the Rust function, calling the UDF will fail or produce arbitrary results.

### Native types

| CQL Type  | Rust type                     |
| --------- | ----------------------------- |
| ASCII     | String                        |
| BIGINT    | i64                           |
| BLOB      | Vec\<u8\>                     |
| BOOLEAN   | bool                          |
| COUNTER   | scylla_udf::Counter           |
| DATE      | chrono::NaiveDate             |
| DECIMAL   | bigdecimal::Decimal           |
| DOUBLE    | f64                           |
| DURATION  | scylla_udf::CqlDuration       |
| FLOAT     | f32                           |
| INET      | std::net::IpAddr              |
| INT       | i32                           |
| SMALLINT  | i16                           |
| TEXT      | String                        |
| TIME      | scylla_udf::Time              |
| TIMESTAMP | scylla_udf::Timestamp         |
| TIMEUUID  | uuid::Uuid                    |
| TINYINT   | i8                            |
| UUID      | uuid::Uuid                    |
| VARCHAR   | String                        |
| VARINT    | num_bigint::BigInt            |

### Collections

If a CQL type `T` maps to Rust type `RustT`, you can use it as a collection parameter:

| CQL Type   | Rust type                                                                             |
| ---------- | ------------------------------------------------------------------------------------- |
| LIST\<T\>  | Vec\<RustT\>                                                                          |
| MAP\<T\>   | std::collections::BTreeMap\<RustT\>, std::collections::HashMap\<RustT\>               |
| SET\<T\>   | Vec\<RustT\>, std::collections::BTreeSet\<RustT\>, std::collections::HashSet\<RustT\> |


### Tuples

If CQL types `T1`, `T2`, ... map to Rust types `RustT1`, `RustT2`, ..., you can use them in tuples:

| CQL Type | Rust type                          |
| -------- | ---------------------------------- |
| TUPLE\<T1, T2, ...\>  | (RustT1, RustT2, ...) |

### Nulls

If a CQL Value of type T that's mapped to type RustT may be a null (all parameter and return types in `CALLED ON NULL INPUT` UDFs), then the type used in the Rust function should be Option\<RustT\>.

## Contributing

In general, try to follow the same rules as in https://github.com/scylladb/scylla-rust-driver/blob/main/CONTRIBUTING.md

### Testing

This crate is meant to be compiled to a `wasm32-wasi` target and ran in a WASM runtime. The tests that use WASM-specific code will most likely not succeed when executed in a different way (in particular, with a simple `cargo test` command).

For example, if you have the [wasmtime](https://docs.wasmtime.dev/cli-install.html) runtime installed and in `PATH`, you can use the following command to run tests:
```text
CARGO_TARGET_WASM32_WASI_RUNNER="wasmtime --allow-unknown-exports" cargo test --target=wasm32-wasi
```
