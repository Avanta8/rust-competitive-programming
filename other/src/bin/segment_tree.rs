/*
               1
        2              1
   2       3       1       3
 2   5   8   3   4   1   9   3
*/

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

        let mut idx = self.size - 1;
        self.vec
            .splice(idx..idx + items.len(), items.iter().copied().map(Some));

        while idx > 0 {
            let last_idx = idx;
            idx = ((idx + 1).next_power_of_two() >> 1) - 1;
            for i in idx..last_idx {
                self.vec[i] = self.op(self.vec[i * 2 + 1], self.vec[i * 2 + 2]);
            }
        }
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut idx = self.size + i - 1;
        self.vec[idx] = Some(value);

        while idx != 0 {
            let parent = (idx - 1) / 2;
            self.vec[parent] = self.op(self.vec[parent * 2 + 1], self.vec[parent * 2 + 2]);
            idx = parent;
        }
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

fn main() {
    // println!("{:?}", SegmentTree::new(&[3; 10], 0, std::cmp::max));
    // println!("{:?}", SegmentTree::new(&[0; 8], 0, std::cmp::max));
    // println!("{:?}", SegmentTree::new(&[0; 16], 0, std::cmp::max));
    // println!("{:?}", SegmentTree::new(&[0; 15], 0, std::cmp::max));
    // println!("{:?}", SegmentTree::new(&[0; 17], 0, std::cmp::max));

    let mut tree = SegmentTree::new(8, std::cmp::min);
    tree.fill(&(1..9).collect::<Vec<_>>());
    println!("{:?}", tree);

    println!("{}", tree.get(0..8).unwrap());
    println!("{}", tree.get(3..6).unwrap());

    let mut tree = SegmentTree::new(22, std::cmp::max);
    // let mut tree = SegmentTree::new(22, |a: i64, b: i64| a + b);
    tree.fill(&(1..23).collect::<Vec<_>>());
    println!("{:?}", tree);

    println!("{}", tree.get(0..8).unwrap());
    println!("{}", tree.get(3..6).unwrap());
    println!("{}", tree.get(4..22).unwrap());
    println!("{}", tree.get(18..21).unwrap());
    println!("{}", tree.get(8..17).unwrap());

    tree.update(3, 234);
    println!("{}", tree.get(0..8).unwrap());
    println!("{}", tree.get(3..6).unwrap());
    println!("{}", tree.get(4..22).unwrap());
    println!("{}", tree.get(18..21).unwrap());
    println!("{}", tree.get(8..17).unwrap());
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
