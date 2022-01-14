#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain
)]

use std::cmp::*;
use std::collections::*;

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn writeln<S: ToString>(&mut self, s: S) {
        self.write(format!("{}\n", s.to_string()));
    }
    pub fn writevec<T: ToString>(&mut self, v: &[T]) {
        let s = v
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        self.writeln(format!("{} ", &s));
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn usize(&mut self) -> usize {
        self.read::<usize>()
    }
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn i32(&mut self) -> i32 {
        self.read::<i32>()
    }
    pub fn i64(&mut self) -> i64 {
        self.read::<i64>()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn vecn<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let n: usize = self.read();
        self.vec(n)
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

#[derive(Debug)]
pub struct SegmentTree<T> {
    vec: Vec<Option<(T, usize)>>,
    size: usize,
}

impl<T> SegmentTree<T>
where
    T: Copy + Ord,
{
    pub fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        let v = vec![None; size * 2 - 1];
        Self { vec: v, size }
    }

    pub fn get(&self, range: std::ops::Range<usize>) -> Option<(T, usize)> {
        self.get_branch(0, range, 0..self.size)
    }

    pub fn fill(&mut self, items: &[T]) {
        assert!(items.len() <= self.size);

        let idx = self.size - 1;
        self.vec.splice(
            idx..idx + items.len(),
            items.iter().copied().map(|x| Some((x, 1))),
        );

        (0..idx).rev().for_each(|i| self.update_single(i));
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut idx = self.size + i - 1;
        self.vec[idx] = Some((value, 1));

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
    ) -> Option<(T, usize)> {
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

    fn op(&self, a: Option<(T, usize)>, b: Option<(T, usize)>) -> Option<(T, usize)> {
        match (a, b) {
            (Some(a), Some(b)) => {
                if a.0 == b.0 {
                    Some((a.0, a.1 + b.1))
                } else if a.0 < b.0 {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            _ => a.or(b),
        }
    }
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let n = sc.read();
    let m = sc.read();

    let v = sc.vec::<i64>(n);
    let mut st = SegmentTree::new(n);
    st.fill(&v);

    for _ in 0..m {
        let q = sc.i32();
        match q {
            1 => {
                st.update(sc.usize(), sc.read());
            }
            2 => {
                let ans = st.get(sc.usize()..sc.usize()).unwrap();
                sc.writeln(format!("{} {}", ans.0, ans.1));
            }
            _ => unreachable!(),
        }
    }
}
