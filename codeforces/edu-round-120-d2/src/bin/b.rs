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

pub fn solve_one(n: usize, p: Vec<i64>, s: Vec<bool>) -> Vec<i64> {
    // println!("{}, {:?}, {:?}", n, p, s);

    let mut disliked = (0..n).filter(|&i| !s[i]).collect::<Vec<_>>();
    disliked.sort_unstable_by(|&a, &b| p[a].cmp(&p[b]));

    let mut liked = (0..n).filter(|&i| s[i]).collect::<Vec<_>>();
    liked.sort_unstable_by(|&a, &b| p[a].cmp(&p[b]));

    // println!("disliked: {:?}", disliked);
    // println!("liked: {:?}", liked);

    let mut ns = disliked.into_iter().chain(liked);

    let mut ans = vec![0; n];
    for g in 1..=n as i64 {
        ans[ns.next().unwrap()] = g;
    }
    ans
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let p = sc.vec(n);
        let s = sc.chars().into_iter().map(|c| c == '1').collect();
        let ans = solve_one(n, p, s);
        sc.writevec(&ans);
    }
}
