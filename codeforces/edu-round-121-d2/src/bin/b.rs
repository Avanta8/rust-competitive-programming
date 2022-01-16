#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
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
        self.writeln(format!("{}", &s));
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

pub fn solve_one(x: Vec<char>) -> Vec<u32> {
    let x = x
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let len = x.len();

    for idx in (0..len - 1).rev() {
        let a = x[idx];
        let b = x[idx + 1];
        if a + b >= 10 {
            let mut nx = x;
            nx[idx] = 1;
            nx[idx + 1] = a + b - 10;
            return nx;
        }
    }

    let mut nx = x;
    let a = nx.remove(0);
    let b = nx.remove(0);
    nx.insert(0, a + b);

    nx
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let ans = solve_one(sc.chars());
        sc.writejoin(&ans);
    }
}
