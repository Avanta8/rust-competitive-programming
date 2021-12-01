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

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let (x1, p1) = (sc.i64(), sc.usize());
        let (x2, p2) = (sc.i64(), sc.usize());

        let d1 = x1.to_string().len() + p1;
        let d2 = x2.to_string().len() + p2;

        if d1 > d2 {
            sc.writeln(">");
        } else if d2 > d1 {
            sc.writeln("<");
        } else {
            let s1 = x1.to_string().chars().collect::<Vec<_>>();
            let s2 = x2.to_string().chars().collect::<Vec<_>>();
            let mut f = false;
            for i in 0..min(s1.len(), s2.len()) {
                if s1[i] > s2[i] {
                    sc.writeln(">");
                    f = true;
                    break;
                } else if s1[i] < s2[i] {
                    sc.writeln("<");
                    f = true;
                    break;
                }
            }
            if f {
                // sc.writeln("=0");
            } else {
                let m = min(s1.len(), s2.len());
                let v1 = &s1[m..];
                let v2 = &s2[m..];
                if v1.is_empty() {
                    if v2.iter().any(|&c| c != '0') {
                        sc.writeln("<");
                    } else {
                        sc.writeln("=");
                    }
                } else if v2.is_empty() {
                    if v1.iter().any(|&c| c != '0') {
                        sc.writeln(">");
                    } else {
                        sc.writeln("=");
                    }
                } else {
                    sc.writeln("=");
                }
            }
        }
    }
}
