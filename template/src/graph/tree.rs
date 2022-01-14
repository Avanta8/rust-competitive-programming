use std::collections::HashSet;

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
