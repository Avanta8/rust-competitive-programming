pub fn log2_ceil(n: i64) -> i64 {
    (i64::BITS - n.leading_zeros()) as i64 - if n.count_ones() > 1 { 0 } else { 1 }
}
