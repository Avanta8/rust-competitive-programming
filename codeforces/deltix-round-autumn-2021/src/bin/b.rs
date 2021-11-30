#![allow(unused_imports, clippy::many_single_char_names)]

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

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let (n, q) = (sc.usize(), sc.usize());
    let mut s = sc.chars();
    s.push('_');
    s.push('_');
    s.push('_');

    let mut loc = vec![None; n];
    for i in 0..max(n, 2) - 2 {
        if s[i] == 'a' && s[i + 1] == 'b' && s[i + 2] == 'c' {
            loc[i] = Some(i);
            loc[i + 1] = Some(i);
            loc[i + 2] = Some(i);
        }
    }

    // println!("{:?}", loc);

    let mut count = loc.iter().filter_map(|&a| a).count() / 3;

    for _ in 0..q {
        // println!();
        let idx = sc.usize() - 1;
        let c: char = sc.read();

        let ps = loc[idx];
        if let Some(i) = ps {
            loc[i] = None;
            loc[i + 1] = None;
            loc[i + 2] = None;
            count -= 1;
        }

        s[idx] = c;
        // for i in (max(3, idx) - 3)..idx + 3 {
        //     if s[i] == 'a' && s[i + 1] == 'b' && s[i + 2] == 'c' {
        //         loc[i] = Some(i);
        //         loc[i + 1] = Some(i);
        //         loc[i + 2] = Some(i);
        //     }
        // }
        let i = if c == 'a' {
            idx + 2
        } else if c == 'b' {
            idx + 1
        } else {
            idx
        };

        if i >= 2 {
            let i = i - 2;
            if s[i] == 'a' && s[i + 1] == 'b' && s[i + 2] == 'c' {
                loc[i] = Some(i);
                loc[i + 1] = Some(i);
                loc[i + 2] = Some(i);
                count += 1;
            }
        }

        // println!("{} {}", idx, c);
        // println!("{:?}", s);
        // println!("{:?}", loc);
        sc.writeln(count);
    }
}
