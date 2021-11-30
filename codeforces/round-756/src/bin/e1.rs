#![allow(dead_code, unused_imports, unused_variables)]

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

type Graph = HashMap<i64, HashSet<i64>>;

fn gen_graph(edges: &[(i64, i64)]) -> Graph {
    let mut graph = HashMap::new();
    for &(a, b) in edges {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    graph
}

fn find_times(graph: &Graph, friends: &[i64]) -> HashMap<i64, i64> {
    let mut bag = VecDeque::new();
    for &f in friends {
        bag.push_back((0, f));
    }

    let mut times = HashMap::new();

    while let Some((time, pos)) = bag.pop_back() {
        if times.contains_key(&pos) {
            continue;
        }
        // println!("{:?}", pos);

        times.insert(pos, time);

        for &neighbour in graph.get(&pos).unwrap() {
            bag.push_front((time + 1, neighbour));
        }
    }

    times
}

fn solve_one(n: usize, friends: &[i64], edges: &[(i64, i64)]) -> bool {
    let graph = gen_graph(edges);
    let times = find_times(&graph, friends);
    // println!();
    // println!("{:?}", graph);
    // println!("{:?}", times);

    let mut bag = VecDeque::new(); // (time, pos)
    let mut visited = HashSet::new();
    bag.push_back((0, 1));

    while let Some((time, pos)) = bag.pop_back() {
        if *times.get(&pos).unwrap() <= time || visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        let neighbours = graph.get(&pos).unwrap();

        if pos != 1 && neighbours.len() == 1 {
            return true;
        }

        for &neighbour in neighbours {
            bag.push_front((time + 1, neighbour));
        }
    }

    false
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let n = sc.read();
        let k = sc.vecn();
        let edges = (0..n - 1)
            .map(|_| (sc.read(), sc.read()))
            .collect::<Vec<_>>();
        let ans = solve_one(n, &k, &edges);
        sc.writeln(if ans { "yes" } else { "no" });
    }
}
