#![allow(unused_imports)]

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

fn solve_one(money: i64, people: &[i64]) -> Option<(usize, usize)> {
    let mut prefix = vec![0];
    for &p in people {
        let l = prefix[prefix.len() - 1];
        prefix.push(l + p);
    }
    // println!();
    // println!("{} {:?}", money, people);
    // println!("{:?}", prefix);

    let mut left = 0;
    let mut right = 1;

    let mut best_len = 0;
    let mut best_idx = None;

    while right < prefix.len() {
        let diff = prefix[right] - prefix[left];
        if diff >= -money {
            let len = right - left;
            if len > best_len {
                best_len = len;
                best_idx = Some((left + 1, right));
            }
            right += 1;
        } else {
            left += 1;
        }
    }

    best_idx
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let s = sc.read();
        let a = sc.vec(n);
        let ans = solve_one(s, &a);
        match ans {
            Some((a, b)) => sc.writeln(format!("{} {}", a, b)),
            None => sc.writeln(-1),
        }
    }
}
