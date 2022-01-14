#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_else_if
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
    pub fn writesep<T: ToString>(&mut self, v: &[T], sep: &str) {
        let s = v
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep);
        self.writeln(format!("{} ", &s));
    }
    pub fn writevec<T: ToString>(&mut self, v: &[T]) {
        self.writesep(v, " ")
    }
    pub fn writejoin<T: ToString>(&mut self, v: &[T]) {
        self.writesep(v, "")
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

pub fn solve_one(n: usize, len: usize, vec: Vec<Vec<u32>>) -> Option<i64> {
    let mut idx = 0;
    // let mut last = (0, 0);

    let target = vec.last().unwrap();

    let mut segs = vec![];
    segs.push((0usize, 0usize, 0usize));

    while idx < len {
        if let Some(&(left, right, p)) = segs.last() {
            if let Some(&v) = vec[p].get(right + 1) {
                if v == target[idx] {
                    // segs.push((left, right + 1, p));
                    segs.last_mut().unwrap().1 += 1;
                    continue;
                }
            }

            let mut found = None;
            for (t, number) in vec.iter().enumerate() {
                for i in 0..len - 1 {
                    if number[i] == target[i] && number[i + 1] == target[i + 1] {
                        found = Some((i, i + 1, t));
                        break;
                    }
                }
            }
            if let Some(seg) = found {
                segs.push(seg);
            }
        } else {
            // Start
        }
        idx += 1;
    }
    unimplemented!();
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.read();
        let len = sc.read();
        let vec = (0..n)
            .map(|_| {
                sc.chars()
                    .into_iter()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        let ans = solve_one(n, len, vec);
        sc.writeln(ans);
    }
}
