use scylla_udf::*;

#[export_udt]
struct Udt {
    a: i32,
    b: i32,
    c: String,
    d: String,
}

#[export_udf]
fn udt(arg: Udt) -> Udt {
    Udt {
        a: arg.b,
        b: arg.a,
        c: arg.d,
        d: arg.c,
    }
}
