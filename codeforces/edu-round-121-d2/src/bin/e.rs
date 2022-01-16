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

pub fn gen_graph(edges: &[(usize, usize)], len: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; len];
    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph
}

pub fn solve_one(len: usize, colours: Vec<bool>, edges: Vec<(usize, usize)>) -> Vec<bool> {
    let graph = gen_graph(&edges, len);

    let mut edge_marks = vec![];

    // only need to consider 2 starting points
    for start in (0..len).filter(|&i| colours[i]).take(2) {
        let mut bag = vec![(start, false)];

        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some((pos, seen)) = bag.pop() {
            for &neighbour in graph[pos].iter() {
                if visited.contains(&neighbour) {
                    continue;
                }
                let new_seen = seen || graph[neighbour].iter().any(|&n| n != start && colours[n]);

                if seen {
                    edge_marks.push((pos, neighbour))
                }

                visited.insert(neighbour);
                bag.push((neighbour, new_seen));
            }
        }
    }

    let mut ans = [false].repeat(len);
    for (a, b) in edge_marks {
        ans[a] = true;
        ans[b] = true;
    }

    ans
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let len = sc.read();
    let colours = sc.vec::<u8>(len).into_iter().map(|x| x == 1).collect();
    let edges = (0..len - 1).map(|_| (sc.usize0(), sc.usize0())).collect();
    let ans = solve_one(len, colours, edges);
    sc.writevec(
        &ans.into_iter()
            .map(|x| if x { 1 } else { 0 })
            .collect::<Vec<_>>(),
    );
}
