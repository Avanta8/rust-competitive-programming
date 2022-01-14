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

pub fn solve_one(mut a: Vec<i8>, mut s: Vec<i8>) -> i64 {
    // println!("{:?}", a);
    // println!("{:?}", s);

    a.reverse();
    s.reverse();

    let mut it_s = s.iter().copied().peekable();

    let mut b = vec![];

    for digit in a {
        // let mut m = it_s.next().unwrap();
        let mut m = match it_s.next() {
            Some(v) => v,
            None => return -1,
        };
        if m < digit {
            m += 10;
            if it_s.next().unwrap_or_default() != 1 {
                return -1;
            }
        }
        b.push(m - digit);
    }

    b.extend(it_s);

    b.reverse();
    b.into_iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let a = sc.chars();
        let s = sc.chars();
        let ans = solve_one(
            a.into_iter()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect(),
            s.into_iter()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect(),
        );
        sc.writeln(ans);
    }
}
