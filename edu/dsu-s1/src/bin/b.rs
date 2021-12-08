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

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    min: Vec<usize>,
    max: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        let p = (0..size).collect::<Vec<_>>();
        Self {
            parent: p.clone(),
            size: vec![1; size],
            min: p.clone(),
            max: p,
        }
    }

    fn get(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            return v;
        }

        let p = self.get(self.parent[v]);
        self.parent[v] = p;
        p
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.get(a);
        let mut b = self.get(b);
        if a != b {
            if self.size[a] > self.size[b] {
                std::mem::swap(&mut a, &mut b);
            }
            self.parent[a] = b;
            self.size[b] += self.size[a];
            self.min[b] = min(self.min[b], self.min[a]);
            self.max[b] = max(self.max[b], self.max[a]);
        }
    }
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let n = sc.read();
    let m = sc.read();

    let mut dsu = Dsu::new(n);

    for _ in 0..m {
        match sc.read::<String>().as_str() {
            "union" => {
                dsu.union(sc.usize0(), sc.usize0());
            }
            "get" => {
                let p = dsu.get(sc.usize0());
                sc.writeln(format!(
                    "{} {} {}",
                    dsu.min[p] + 1,
                    dsu.max[p] + 1,
                    dsu.size[p]
                ));
            }
            _ => unreachable!(),
        }
    }
}
