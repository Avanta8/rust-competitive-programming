#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
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

#[derive(Debug)]
struct Row {
    row: BTreeMap<usize, i64>,
    pentalty: i64,
}

impl Row {
    fn new(pentalty: i64) -> Self {
        Self {
            row: [].into(),
            pentalty,
        }
    }

    fn insert(&mut self, pos: usize, value: i64) {
        if let Some(v) = self.get(pos) {
            if v <= value {
                return;
            }
        }

        self.row.insert(pos, value);

        let mut to_remove = vec![];

        for (&l, &cost) in self.row.range(..pos).rev() {
            if value + (pos - l) as i64 * self.pentalty <= cost {
                to_remove.push(l);
            } else {
                break;
            }
        }

        for (&r, &cost) in self.row.range(pos + 1..) {
            if value + (r - pos) as i64 * self.pentalty <= cost {
                to_remove.push(r);
            } else {
                break;
            }
        }

        for x in to_remove {
            self.row.remove(&x);
        }
    }

    fn get(&self, pos: usize) -> Option<i64> {
        let left = self
            .row
            .range(..pos)
            .next_back()
            .map(|left| left.1 + self.pentalty * (pos - left.0) as i64);
        let right = self
            .row
            .range(pos..)
            .next()
            .map(|right| right.1 + self.pentalty * (right.0 - pos) as i64);

        match (left, right) {
            (Some(l), Some(r)) => Some(min(l, r)),
            (l, r) => l.or(r),
        }
    }

    fn is_empty(&self) -> bool {
        self.row.is_empty()
    }
}

pub fn solve_one(
    height: usize,
    width: usize,
    penalty: Vec<i64>,
    ladders_raw: Vec<((usize, usize), (usize, usize), i64)>,
) -> Option<i64> {
    let mut ladders = HashMap::new();
    for &(start, end, h) in ladders_raw.iter() {
        ladders.entry(start).or_insert_with(Vec::new).push((end, h));
    }

    let mut room_ladders = vec![HashMap::new(); height];
    for ((x, y), map) in ladders {
        room_ladders[y].insert(x, map);
    }

    let mut rows = vec![];
    for p in penalty {
        rows.push(Row::new(p));
    }
    rows[0].insert(0, 0);

    for row in 0..height {
        if rows[row].is_empty() {
            continue;
        }
        for (&sx, map) in room_ladders[row].iter() {
            let dist = rows[row].get(sx).unwrap();
            for &((ex, ey), h) in map {
                rows[ey].insert(ex, dist - h);
            }
        }
    }

    rows[height - 1].get(width - 1)
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let height = sc.read();
        let width = sc.read();
        let num_ladders = sc.read();
        let penalty = sc.vec(height);
        let ladders = (0..num_ladders)
            .map(|_| {
                let (a, b, c, d, h) = (
                    sc.usize0(),
                    sc.usize0(),
                    sc.usize0(),
                    sc.usize0(),
                    sc.read(),
                );
                ((b, a), (d, c), h)
            })
            .collect();
        let ans = solve_one(height, width, penalty, ladders);
        if let Some(a) = ans {
            sc.writeln(a);
        } else {
            sc.writeln("NO ESCAPE");
        }
    }
}
