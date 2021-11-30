#[derive(Debug)]
pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn is_ancestor(&self, v: usize) -> bool {
        self.parent[v] == v
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.get(a) == self.get(b)
    }

    pub fn get(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            return v;
        }

        self.parent[v] = self.get(self.parent[v]);
        self.parent[v]
    }

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut sa = self.get(a);
        let mut sb = self.get(b);
        if sa != sb {
            if self.size[sa] < self.size[sb] {
                std::mem::swap(&mut sa, &mut sb);
            }
            self.parent[sb] = sa;
            self.size[sa] += self.size[sb];
        }

        sa != sb
    }
}

fn main() {
    let mut dsu = Dsu::new(11);

    let cn = [
        (10, 8),
        (1, 2),
        (2, 3),
        (3, 4),
        (1, 4),
        (6, 7),
        (8, 9),
        (8, 10),
        (1, 4),
    ];

    println!("{:?}", dsu);

    for (a, b) in cn {
        dsu.unite(a, b);
    }

    println!("{:?}", dsu);

    for i in 1..=10 {
        println!("{}: {}", i, dsu.get(i));
    }

    println!("{:?}", dsu);
}
