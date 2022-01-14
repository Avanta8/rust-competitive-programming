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

#[derive(Default, Debug)]
struct RotMap<T> {
    vec: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> RotMap<T> {
    fn add(&mut self, item: T) -> (usize, &T) {
        let idx = self.free.pop().unwrap_or_else(|| {
            let idx = self.vec.len();
            self.vec.push(None);
            idx
        });
        assert!(idx < self.vec.len());
        assert!(self.vec[idx].is_none());
        self.vec[idx] = Some(item);
        (idx, self.vec[idx].as_ref().unwrap())
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx).map(|x| x.as_ref()).flatten()
    }

    fn remove(&mut self, idx: usize) -> Option<T> {
        if let Some(item) = self.vec.get_mut(idx) {
            self.free.push(idx);
            std::mem::take(item)
        } else {
            None
        }
    }
}

pub fn solve_one(n: usize, grid: Vec<Vec<i64>>) -> i64 {
    println!("{} {:?}", n, grid);

    let mut rotmap = RotMap::default();
    let mut bag = BinaryHeap::new();
    let mut visited = HashMap::new();
    {
        // let removed = HashSet::new();
        let removed = BTreeSet::new();
        // let mut positions = HashSet::new();
        let mut positions = vec![];
        for x in 0..n {
            for y in 0..n {
                // positions.insert((x, y));
                positions.push((x, y));
            }
        }
        let state = (positions, removed);
        visited.insert(state.clone(), 0);
        bag.push((Reverse(0i64), rotmap.add(state).0));
    }

    while let Some((Reverse(cost), idx)) = bag.pop() {
        let state = rotmap.remove(idx).unwrap();
        assert!(cost == *visited.get(&state).unwrap());

        let (positions, removed) = state;
        // println!(
        //     "\n\n{}\n{}\n{:?}\n{:?}\n{:?}",
        //     cost, idx, positions, removed, visited
        // );
        // println!("{:?}", bag);
        // println!("postiions: {:?}", positions);

        if positions.iter().all(|&(x, y)| x >= n && y >= n) {
            println!("cost: {}", cost);
            return cost;
        }

        let mut new_positions = vec![];
        for row in 0..n * 2 {
            new_positions.push(
                positions
                    .iter()
                    .map(|&(x, y)| {
                        if y != row {
                            (x, y)
                        } else {
                            ((x + 1) % (2 * n), y)
                            // (x, (y + 1) % (2 * n))
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            new_positions.push(
                positions
                    .iter()
                    .map(|&(x, y)| {
                        if y != row {
                            (x, y)
                        } else {
                            // (x, ((y as isize - 1).rem_euclid(2 * n as isize)) as usize)
                            // (x, (2 * n + y - 1) % (2 * n))
                            ((2 * n + x - 1) % (2 * n), y)
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }
        for col in 0..n * 2 {
            new_positions.push(
                positions
                    .iter()
                    .map(|&(x, y)| {
                        if x != col {
                            (x, y)
                        } else {
                            // ((x + 1) % (2 * n), y)
                            (x, (y + 1) % (2 * n))
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            new_positions.push(
                positions
                    .iter()
                    .map(|&(x, y)| {
                        if x != col {
                            (x, y)
                        } else {
                            // (((x as isize - 1).rem_euclid(2 * n as isize)) as usize, y)
                            // ((2 * n + x - 1) % (2 * n), y)
                            (x, (2 * n + y - 1) % (2 * n))
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }

        // println!("new positions: {:?}", new_positions);

        let mut next_states = vec![];
        for mut positions in new_positions {
            positions.sort_unstable();
            positions.dedup();
            if positions.len() != n * n {
                panic!();
                continue;
            }
            let mut new_removed = removed.clone();
            let mut new_cost = cost;
            for &(x, y) in positions.iter() {
                if grid[y][x] > 0 && !new_removed.contains(&(x, y)) {
                    new_removed.insert((x, y));
                    new_cost += grid[y][x];
                }
            }
            next_states.push((new_cost, (positions, new_removed)))
        }

        for (new_cost, state) in next_states {
            // println!("new cost: {}", new_cost);
            if visited.get(&state).copied().unwrap_or(i64::MAX) <= new_cost {
                continue;
            }
            // println!("keep {:?}", state);
            visited.insert(state.clone(), new_cost);
            bag.push((Reverse(new_cost), rotmap.add(state).0));
        }
    }

    -1
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
