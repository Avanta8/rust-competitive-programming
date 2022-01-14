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

pub fn solve_one(n: usize, ranges: Vec<(usize, usize)>) -> Vec<(usize, usize, usize)> {
    // println!("\n{} {:?}", n, ranges);

    let mut starts = vec![vec![]; n];
    let mut ends = vec![vec![]; n];
    for &(a, b) in ranges.iter() {
        starts[a].push(b);
        ends[b].push(a);
    }

    for thing in starts.iter_mut() {
        thing.sort_unstable();
    }
    for thing in ends.iter_mut() {
        thing.sort_unstable();
        thing.reverse()
    }

    // println!("{:?}", starts);
    // println!("{:?}", ends);

    let mut done = HashSet::new();

    let mut res = vec![];
    for (start, end) in ranges.iter().copied() {
        // println!("{} {}", start, end);
        if start == end {
            res.push((start, start, start));
        }

        let mut tar_end = None;
        for &cho_end in starts[start].iter() {
            if cho_end > end {
                tar_end = Some(cho_end);
                break;
            }
        }

        let mut tar_start = None;
        for &cho_start in ends[end].iter() {
            if cho_start < start {
                tar_start = Some(cho_start);
                break;
            }
        }

        if tar_end.is_some() {
            assert!(tar_start.is_none());
            if done.contains(&(start, tar_end.unwrap())) {
                continue;
            }
            done.insert((start, tar_end.unwrap()));
            res.push((start, tar_end.unwrap(), end + 1));
        }
        if tar_start.is_some() {
            assert!(tar_end.is_none());
            if done.contains(&(tar_start.unwrap(), end)) {
                continue;
            }
            done.insert((tar_start.unwrap(), end));
            res.push((tar_start.unwrap(), end, start - 1));
        }

        if tar_end.is_none() && tar_start.is_none() {
            // println!("NONONO");
        }
    }

    res
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.read();
        let ranges = (0..n).map(|_| (sc.usize0(), sc.usize0())).collect();
        let ans = solve_one(n, ranges);
        // sc.writeln(ans);
        for (a, b, c) in ans {
            sc.writevec(&[a + 1, b + 1, c + 1]);
        }
        sc.write("\n");
    }
}
