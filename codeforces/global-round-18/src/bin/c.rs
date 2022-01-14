#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::collapsible_else_if
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

pub fn solve_one(len: usize, start: Vec<bool>, target: Vec<bool>) -> i64 {
    if start == target {
        return 0;
    }
    // println!("{} {:?} {:?}", len, start, target);
    let parity = start
        .iter()
        .copied()
        .zip(target.iter().copied())
        .map(|(a, b)| a == b)
        .collect::<Vec<_>>();
    // even: true, odd: false

    let mut evens = (0, 0); // (0s, 1s)
    let mut odds = (0, 0);

    for i in 0..len {
        if start[i] {
            if parity[i] {
                evens.1 += 1;
            } else {
                odds.1 += 1;
            }
        } else {
            if parity[i] {
                evens.0 += 1;
            } else {
                odds.0 += 1;
            }
        }
    }

    // println!("{:?}", parity);
    // println!("{:?} {:?}", evens, odds);

    let can_odd = odds.0 == odds.1;
    let can_even = evens.1 == evens.0 + 1;

    let even_count = evens.1 + evens.0;
    let odd_count = odds.1 + odds.0;

    if odd_count <= even_count {
        if can_odd {
            return odd_count;
        } else if can_even {
            return even_count;
        }
    } else {
        if can_even {
            return even_count;
        } else if can_odd {
            return odd_count;
        }
    }

    -1
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let len = sc.usize();
        let a = sc.chars();
        let b = sc.chars();

        let a = a.into_iter().map(|c| c == '1').collect::<Vec<_>>();
        let b = b.into_iter().map(|c| c == '1').collect::<Vec<_>>();
        let ans = solve_one(len, a, b);
        sc.writeln(ans);
    }
}
