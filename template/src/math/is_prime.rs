pub fn is_prime(n: i64) -> bool {
    if n < 2 { return false }
    if n % 2 == 0 || n % 3 == 0 {
        return n < 4;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
    true
}
