use scylla_udf::export_udf;

type SmallInt = i16;

#[export_udf]
fn add(i1: SmallInt, i2: SmallInt) -> SmallInt {
    i1 + i2
}
