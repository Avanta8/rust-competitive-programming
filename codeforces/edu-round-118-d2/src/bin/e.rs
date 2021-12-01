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

fn solve_one(width: usize, height: usize, grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // println!("start ----------------");
    let mut lab = None;
    for (y, row) in grid.iter().enumerate() {
        for (x, &sq) in row.iter().enumerate() {
            if sq == 'L' {
                lab = Some((x, y));
            }
        }
    }
    let lab = lab.unwrap();

    let mut res = grid.clone();
    let mut bag = vec![lab];

    while let Some(current) = bag.pop() {
        let (cx, cy) = current;
        // println!("\ncurrent: {}, {}", cx, cy);
        if res[cy][cx] == '+' {
            // println!("quick continue");
            continue;
        }
        // } else if res[cy][cx] == 'L' {
        //     grid
        // }

        let mut neighbours = vec![];
        let mut sqs = vec![];
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nx, ny) = (cx as isize + dx, cy as isize + dy);
            if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[ny][nx] != '#' {
                // println!("{}, {}", nx, ny);
                neighbours.push((nx, ny));
                sqs.push(res[ny][nx]);
            }
        }

        let free = sqs.iter().filter(|&&c| c == '.').count();
        let reach = sqs.iter().filter(|&&c| c == '+' || c == 'L').count();

        // println!("free: {} reach: {}", free, reach);

        if (free > 1 || reach == 0) && grid[cy][cx] != 'L' {
            // println!("Cant");
            continue;
        }

        // println!("adding");
        res[cy][cx] = '+';
        for pos in neighbours {
            bag.push(pos);
        }
    }

    res[lab.1][lab.0] = 'L';
    res
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let (height, width) = (sc.usize(), sc.usize());
        let grid = (0..height).map(|_| sc.chars()).collect::<Vec<_>>();
        let ans = solve_one(width, height, grid);
        let p = ans
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        sc.writeln(p);
    }
}
