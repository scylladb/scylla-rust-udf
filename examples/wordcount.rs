use scylla_udf::export_udf;

#[export_udf]
fn wordcount(text: String) -> i32 {
    text.split(' ').count() as i32
}
