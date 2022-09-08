use scylla_udf::export_udf;

#[export_udf]
fn keys(map: std::collections::BTreeMap<String, String>) -> Vec<String> {
    map.into_keys().collect()
}
