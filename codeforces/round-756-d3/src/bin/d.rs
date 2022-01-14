#![allow(dead_code, unused_imports, unused_variables)]

use std::cmp::*;
use std::collections::*;

struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

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

fn solve_one(len: usize, b: &[usize], p: &[usize]) -> Vec<i64> {
    let mut root = 0;
    for (i, &v) in b.iter().enumerate() {
        if v == i + 1 {
            root = v;
        }
    }

    // dbg!(&root);

    if p[0] != root {
        // println!("0 not root");
        return [-1].into();
    }

    let mut dists = vec![None; len];
    dists[root - 1] = Some(0);
    let mut next_dist = 1;

    let mut ans = vec![0; len];

    for &v in p[1..].iter() {
        // dbg!(&dists, &ans);
        let parent = b[v - 1];
        // dbg!(v, parent);
        if let Some(pdist) = dists[parent - 1] {
            let diff = next_dist - pdist;
            dists[v - 1] = Some(next_dist);
            ans[v - 1] = diff;
        } else {
            // println!("2 out ret, v:{}", v);
            return [-1].into();
        }
        next_dist += 1;
    }

    ans
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let b = sc.vec(n);
        let p = sc.vec(n);
        let ans = solve_one(n, &b, &p);
        sc.writevec(&ans);
    }
}
