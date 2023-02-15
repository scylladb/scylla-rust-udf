use scylla_udf::*;
use std::collections::BTreeSet;

#[export_newtype]
struct StringLen(String);

impl std::cmp::PartialEq for StringLen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::Eq for StringLen {}

impl std::cmp::PartialOrd for StringLen {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for StringLen {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0.len().cmp(&other.0.len()) == std::cmp::Ordering::Equal {
            self.0.cmp(&other.0)
        } else {
            self.0.len().cmp(&other.0.len())
        }
    }
}

// Store the top N strings by length, without repetitions.
#[export_udf]
fn topn_row(
    acc_tup: Option<(i32, BTreeSet<StringLen>)>,
    v: Option<StringLen>,
) -> Option<(i32, BTreeSet<StringLen>)> {
    if let Some((n, mut acc)) = acc_tup {
        if let Some(v) = v {
            acc.insert(v);
            while acc.len() > n as usize {
                acc.pop_first();
            }
        }
        Some((n, acc))
    } else {
        None
    }
}

#[export_udf]
fn topn_reduce(
    (n1, mut acc1): (i32, BTreeSet<StringLen>),
    (n2, mut acc2): (i32, BTreeSet<StringLen>),
) -> (i32, BTreeSet<StringLen>) {
    assert!(n1 == n2);
    acc1.append(&mut acc2);
    while acc1.len() > n1 as usize {
        acc1.pop_first();
    }
    (n1, acc1)
}

#[export_udf]
fn topn_final((_, acc): (i32, BTreeSet<StringLen>)) -> BTreeSet<StringLen> {
    acc
}
