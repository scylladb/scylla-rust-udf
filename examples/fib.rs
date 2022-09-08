use scylla_udf::*;

#[export_newtype]
struct FibInputNumber(i32);

#[export_newtype]
struct FibReturnNumber(i64);

#[export_udf]
fn fib(i: FibInputNumber) -> FibReturnNumber {
    FibReturnNumber(if i.0 <= 2 {
        1
    } else {
        fib(FibInputNumber(i.0 - 1)).0 + fib(FibInputNumber(i.0 - 2)).0
    })
}
