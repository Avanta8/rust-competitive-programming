pub fn group_slice<T: PartialEq>(v: &[T]) -> Vec<(usize, &T)> {
    if v.is_empty() {
        return [].into();
    }

    let mut groups = vec![];

    let mut prev = &v[0];
    let mut count = 1;
    for item in v.iter().skip(1) {
        if item == prev {
            count += 1;
        } else {
            groups.push((count, prev));
            prev = item;
            count = 1;
        }
    }
    groups.push((count, prev));
    groups
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_i32() {
        let v = [
            1, 1, 1, 2, 2, 1, 1, 2, 1, 3, 3, 3, 3, 3, 3, 3, 2, 3, 2, 2, 2, 1, 1, 1,
        ];
        assert_eq!(
            group_slice(&v),
            vec![
                (3, &1),
                (2, &2),
                (2, &1),
                (1, &2),
                (1, &1),
                (7, &3),
                (1, &2),
                (1, &3),
                (3, &2),
                (3, &1)
            ]
        );
    }

    #[test]
    fn test_str() {
        let v = [
            "hello", "hello", "m", "m", "m", "m", "ar", "be", "be", "ar", "ar",
        ];
        assert_eq!(
            group_slice(&v),
            vec![(2, &"hello"), (4, &"m"), (1, &"ar"), (2, &"be"), (2, &"ar")]
        );
    }

    #[test]
    fn test_empty() {
        let v: [i32; 0] = [];
        assert_eq!(group_slice(&v), vec![]);
    }

    #[test]
    fn test_one() {
        let v = [5];
        assert_eq!(group_slice(&v), vec![(1, &5)]);
    }

    #[test]
    fn test_random() {
        use rand::thread_rng;

        let mut rng = thread_rng();

        for chance in [0., 0.1, 0.25, 0.5, 0.75, 0.9, 1.] {
            for len in (0..500).step_by(5) {
                let mut v = vec![];
                let mut last: i64 = rng.gen();
                for _ in 0..len {
                    if rng.gen_bool(chance) {
                        last = rng.gen();
                    }
                    v.push(last);
                }

                let groups = group_slice(&v);

                let mut rebuild = vec![];
                for (count, &value) in groups {
                    for _ in 0..count {
                        rebuild.push(value);
                    }
                }

                assert_eq!(v, rebuild);
            }
        }
    }
}
