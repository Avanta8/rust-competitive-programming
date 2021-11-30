#![allow(dead_code, unused_imports, unused_variables)]

use std::cmp::*;
use std::collections::*;

struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

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

    pub fn is_ancestor(&self, v: usize) -> bool {
        self.parent[v] == v
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

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut sa = self.get(a);
        let mut sb = self.get(b);
        if sa != sb {
            if self.size[sa] < self.size[sb] {
                std::mem::swap(&mut sa, &mut sb);
            }
            self.parent[sb] = sa;
            self.size[sa] += self.size[sb];
        }

        sa != sb
    }
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let (n, d) = (sc.read(), sc.read());

    let mut dsu = Dsu::new(n);

    for idx in 0..d {
        dsu.unite(sc.usize0(), sc.usize0());

        let mut lens = (0..n)
            .filter_map(|v| {
                if dsu.is_ancestor(v) {
                    Some(dsu.size[v])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        lens.sort_unstable();
        lens.reverse();

        let mut con: usize = lens.iter().map(|&x| x - 1).sum();

        let mut total = 0;
        let mut cnt = idx + 2;
        for len in lens {
            if cnt == con {
                break;
            }
            cnt -= len;
            con -= len - 1;
            total += len;
        }

        sc.writeln(total - 1);
    }
}
