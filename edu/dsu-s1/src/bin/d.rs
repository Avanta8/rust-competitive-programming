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
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.get(a) == self.get(b)
    }

    pub fn get(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            return v;
        }

        self.parent[v] = self.get(self.parent[v]);
        self.parent[v]
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let mut sa = self.get(a);
        let mut sb = self.get(b);
        if sa == sb {
            return;
        }

        if self.size[sa] < self.size[sb] {
            std::mem::swap(&mut sa, &mut sb);
        }
        self.parent[sb] = sa;
        self.size[sa] += self.size[sb];
    }
}

#[allow(clippy::needless_collect)]
pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let n = sc.read();
    let m = sc.read();
    let k = sc.read();

    for _ in 0..m {
        sc.usize0();
        sc.usize0();
    }

    let op = (0..k)
        .map(|_| (sc.read::<String>(), sc.usize0(), sc.usize0()))
        .collect::<Vec<_>>();

    let mut dsu = Dsu::new(n);

    let mut ans = vec![];
    for (s, a, b) in op.into_iter().rev() {
        match s.as_str() {
            "ask" => ans.push(dsu.is_same(a, b)),
            "cut" => dsu.unite(a, b),
            _ => unreachable!(),
        }
    }

    for a in ans.into_iter().rev() {
        sc.writeln(if a { "YES" } else { "NO" });
    }
}
