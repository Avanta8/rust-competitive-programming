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

pub fn solve_one(
    width: i64,
    height: i64,
    x0: Vec<i64>,
    xh: Vec<i64>,
    y0: Vec<i64>,
    yw: Vec<i64>,
) -> i64 {
    let mut points = vec![];
    for &x in x0.iter() {
        points.push((x, 0));
    }
    for &x in xh.iter() {
        points.push((x, height));
    }
    for &y in y0.iter() {
        points.push((0, y));
    }
    for &y in yw.iter() {
        points.push((width, y));
    }

    let mut best = 0;

    let d = x0.last().unwrap() - x0[0];
    for &(px, py) in points.iter() {
        best = max(best, d * py);
    }
    let d = xh.last().unwrap() - xh[0];
    for &(px, py) in points.iter() {
        best = max(best, d * (height - py));
    }
    let d = y0.last().unwrap() - y0[0];
    for &(px, py) in points.iter() {
        best = max(best, d * px);
    }
    let d = yw.last().unwrap() - yw[0];
    for &(px, py) in points.iter() {
        best = max(best, d * (width - px));
    }

    best
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let w = sc.read();
        let h = sc.read();
        let mut x0 = sc.vecn();
        let mut xh = sc.vecn();
        let mut y0 = sc.vecn();
        let mut yw = sc.vecn();
        x0.sort_unstable();
        xh.sort_unstable();
        y0.sort_unstable();
        yw.sort_unstable();
        let ans = solve_one(w, h, x0, xh, y0, yw);
        sc.writeln(ans);
    }
}
