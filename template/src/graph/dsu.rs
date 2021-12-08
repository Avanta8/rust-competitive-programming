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
        if sa == sb {
            return false;
        }

        if self.size[sa] < self.size[sb] {
            std::mem::swap(&mut sa, &mut sb);
        }
        self.parent[sb] = sa;
        self.size[sa] += self.size[sb];

        true
    }
}
