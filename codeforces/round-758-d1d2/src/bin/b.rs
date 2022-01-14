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

fn solve_one(n: i64, a: i64, b: i64) -> Vec<i64> {
    if (a - b).abs() > 1 || a + b + 2 > n {
        return [-1].into();
    }

    let mut queue = (1..=n).collect::<VecDeque<_>>();
    let mut v = vec![];
    if a >= b {
        if a > b {
            v.push(queue.pop_front().unwrap());
        }
        for _ in 0..b {
            v.push(queue.pop_back().unwrap());
            v.push(queue.pop_front().unwrap());
        }
        while let Some(x) = queue.pop_back() {
            v.push(x);
        }
    } else {
        v.push(queue.pop_back().unwrap());
        for _ in 0..a {
            v.push(queue.pop_front().unwrap());
            v.push(queue.pop_back().unwrap());
        }
        while let Some(x) = queue.pop_front() {
            v.push(x);
        }
    }
    v
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let ans = solve_one(sc.read(), sc.read(), sc.read());
        sc.writevec(&ans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_name() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let n: i64 = rng.gen_range(2..10_000);
            let a: i64 = rng.gen_range(0..n);
            // let b: i64 = rng.gen_range(0..n);

            let choice = rng.gen_range(0..=2);
            let b = if choice == 0 {
                a - 1
            } else if choice == 1 {
                a + 1
            } else {
                a
            };

            println!("{} {} {}", n, a, b);
            let ans = solve_one(n, a, b);

            if ans == [-1].to_vec() {
                continue;
            }

            let mut low = 0;
            let mut high = 0;
            for i in 1..n as usize - 1 {
                if ans[i] > ans[i - 1] && ans[i] > ans[i + 1] {
                    high += 1;
                }
                if ans[i] < ans[i - 1] && ans[i] < ans[i + 1] {
                    low += 1;
                }
            }

            println!("low: {}, high: {}", low, high);

            assert!(a == high);
            assert!(b == low);
        }
    }
}
