use scylla_udf::export_udf;

#[export_udf]
fn len(strings: std::collections::BTreeSet<String>) -> i16 {
    strings.len() as i16
}
