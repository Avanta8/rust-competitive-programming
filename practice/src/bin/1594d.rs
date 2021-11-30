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

// crewmate: true, impostor: false

type Graph = HashMap<i64, (HashSet<(i64, bool)>, HashSet<(i64, bool)>)>;

fn find_cycles(graph: &Graph) -> Vec<HashSet<i64>> {
    let mut visited = HashSet::new();

    let mut cycles = vec![];

    for &p in graph.keys() {
        if visited.contains(&p) {
            continue;
        }

        let mut cycle = HashSet::new();

        let mut bag = vec![p];
        while let Some(pos) = bag.pop() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);
            cycle.insert(pos);

            let c = graph.get(&pos).unwrap();

            for &(n, v) in c.0.iter().chain(c.1.iter()) {
                bag.push(n);
            }
        }

        cycles.push(cycle);
    }

    cycles
}

fn parse(n: i64, com: &[(i64, i64, bool)]) -> Graph {
    let mut graph: HashMap<_, _> = (1..=n)
        .map(|f| (f, (HashSet::new(), HashSet::new())))
        .collect();

    for &(a, b, v) in com {
        graph.get_mut(&a).unwrap().0.insert((b, v));
        graph.get_mut(&b).unwrap().1.insert((a, v));
    }

    graph
}

fn solve_one(n: i64, com: Vec<(i64, i64, bool)>) -> i64 {
    let graph = parse(n, &com);
    // println!("\n{:?}", graph);
    let cycles = find_cycles(&graph);
    // println!("{:?}", cycles);

    let mut count = 0;

    for cycle in cycles {
        let mut bag = vec![*cycle.iter().next().unwrap()];
        let mut res = cycle
            .into_iter()
            .map(|n| (n, None))
            .collect::<HashMap<_, _>>();
        res.insert(bag[0], Some(true));

        while let Some(pos) = bag.pop() {
            let (to, from) = graph.get(&pos).unwrap();

            let s = res.get(&pos).unwrap().unwrap();

            for &(n, v) in to.iter().chain(from) {
                let exp = !(s ^ v);
                match res.get(&n).unwrap() {
                    Some(v) => {
                        if *v != exp {
                            return -1;
                        };
                    }
                    None => {
                        res.insert(n, Some(exp));
                        bag.push(n);
                    }
                }
            }
        }
        let c = res.values().filter(|x| x.unwrap()).count();
        count += max(c, res.len() - c) as i64;
    }
    count
}

pub fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    for _ in 0..sc.read() {
        let (n, m) = (sc.read(), sc.read());
        let com = (0..m)
            .map(|_| (sc.read(), sc.read(), sc.read::<String>() == "crewmate"))
            .collect();
        let ans = solve_one(n, com);
        sc.writeln(ans);
    }
}
