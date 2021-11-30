pub fn pow(mut a: i64, mut b: i64) -> i64 {
    let mut r = 1;
    loop {
        if b & 1 == 1 {
            r *= a;
        }
        b >>= 1;
        if b == 0 {
            break;
        }
        a *= a;
    }
    r
}
