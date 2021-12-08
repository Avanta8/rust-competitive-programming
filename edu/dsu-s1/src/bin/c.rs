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
pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    xp: Vec<i64>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
            xp: vec![0; size],
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut sa = self.get(a).0;
        let mut sb = self.get(b).0;
        if sa == sb {
            return false;
        }

        if self.size[sa] < self.size[sb] {
            std::mem::swap(&mut sa, &mut sb);
        }

        self.xp[sb] -= self.xp[sa];
        self.parent[sb] = sa;

        true
    }

    fn add(&mut self, v: usize, x: i64) {
        let p = self.get(v).0;
        self.xp[p] += x;
    }

    fn get(&mut self, v: usize) -> (usize, i64) {
        if v == self.parent[v] {
            return (v, self.xp[v]);
        }

        let (p, uxp) = self.get(self.parent[v]);

        if self.parent[v] != p {
            self.parent[v] = p;
            self.xp[v] += uxp - self.xp[p];
        }

        (p, self.xp[p] + self.xp[v])
    }
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let n = sc.read();
    let m = sc.read();

    let mut dsu = Dsu::new(n);

    for _ in 0..m {
        match sc.read::<String>().as_str() {
            "add" => {
                dsu.add(sc.usize0(), sc.read());
            }
            "join" => {
                dsu.unite(sc.usize0(), sc.usize0());
            }
            "get" => {
                let v = sc.usize0();
                let x = dsu.get(v).1;
                sc.writeln(x);
            }
            _ => unreachable!(),
        }
    }
}
