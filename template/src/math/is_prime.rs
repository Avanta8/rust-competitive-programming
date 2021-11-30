pub fn is_prime(n: i64) -> bool {
    match n.cmp(&2) {
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => {
            if n % 2 == 0 {
                return false;
            }
            let mut d = 3;
            while d * d <= n {
                if n % d == 0 {
                    return false;
                }
                d += 2;
            }
            true
        }
    }
}
