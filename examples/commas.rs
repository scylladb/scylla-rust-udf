use scylla_udf::export_udf;

#[export_udf]
fn commas(strings: Option<Vec<String>>) -> Option<String> {
    strings.map(|strings| strings.join(", "))
}
