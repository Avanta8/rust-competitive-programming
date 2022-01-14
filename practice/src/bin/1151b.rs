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
use std::ops::ShlAssign;

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

// a bit must taken an odd number of times

pub fn solve_one(width: usize, height: usize, grid: Vec<Vec<u16>>) -> Option<Vec<usize>> {
    let mut bits_grid = vec![];

    for row in grid.iter() {
        let mut are = vec![];
        for shift in 0..10 {
            let mut avail = (None, None);
            for (i, &x) in row.iter().enumerate() {
                if x & (1 << shift) != 0 {
                    avail.0 = Some(i);
                } else {
                    avail.1 = Some(i);
                }
            }
            are.push(avail);
        }
        bits_grid.push(are);
    }

    for shift in 0usize..10 {
        let mut ans = vec![];
        let mut count = 0;
        let mut swap = None;
        for (row, there) in bits_grid.iter().enumerate() {
            let current = there[shift];
            if let Some(idx) = current.0 {
                count += 1;
                ans.push(idx);
                if let Some(s) = current.1 {
                    swap = Some((row, s));
                }
            } else {
                let idx = current.1.unwrap();
                ans.push(idx);
            }
        }

        if count % 2 == 1 {
            return Some(ans);
        } else {
            if let Some((row, idx)) = swap {
                ans[row] = idx;
                return Some(ans);
            }
        }
    }

    None
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let height = sc.read();
    let width = sc.read();
    let grid = (0..height).map(|_| sc.vec(width)).collect();
    let ans = solve_one(width, height, grid);
    if let Some(v) = ans {
        sc.writeln("TAK");
        sc.writevec(&v.into_iter().map(|i| i + 1).collect::<Vec<_>>());
    } else {
        sc.writeln("NIE");
    }
}
