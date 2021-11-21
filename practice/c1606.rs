#![allow(unused_imports, dead_code, clippy::many_single_char_names)]

use std::collections::*;

struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        // I should clear stdout
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

fn pow10(n: i64) -> i64 {
    let mut s = 1;
    for _ in 0..n {
        s *= 10
    }
    s
}

/*
Let d be the difference between two denominations.
take 10^d - 1 of the smaller demonation then repeat until we get to the largest denomination or run out.
Take whatever is left from the largest denomication then + 1 to get the smallest that cannot be made.
*/

fn solve_one(a: &[i64], mut k: i64) -> i64 {
    let mut s = 0;
    let mut i = 0;
    for (&p, &q) in a.iter().zip(a.iter().skip(1)) {
        let t = pow10(q - p) - 1;
        if k < t {
            break;
        }
        k -= t;
        s += t * pow10(p);
        i += 1;
    }
    s + (k + 1) * pow10(a[i])
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let k = sc.i64();
        let a = sc.vec::<i64>(n);
        let ans = solve_one(&a, k);
        sc.writeln(ans);
    }
}
