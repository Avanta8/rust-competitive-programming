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

fn solve_one(n: usize, sa: Vec<i64>, sb: Vec<i64>) -> String {
    let mut ka = (0..n).collect::<Vec<_>>();
    let mut kb = (0..n).collect::<Vec<_>>();
    ka.sort_unstable_by_key(|&i| sa[i]);
    kb.sort_unstable_by_key(|&i| sb[i]);

    let mut get = HashMap::new();
    for (i, &a) in ka.iter().enumerate() {
        get.entry(a).or_insert((0, 0)).0 = i;
    }
    for (i, &b) in kb.iter().enumerate() {
        get.entry(b).or_insert((0, 0)).1 = i;
    }

    let mut ba = n - 1;
    let mut bb = n - 1;

    loop {
        let old = (ba, bb);
        for i in ba..n {
            bb = min(get.get(&i).unwrap().1, bb);
        }
        for i in bb..n {
            ba = min(get.get(&i).unwrap().0, ba);
        }
        if (ba, bb) == old {
            break;
        }
    }

    // println!("{} {}", ba, bb);

    let mut v = vec![0; n];
    for &a in ka[ba..].iter() {
        v[a] = 1;
    }
    for &b in kb[bb..].iter() {
        v[b] = 1;
    }

    v.iter().map(|&v| v.to_string()).collect::<String>()
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let sa = sc.vec(n);
        let sb = sc.vec(n);
        let ans = solve_one(n, sa, sb);
        sc.writeln(ans);
    }
}
