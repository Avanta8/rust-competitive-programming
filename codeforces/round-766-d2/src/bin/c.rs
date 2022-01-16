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
use std::ops::RangeBounds;

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

pub fn gen_graph(edges: &[(usize, usize)], len: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; len];
    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph
}

pub fn find_leaves(graph: &[Vec<usize>]) -> Vec<usize> {
    graph
        .iter()
        .enumerate()
        .filter_map(|(i, neighbours)| (neighbours.len() == 1).then(|| i))
        .collect()
}

pub fn solve_one(len: usize, edges: Vec<(usize, usize)>) -> Option<Vec<i64>> {
    // dbg!(len, &edges);
    let graph = gen_graph(&edges, len);

    let leaves = find_leaves(&graph);

    let mut current = leaves[0];

    let mut val = 2;

    let mut results = HashMap::new();

    let mut visited = HashSet::new();
    visited.insert(current);

    loop {
        let neighbours = &graph[current];
        if neighbours.len() > 2 {
            return None;
        }
        let next = if neighbours.len() == 1 {
            if visited.contains(&neighbours[0]) {
                break;
            }
            neighbours[0]
        } else {
            if !visited.contains(&neighbours[0]) {
                neighbours[0]
            } else if !visited.contains(&neighbours[1]) {
                neighbours[1]
            } else {
                break;
            }
        };
        assert!(!visited.contains(&next));
        visited.insert(next);

        results.insert((current, next), val);
        current = next;
        val = if val == 2 { 3 } else { 2 };
    }

    let mut ans = vec![];
    for (a, b) in edges {
        let t = if let Some(&f) = results.get(&(a, b)) {
            f
        } else {
            *results.get(&(b, a)).unwrap()
        };
        ans.push(t);
    }

    Some(ans)
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let len = sc.read();
        let edges = (0..len - 1).map(|_| (sc.usize0(), sc.usize0())).collect();
        let ans = solve_one(len, edges);
        if let Some(ans) = ans {
            sc.writevec(&ans);
        } else {
            sc.writeln(-1);
        }
    }
}
