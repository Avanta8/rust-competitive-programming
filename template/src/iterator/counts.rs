use std::collections::HashMap;

pub fn count_slice<T: Eq + std::hash::Hash>(v: &[T]) -> HashMap<&T, usize> {
    let mut counts = HashMap::new();

    for item in v {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let v = [
            1, 1, 1, 2, 2, 1, 1, 2, 1, 3, 3, 3, 3, 3, 3, 3, 2, 3, 2, 2, 2, 1, 1, 1,
        ];

        for (item, count) in count_slice(&v) {
            assert_eq!(count, v.iter().filter(|&x| x == item).count());
        }
    }
}
