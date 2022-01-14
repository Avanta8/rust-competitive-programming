#[derive(Debug)]
pub struct SegmentTree<T> {
    vec: Vec<Option<T>>,
    func: fn(T, T) -> T,
    size: usize,
}

impl<T> SegmentTree<T>
where
    T: Copy,
{
    pub fn new(size: usize, f: fn(T, T) -> T) -> Self {
        let size = size.next_power_of_two();
        let v = vec![None; size * 2 - 1];
        Self {
            vec: v,
            func: f,
            size,
        }
    }

    pub fn get(&self, range: std::ops::Range<usize>) -> Option<T> {
        self.get_branch(0, range, 0..self.size)
    }

    pub fn fill(&mut self, items: &[T]) {
        assert!(items.len() <= self.size);

        let idx = self.size - 1;
        self.vec
            .splice(idx..idx + items.len(), items.iter().copied().map(Some));

        (0..idx).rev().for_each(|i| self.update_single(i));
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut idx = self.size + i - 1;
        self.vec[idx] = Some(value);

        while idx != 0 {
            idx = (idx - 1) / 2;
            self.update_single(idx);
        }
    }

    fn update_single(&mut self, idx: usize) {
        self.vec[idx] = self.op(self.vec[idx * 2 + 1], self.vec[idx * 2 + 2]);
    }

    fn get_branch(
        &self,
        idx: usize,
        range: std::ops::Range<usize>,
        seg_range: std::ops::Range<usize>,
    ) -> Option<T> {
        if range.start >= seg_range.end || range.end <= seg_range.start {
            None
        } else if range.start <= seg_range.start && range.end >= seg_range.end {
            self.vec[idx]
        } else {
            let mid = (seg_range.start + seg_range.end) / 2;
            self.op(
                self.get_branch(2 * idx + 1, range.clone(), seg_range.start..mid),
                self.get_branch(2 * idx + 2, range, mid..seg_range.end),
            )
        }
    }

    fn op(&self, a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some((self.func)(a, b)),
            _ => a.or(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    const INF: i64 = 1 << 60;

    #[test]
    fn random_array() {
        const N: usize = 1000;
        let mut rng = thread_rng();

        for _ in 0..5 {
            let mut arr = vec![0; N];
            for i in 0..N {
                arr[i] = rng.gen_range(0..INF);
            }

            let mut seg = SegmentTree::new(N, |a: i64, b: i64| a.min(b));
            for i in 0..N {
                let mut minimum = INF;
                for j in 0..=i {
                    minimum = minimum.min(arr[j]);
                }
                seg.update(i, arr[i]);
                assert_eq!(seg.get(0..N), Some(minimum));
                assert_eq!(seg.get(0..(i + 1)), Some(minimum));
            }
        }
    }
}
