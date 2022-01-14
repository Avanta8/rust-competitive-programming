#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain
)]

use std::cell::RefCell;
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

#[derive(Debug)]
pub struct Tree {
    root: usize,
    parent: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
    leaves: HashSet<usize>,
    n: usize,
}

impl Tree {
    pub fn new(n: usize, graph: &[Vec<usize>], root: usize) -> Self {
        let mut parent = vec![None; n];
        let mut children = vec![vec![]; n];

        let mut bag = vec![root];
        let mut seen = vec![false; n];
        seen[root] = true;

        while let Some(pos) = bag.pop() {
            for &neighbour in graph[pos].iter() {
                if seen[neighbour] {
                    continue;
                }
                seen[neighbour] = true;
                bag.push(neighbour);
                parent[neighbour] = Some(pos);
                children[pos].push(neighbour);
            }
        }

        let leaves = children
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.is_empty().then(|| i))
            .collect();

        Self {
            root,
            parent,
            children,
            leaves,
            n,
        }
    }

    pub fn parents_up(&self, node: usize) -> Vec<usize> {
        let mut vec = vec![];
        let mut current = node;
        while let Some(parent) = self.parent[current] {
            vec.push(parent);
            current = parent;
        }
        vec
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

type Sizes = (usize, BTreeMap<usize, HashSet<usize>>);

fn calc_size(tree: &Tree) -> Vec<Sizes> {
    let sizes = RefCell::new(vec![(0, BTreeMap::new()); tree.n]);

    struct SizeHelper<'a> {
        f: &'a dyn Fn(&SizeHelper, usize) -> usize,
    }

    let size_helper = SizeHelper {
        f: &|size_helper, current| {
            if tree.children[current].is_empty() {
                sizes.borrow_mut()[current].0 = 1;
                1
            } else {
                let mut total = 1;
                let mut children_sizes = BTreeMap::new();
                for &child in tree.children[current].iter() {
                    let child_size = (size_helper.f)(size_helper, child);
                    children_sizes
                        .entry(child_size)
                        .or_insert_with(HashSet::new)
                        .insert(child);
                    total += child_size;
                }
                sizes.borrow_mut()[current] = (total, children_sizes);
                total
            }
        },
    };
    (size_helper.f)(&size_helper, 0);

    sizes.into_inner()
}

fn find_best(tree: &Tree, sizes: &[Sizes]) -> usize {
    let mut current = 0;
    while !tree.children[current].is_empty() {
        let best = sizes[current].1.values().next_back().unwrap();
        current = *best.iter().next().unwrap();
    }
    current
}

fn update(tree: &Tree, sizes: &mut [Sizes], used: &mut [bool], node: usize) -> usize {
    // Size of parent should end up being the maximum of the sizes of its children.

    // node must be a leaf.
    assert!(tree.leaves.contains(&node));
    assert!(!used[node]);

    let mut current = node;
    sizes[current].0 = 0;

    used[current] = true;
    let mut used_count = 1;

    let mut last_size = 1;

    while let Some(parent) = tree.parent[current] {
        if !used[parent] {
            used[parent] = true;
            used_count += 1;
        }
        let children = sizes[parent].1.get_mut(&last_size).unwrap();
        children.remove(&current);
        if children.is_empty() {
            sizes[parent].1.remove(&last_size).unwrap();
        }

        last_size = sizes[parent].0;
        sizes[parent]
            .1
            .entry(sizes[current].0)
            .or_insert_with(HashSet::new)
            .insert(current);
        sizes[parent].0 = *sizes[parent].1.keys().next_back().unwrap();

        current = parent;
    }
    used_count
}

fn solve_one(edges: Vec<(usize, usize)>, len: usize, k: usize) -> i64 {
    let graph = gen_graph(&edges, len);
    let tree = Tree::new(len, &graph, 0);

    let mut sizes = calc_size(&tree);
    let mut used = vec![false; len];

    // println!("sizes: {:?}", sizes);

    let mut unused_count = len;
    let mut best_score = i64::MIN;
    for red in 1..=min(k, tree.leaves.len()) {
        let best = find_best(&tree, &sizes);
        // println!("best: {}", best);
        assert!(sizes[best].0 == 1);
        unused_count -= update(&tree, &mut sizes, &mut used, best);
        // println!("new sizes: {:?}", sizes);

        let blue = min(unused_count, len / 2);
        let white = len - red - blue;
        best_score = white as i64 * (red as i64 - blue as i64);
    }

    if k >= tree.leaves.len() {
        let opt = min(len / 2, k) as i64;
        best_score = max(best_score, opt * (len as i64 - opt));
        assert!(best_score >= 0);
    }

    best_score
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let len = sc.read();
    let k = sc.read();
    let edges = (0..len - 1).map(|_| (sc.usize0(), sc.usize0())).collect();

    let ans = solve_one(edges, len, k);
    sc.writeln(ans);
}
