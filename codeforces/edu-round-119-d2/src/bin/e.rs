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

    let mut length = 0;
    let mut positions = HashMap::new();

    for _ in 0..sc.read() {
        let q = sc.i64();
        match q {
            1 => {
                let x = sc.i64();
                positions.entry(x).or_insert_with(Vec::new).push(length);
                length += 1;
            }
            2 => {
                let x = sc.i64();
                let y = sc.i64();

                let mut xs = positions.remove(&x).unwrap_or_default();
                let mut ys = positions.remove(&y).unwrap_or_default();

                if xs.len() > ys.len() {
                    std::mem::swap(&mut xs, &mut ys);
                }
                ys.extend(xs);
                positions.insert(y, ys);
            }
            _ => unreachable!(),
        }
    }

    let mut arr = vec![];
    for (k, v) in positions {
        for idx in v {
            if idx + 1 > arr.len() {
                arr.resize(idx + 1, 0);
            }
            arr[idx] = k;
        }
    }

    sc.writevec(&arr);
}
