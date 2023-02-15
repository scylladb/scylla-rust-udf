use scylla_udf::{export_udf, CqlDuration, Time, Timestamp};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
#[export_udf]
fn combine(
    b: bool,
    blob: Vec<u8>,
    date: chrono::NaiveDate,
    bd: bigdecimal::BigDecimal,
    dbl: f64,
    cqldur: CqlDuration,
    flt: f32,
    int32: i32,
    int64: i64,
    s: String,
    tstamp: Timestamp,
    ip: std::net::IpAddr,
    int16: i16,
    int8: i8,
    tim: Time,
    uid: uuid::Uuid,
    bi: num_bigint::BigInt,
) -> (
    (
        bool,
        Vec<u8>,
        chrono::NaiveDate,
        bigdecimal::BigDecimal,
        f64,
        CqlDuration,
        f32,
        i32,
        i64,
    ),
    (
        String,
        Timestamp,
        std::net::IpAddr,
        i16,
        i8,
        Time,
        uuid::Uuid,
        num_bigint::BigInt,
    ),
) {
    (
        (b, blob, date, bd, dbl, cqldur, flt, int32, int64),
        (s, tstamp, ip, int16, int8, tim, uid, bi),
    )
}
