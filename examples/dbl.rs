use scylla_udf::export_udf;

#[export_udf]
fn dbl(s: String) -> String {
    let mut newstr = String::new();
    newstr.push_str(&s);
    newstr.push_str(&s);
    newstr
}
