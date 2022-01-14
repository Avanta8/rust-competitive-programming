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
