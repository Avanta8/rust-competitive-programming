#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
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
    pub fn writesep<T: ToString>(&mut self, v: &[T], sep: &str) {
        let s = v
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep);
        self.writeln(format!("{} ", &s));
    }
    pub fn writevec<T: ToString>(&mut self, v: &[T]) {
        self.writesep(v, " ")
    }
    pub fn writejoin<T: ToString>(&mut self, v: &[T]) {
        self.writesep(v, "")
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

pub fn solve_one() -> i64 {
    unimplemented!();
}

// pub fn main() {
//     let mut sc = IO::new(std::io::stdin(), std::io::stdout());

//     for _ in 0..sc.read() {
//         let n = sc.usize();
//         let mut low = (i64::MAX, 0, 0); // (value, cost, right)
//         let mut high = (0, 0, i64::MAX);
//         let mut total_cost = 0;
//         for i in 0..n {
//             let left = sc.i64();
//             let right = sc.i64();
//             let cost = sc.i64();

//             if left < low.0 || left == low.0 && (cost < low.1 || right > low.2) {
//                 // if left < low.0 || left == low.0 && (right > low.2) {
//                 low = (left, cost, right);
//             }
//             if right > high.0 || right == high.0 && (cost < high.1 || left < high.2) {
//                 // if right > high.0 || right == high.0 && (left < high.2) {
//                 high = (right, cost, left);
//             }
//             if (low.2 == high.0) {
//                 assert!(high.2 == low.0);
//                 assert!(low.1 == high.1);
//                 sc.writeln(low.1);
//             } else {
//                 sc.writeln(low.1 + high.1);
//             }
//         }
//     }
// }

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let mut low = (i64::MAX, i64::MAX, n + 1); // (value, cost, seg)
        let mut high = (0, i64::MAX, n + 2);
        for i in 0..n {
            let left = sc.i64();
            let right = sc.i64();
            let cost = sc.i64();
            // println!("{:?} {:?}", low, high);

            if low.2 == high.2 {
                if left <= low.0 && right >= high.0 && cost < low.1 {
                    low = (left, cost, i);
                    high = (right, cost, i);
                } else {
                    if left < low.0 {
                        low = (left, cost, i);
                    }
                    if right > high.0 {
                        high = (right, cost, i);
                    }
                }
            } else {
                if left <= low.0 && right <= high.0 && cost < (low.1 + high.1) {
                    low = (left, cost, i);
                    high = (right, cost, i);
                } else {
                    if left < low.0 || left == low.0 && cost < low.1 {
                        low = (left, cost, i);
                        if right >= high.0 {
                            high = (right, cost, i);
                        }
                    }
                    if right > high.0 || right == high.0 && cost < high.1 {
                        high = (right, cost, i);
                        if left <= low.0 {
                            low = (left, cost, i);
                        }
                    }
                }
            }

            if low.2 == high.2 {
                assert!(low.1 == high.1);
                sc.writeln(low.1);
            } else {
                sc.writeln(low.1 + high.1);
            }
        }
    }
}
