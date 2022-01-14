#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity
)]

use std::cmp::*;
use std::collections::*;
use std::io::Stdin;
use std::io::Stdout;

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
    pub fn bytes(&mut self) -> Vec<u8> {
        self.read::<String>().bytes().collect()
    }
}

pub struct Solver {
    sc: IO<Stdin, Stdout>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            sc: IO::new(std::io::stdin(), std::io::stdout()),
        }
    }
}

impl Solver {
    pub fn solve(&mut self) {
        self.run_case();
    }

    pub fn run_case(&mut self) {
        let len = self.sc.read();
        let height = self.sc.read();
        let width = self.sc.read();
        let moves = self.sc.bytes();

        if let Some(r) = self.solve_one(len, height, width, moves) {
            self.sc.writeln(r);
        } else {
            self.sc.writeln(-1);
        }
    }

    pub fn solve_one(&mut self, _n: usize, height: i64, width: i64, moves: Vec<u8>) -> Option<i64> {
        let mut state = State::new(&moves);

        let mut rem = (width, height);
        let mut total = 0;

        while rem.0 > 0 && rem.1 > 0 {
            let dir = state.next_state()?;

            let cnt = match dir {
                Left | Right => {
                    rem.0 -= 1;
                    height - (state.bounds.1 .1 - state.bounds.1 .0)
                }
                Up | Down => {
                    rem.1 -= 1;
                    width - (state.bounds.0 .1 - state.bounds.0 .0)
                }
            };
            total = (total + cnt * state.total_steps) % MOD;
        }

        Some(total)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

struct State {
    len: i64,
    shifts: Vec<((i64, i64), ((i64, i64), (i64, i64)))>, // prefix sum ((px, py), ((minx, maxx), (miny, maxy)))
    pos: (i64, i64),
    bounds: ((i64, i64), (i64, i64)), // ((minx, maxx), (miny, maxy))\
    total_steps: i64,
}

impl State {
    fn new(moves: &[u8]) -> Self {
        Self {
            len: moves.len() as i64,
            shifts: {
                let mut vec = vec![((0, 0), ((0, 0), (0, 0)))];
                let mut pos = (0, 0);
                let mut bounds = ((0, 0), (0, 0));
                for mv in moves.repeat(2) {
                    match mv {
                        b'L' => pos.0 -= 1,
                        b'R' => pos.0 += 1,
                        b'U' => pos.1 -= 1,
                        b'D' => pos.1 += 1,
                        _ => unreachable!(),
                    }
                    bounds.0 .0 = min(bounds.0 .0, pos.0);
                    bounds.0 .1 = max(bounds.0 .1, pos.0);
                    bounds.1 .0 = min(bounds.1 .0, pos.1);
                    bounds.1 .1 = max(bounds.1 .1, pos.1);
                    vec.push((pos, bounds));
                }
                vec
            },
            pos: (0, 0),
            bounds: ((0, 0), (0, 0)),
            total_steps: 0,
        }
    }

    fn next_state(&mut self) -> Option<Dir> {
        if let Some((steps, dir)) = self.next_exceed() {
            self.make_move(steps);
            Some(dir)
        } else {
            None
        }
    }

    fn next_exceed(&self) -> Option<(i64, Dir)> {
        let mut low = 0;
        let mut high = self.len;

        while low < high {
            let mid = (low + high + 1) / 2;

            if let Some(dir) = self.would_exceed(mid) {
                if high == mid {
                    return Some((mid, dir));
                }
                high = mid;
            } else {
                low = mid;
            }
        }
        None
    }

    fn would_exceed(&self, steps: i64) -> Option<Dir> {
        let (pre, _) = self.shifts[self.steps() as usize];
        let (_, bounds) = self.shifts[self.steps() + steps as usize];

        let start = (self.pos.0 - pre.0, self.pos.1 - pre.1);

        let left = start.0 + bounds.0 .0 < self.bounds.0 .0;
        let right = start.0 + bounds.0 .1 > self.bounds.0 .1;
        let up = start.1 + bounds.1 .0 < self.bounds.1 .0;
        let down = start.1 + bounds.1 .1 > self.bounds.1 .1;

        None.or_else(|| left.then(|| Left))
            .or_else(|| right.then(|| Right))
            .or_else(|| up.then(|| Up))
            .or_else(|| down.then(|| Down))
    }

    fn make_move(&mut self, steps: i64) {
        let (pre, _) = self.shifts[self.steps()];
        let start = (self.pos.0 - pre.0, self.pos.1 - pre.1);

        let (dpos, bounds) = self.shifts[self.steps() + steps as usize];

        self.pos = (start.0 + dpos.0, start.1 + dpos.1);
        self.bounds.0 .0 = min(self.bounds.0 .0, start.0 + bounds.0 .0);
        self.bounds.0 .1 = max(self.bounds.0 .1, start.0 + bounds.0 .1);
        self.bounds.1 .0 = min(self.bounds.1 .0, start.1 + bounds.1 .0);
        self.bounds.1 .1 = max(self.bounds.1 .1, start.1 + bounds.1 .1);

        self.total_steps += steps;
    }

    fn steps(&self) -> usize {
        (self.total_steps % self.len) as usize
    }
}

const MOD: i64 = 1_000_000_007;

pub fn main() {
    let mut solver = Solver::default();
    solver.solve();
}
