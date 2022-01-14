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
use std::vec;

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

// pub fn solve_one(n: usize, grid: Vec<Vec<i64>>) -> i64 {
//     let mut bag = BinaryHeap::new();
//     let mut visited = HashMap::new();
//     // for i in 0..n {
//     //     bag.push((Reverse(0), (i, n - 1)));
//     //     if i != n - 1 {
//     //         bag.push((Reverse(0), (n - 1, i)));
//     //     }
//     // }
//     // for a in 0..n {
//     //     for b in 0..n {
//     //         visited.insert((a, b), 0);
//     //     }
//     // }
//     bag.push((Reverse(0), (0, 0)));
//     visited.insert((0, 0), 0);

//     while let Some((Reverse(dist), (px, py))) = bag.pop() {
//         if px >= n && (py + 1 == n || py == 0) || py >= n && (px + 1 == n || px == 0) {
//             let mut total = dist;

//             for x in n..n * 2 {
//                 for y in n..n * 2 {
//                     total += grid[y][x];
//                 }
//             }
//             return total;
//         }

//         for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
//             let nx = (px as isize + dx).rem_euclid(n as isize * 2) as usize;
//             let ny = (py as isize + dy).rem_euclid(n as isize * 2) as usize;

//             let new_dist = dist + grid[ny][nx];
//             if visited.get(&(nx, ny)).copied().unwrap_or(i64::MAX) > new_dist {
//                 visited.insert((nx, ny), new_dist);
//                 bag.push((Reverse(new_dist), (nx, ny)));
//             }
//         }
//     }
//     unreachable!()
// }
pub fn solve_one(n: usize, grid: Vec<Vec<i64>>) -> i64 {
    let mut total = [
        grid[n][0],
        grid[0][n],
        grid[n][n - 1],
        grid[n - 1][n],
        grid[2 * n - 1][n - 1],
        grid[n - 1][2 * n - 1],
        grid[2 * n - 1][0],
        grid[0][2 * n - 1],
    ]
    .into_iter()
    .min()
    .unwrap();

    for x in n..n * 2 {
        for y in n..n * 2 {
            total += grid[y][x];
        }
    }
    total
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.usize();
        let grid = (0..n * 2).map(|_| sc.vec(n * 2)).collect();
        let ans = solve_one(n, grid);
        sc.writeln(ans);
    }
}
