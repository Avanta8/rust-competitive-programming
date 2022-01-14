#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain
)]

use std::cmp::*;
use std::collections::*;
use std::f32::consts::E;
use std::process::id;

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

fn gen_graph(edges: &[(usize, usize)], len: usize) -> Vec<HashSet<usize>> {
    let mut graph = vec![HashSet::new(); len];
    for &(a, b) in edges {
        graph[a].insert(b);
        graph[b].insert(a);
    }
    graph
}

fn to_tree(graph: &[HashSet<usize>]) -> Vec<(HashSet<usize>, Option<usize>)> {
    let mut tree = vec![(HashSet::new(), None); graph.len()];
    let mut bag = vec![0];
    let mut seen = vec![false; graph.len()];
    seen[0] = true;

    while let Some(pos) = bag.pop() {
        for &neighbour in graph[pos].iter() {
            if seen[neighbour] {
                continue;
            }
            bag.push(neighbour);
            seen[neighbour] = true;
            tree[pos].0.insert(neighbour);
            tree[neighbour].1 = Some(pos);
        }
    }
    tree
}

// fn calc_size(tree: &[HashSet<usize>]) {
//     let mut sizes = vec![0; tree.len()];
//     let leaves = tree
//         .iter()
//         .enumerate()
//         .filter_map(|(idx, below)| below.is_empty().then(|| idx))
//         .collect::<Vec<_>>();

//     let mut bag = leaves;
// }

fn calc_size(tree: &[(HashSet<usize>, Option<usize>)], sizes: &mut [usize], idx: usize) -> usize {
    sizes[idx] = if tree[idx].0.is_empty() {
        0
    } else {
        tree[idx]
            .0
            .iter()
            .map(|&child| calc_size(tree, sizes, child))
            .sum()
    } + 1;
    sizes[idx]
}

fn get_best(tree: &[(HashSet<usize>, Option<usize>)], sizes: &[usize]) -> usize {
    let mut current = 0;
    while !tree[current].0.is_empty() {
        current = tree[current]
            .0
            .iter()
            .copied()
            .max_by_key(|&child| sizes[child])
            .unwrap();
    }
    current
}

fn mark(tree: &[(HashSet<usize>, Option<usize>)], sizes: &mut [usize], idx: usize) {
    let mut current = idx;
    let mut current_size = 1;
    sizes[current] = 0;
    while let Some(parent) = tree[current].1 {
        sizes[parent] -= current_size + 1;
        current_size += 1;
        current = parent;
    }
}

pub fn solve_one(edges: Vec<(usize, usize)>, len: usize, k: usize) -> i64 {
    let graph = gen_graph(&edges, len);
    let tree = to_tree(&graph);
    let mut used = vec![false; len];

    let mut sizes = vec![0; len];
    calc_size(&tree, &mut sizes, 0);

    println!("{:?}", graph);
    println!("{:?}", tree);

    println!("{:?}", sizes);

    for _ in 0..k {
        let best = get_best(&tree, &sizes);
        mark(&tree, &mut sizes, best);
        used[best] = true;
        println!("{:?}", sizes);

        println!("best: {}", best);
    }
    0
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let len = sc.usize();
    let k = sc.usize();
    let edges = (0..len - 1).map(|_| (sc.usize0(), sc.usize0())).collect();

    let ans = solve_one(edges, len, k);
    sc.writeln(ans);
}
