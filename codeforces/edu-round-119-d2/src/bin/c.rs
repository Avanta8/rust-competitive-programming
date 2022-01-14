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

pub fn solve_one(_n: i64, k: i64, x: i64, s: Vec<char>) -> String {
    let mut tfs = vec![];
    let mut count = 0;
    for &c in s.iter() {
        if c == 'a' {
            if count != 0 {
                tfs.push(count * k);
            }
            count = 0;
        } else {
            count += 1;
        }
    }
    if count != 0 {
        tfs.push(count * k);
    }

    let gaps = tfs.len();
    if gaps == 0 {
        return s.into_iter().collect();
    }

    let mut df = vec![1i64; gaps];
    for i in (0..gaps - 1).rev() {
        df[i] = df[i + 1].checked_mul(tfs[i + 1] + 1).unwrap_or(i64::MAX);
    }

    let mut rem = x - 1;
    let mut counts = vec![0; gaps];
    for i in 0..gaps {
        counts[i] = rem / df[i];
        rem %= df[i];
    }

    let mut ans = vec![];
    let mut s_iter = s.iter().copied().peekable();
    let mut current_count = 0;
    while let Some(current) = s_iter.next() {
        if current == 'a' {
            ans.push('a');
        } else {
            while s_iter.peek() == Some(&'*') {
                s_iter.next();
            }
            ans.resize(ans.len() + counts[current_count] as usize, 'b');
            current_count += 1;
        }
    }

    ans.into_iter().collect::<String>()
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.read();
        let k = sc.read();
        let x = sc.read();
        let s = sc.chars();
        let ans = solve_one(n, k, x, s);
        sc.writeln(ans);
    }
}
