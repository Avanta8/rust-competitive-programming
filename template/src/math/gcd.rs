pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
