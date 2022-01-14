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

pub fn solve_one(arr: Vec<i64>) -> i64 {
    let n = arr.len() as i64;
    // let mut set = arr.iter().copied().collect::<HashSet<_>>();
    // let rems = (1..=n).filter(|x| !set.contains(x)).collect::<Vec<_>>();
    let mut set = (1..=n).collect::<HashSet<_>>();
    let mut idxs = HashSet::new();
    for (i, &m) in arr.iter().enumerate() {
        if set.contains(&m) {
            set.remove(&m);
            idxs.insert(i);
        }
    }
    // println!("{:?} {:?}", idxs, set);

    let mut rems = arr
        .iter()
        .enumerate()
        .filter_map(|(i, &m)| (!idxs.contains(&i)).then(|| m))
        .collect::<Vec<_>>();

    rems.sort_unstable();
    rems.reverse();

    let mut tars = set.into_iter().collect::<Vec<_>>();
    tars.sort_unstable();
    tars.reverse();

    // println!("{:?} {:?}", rems, tars);

    let mut count = 0;
    for (r, t) in rems.into_iter().zip(tars.into_iter()) {
        if r <= t * 2 {
            return -1;
        }
        count += 1;
    }
    count
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let ans = solve_one(sc.vecn());
        sc.writeln(ans);
    }
}
