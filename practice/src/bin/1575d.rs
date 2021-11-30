#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::many_single_char_names
)]

use std::cmp::*;
use std::collections::*;

struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

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

fn pow(mut a: i64, mut b: i64) -> i64 {
    let mut r = 1;
    loop {
        if b & 1 == 1 {
            r *= a;
        }
        b >>= 1;
        if b == 0 {
            break;
        }
        a *= a;
    }
    r
}

fn solve_one(s: Vec<char>) -> i64 {
    if ["__".to_owned(), "_X".to_owned()].contains(&s.iter().collect::<String>()) {
        return 3;
    }
    if s.len() == 1 {
        return if "0_X".contains(s[0]) { 1 } else { 0 };
    }
    if s[0] == '0' {
        return 0;
    }

    let l = s[s.len() - 1];
    let k = s[s.len() - 2];

    let mut combs = HashMap::new();

    if !"05_X".contains(l) {
        return 0;
    }
    if l == '0' || l == '_' {
        if !"05_X".contains(k) {
            if l != '_' {
                return 0;
            }
        } else if k == 'X' {
            *combs.entry(Some(5)).or_insert(0) += 1;
            if s[0] != 'X' {
                *combs.entry(Some(0)).or_insert(0) += 1;
            }
        } else {
            *combs.entry(None).or_insert(0) += if k == '_' { 2 } else { 1 };
        }
    }
    if l == '5' || l == '_' {
        if !"27_X".contains(k) {
            if l != '_' {
                return 0;
            }
        } else if k == 'X' {
            *combs.entry(Some(2)).or_insert(0) += 1;
            *combs.entry(Some(7)).or_insert(0) += 1;
        } else {
            *combs.entry(None).or_insert(0) += if k == '_' { 2 } else { 1 };
        }
    }
    if l == 'X' {
        if k == '0' || k == '5' {
            if s[0] != 'X' {
                *combs.entry(Some(0)).or_insert(0) += 1;
            }
        } else if k == '2' || k == '7' {
            *combs.entry(Some(5)).or_insert(0) += 1;
        } else if k == '_' {
            if s[0] != 'X' {
                *combs.entry(Some(0)).or_insert(0) += if s.len() == 2 { 1 } else { 2 };
            }
            *combs.entry(Some(5)).or_insert(0) += 2;
        } else if k == 'X' {
            if s[0] != 'X' {
                *combs.entry(Some(0)).or_insert(0) += 1;
            }
        } else {
            return 0;
        }
    }

    let b = s[..s.len() - 2].to_vec();
    let u = b.iter().filter(|&&c| c == '_').count();
    let x = b.iter().filter(|&&c| c == 'X').count();

    let mut count = 0;

    for (xv, cnt) in combs {
        let a = match xv {
            Some(p) => u,
            None => u + if x > 0 { 1 } else { 0 },
        } as i64;

        let mut ch = pow(10, a);
        if s[0] == '_' && s.len() > 2 || xv.is_none() && s[0] == 'X' {
            ch = ch / 10 * 9;
        }

        count += ch * cnt;
    }

    count
}

// Wtf is this.......

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());
    let ans = solve_one(sc.chars());
    sc.writeln(ans);
}
