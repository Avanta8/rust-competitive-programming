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

fn solve_one(k: usize, mut v: Vec<i64>) -> i64 {
    // println!();
    let mut cache = HashMap::new();
    v.sort_unstable();
    helper(k, v, &mut cache)
}

fn helper(k: usize, v: Vec<i64>, cache: &mut HashMap<(usize, Vec<i64>), i64>) -> i64 {
    // println!("{} {:?}", k, v);

    if let Some(v) = cache.get(&(k, v.clone())) {
        return *v;
    }

    if k == 0 {
        return v.iter().copied().sum();
    }

    // let mut calc = helper(k - 1, &v[..v.len() - 2]);
    let mut calc = helper(
        k - 1,
        [&v[..v.len() - 1 - k], &v[v.len() - k..v.len() - 1]].concat(),
        cache,
    );

    if v[v.len() - k - 1] == v[v.len() - 1] {
        calc += 1;
    }

    if v[v.len() - k - 1] == v[v.len() - 1] && v[0] != v[v.len() - 1] {
        let idx = v.iter().position(|&x| x == v[v.len() - 1]).unwrap() - 1;
        let nv = (0..v.len())
            .filter_map(|i| if i != idx { Some(v[i]) } else { None })
            .collect::<Vec<_>>();
        calc = min(calc, helper(k - 1, nv[..nv.len() - 1].to_vec(), cache) + 1);
    }

    cache.insert((k, v), calc);

    calc
}

// fn helper(k: usize, v: &[i64]) -> i64 {
//     println!("{} {:?}", k, v);
//     if k == 0 {
//         return v.iter().copied().sum();
//     }

//     let mut calc = helper(k - 1, &v[..v.len() - 2]);

//     if v[v.len() - 2] == v[v.len() - 1] {
//         calc += 1;
//     }

//     if v[v.len() - 1] == v[v.len() - 2] && v.len() > 2 && v[0] != v[v.len() - 1] {
//         let idx = v.iter().position(|&x| x == v[v.len() - 1]).unwrap() - 1;
//         let nv = (0..v.len())
//             .filter_map(|i| if i != idx { Some(v[i]) } else { None })
//             .collect::<Vec<_>>();
//         calc = min(calc, helper(k - 1, &nv[..nv.len() - 1]) + 1);
//     }

//     calc
// }

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.read();
        let k = sc.read();
        let v = sc.vec(n);
        let ans = solve_one(k, v);
        sc.writeln(ans);
    }
}
