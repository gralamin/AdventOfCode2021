use rustc_hash::{FxHashMap, FxHashSet};

pub use filelib::load_no_blanks;

// Doing a proper graph in Rust is HARD. So we cheat.
type CaveData = String;

#[derive(Debug)]
struct LiteGraph {
    adjacencies: FxHashMap<CaveData, Vec<CaveData>>,
}

impl LiteGraph {
    fn new() -> Self {
        return Self {
            adjacencies: FxHashMap::default(),
        };
    }

    fn add_directed_edge(&mut self, src: &CaveData, dst: &CaveData) {
        self.adjacencies
            .entry(src.to_string())
            .or_insert_with(Vec::new)
            .push(dst.to_string());
    }

    fn add_undirected_edge(&mut self, a: &CaveData, b: &CaveData) {
        self.add_directed_edge(a, b);
        self.add_directed_edge(b, a);
    }

    fn adj(&self, node: &CaveData) -> impl Iterator<Item = &CaveData> {
        return self.adjacencies[node].iter();
    }

    fn is_end(&self, node: &CaveData) -> bool {
        return node == "end";
    }

    fn is_start(&self, node: &CaveData) -> bool {
        return node == "start";
    }

    fn is_large(&self, node: &CaveData) -> bool {
        return node.to_uppercase() == *node;
    }
}

fn recurse_walk_counting_paths<'a>(
    graph: &LiteGraph,
    node: &'a CaveData,
    mut seen: FxHashSet<&'a CaveData>,
) -> usize {
    if seen.contains(&node) {
        return 0;
    }
    if graph.is_end(node) {
        return 1;
    }
    if !graph.is_large(node) {
        seen.insert(node);
    }
    return graph
        .adj(node)
        .map(|node| recurse_walk_counting_paths(graph, node, seen.clone()))
        .sum();
}

// Not happy with this solution due to time it takes
// Solution to 2: 96528
//
// real    0m1.427s
// user    0m1.427s
// sys     0m0.000s
fn recurse_walk_counting_paths_allow_a_double<'a>(
    graph: &LiteGraph,
    node: &'a CaveData,
    mut seen: FxHashSet<&'a CaveData>,
    mut double: bool,
) -> usize {
    if seen.contains(&node) {
        if double {
            return 0;
        } else {
            double = true;
        }
    }
    if graph.is_end(node) {
        return 1;
    }
    if !graph.is_large(node) {
        seen.insert(node);
    }
    return graph
        .adj(node)
        .filter(|&node| !graph.is_start(node))
        .map(|node| recurse_walk_counting_paths_allow_a_double(graph, node, seen.clone(), double))
        .sum();
}

/// Walk through all the paths visting small caves only once.
///
/// ```
/// let input = vec![
///     "fs-end".to_string(),
///     "he-DX".to_string(),
///     "fs-he".to_string(),
///     "start-DX".to_string(),
///     "pj-DX".to_string(),
///     "end-zg".to_string(),
///     "zg-sl".to_string(),
///     "zg-pj".to_string(),
///     "pj-he".to_string(),
///     "RW-he".to_string(),
///     "fs-DX".to_string(),
///     "pj-RW".to_string(),
///     "zg-RW".to_string(),
///     "start-pj".to_string(),
///     "he-WI".to_string(),
///     "zg-he".to_string(),
///     "pj-fs".to_string(),
///     "start-RW".to_string(),
/// ];
/// assert_eq!(day12::puzzle_a(&input), 226);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let mut graph = LiteGraph::new();
    for line in input.iter() {
        let (src, dst) = line.split_once("-").unwrap();
        graph.add_undirected_edge(&src.to_string(), &dst.to_string());
    }
    return recurse_walk_counting_paths(&graph, &"start".to_string(), FxHashSet::default());
}

/// Walk through all the paths visting small caves only once, except for one double
///
/// ```
/// let input = vec![
///     "fs-end".to_string(),
///     "he-DX".to_string(),
///     "fs-he".to_string(),
///     "start-DX".to_string(),
///     "pj-DX".to_string(),
///     "end-zg".to_string(),
///     "zg-sl".to_string(),
///     "zg-pj".to_string(),
///     "pj-he".to_string(),
///     "RW-he".to_string(),
///     "fs-DX".to_string(),
///     "pj-RW".to_string(),
///     "zg-RW".to_string(),
///     "start-pj".to_string(),
///     "he-WI".to_string(),
///     "zg-he".to_string(),
///     "pj-fs".to_string(),
///     "start-RW".to_string(),
/// ];
/// assert_eq!(day12::puzzle_b(&input), 3509);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let mut graph = LiteGraph::new();
    for line in input.iter() {
        let (src, dst) = line.split_once("-").unwrap();
        graph.add_undirected_edge(&src.to_string(), &dst.to_string());
    }
    return recurse_walk_counting_paths_allow_a_double(
        &graph,
        &"start".to_string(),
        FxHashSet::default(),
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cave_systems() -> (LiteGraph, CaveData, CaveData) {
        let node_start = "start".to_string();
        let node_a = "A".to_string();
        let node_b = "b".to_string();
        let node_c = "c".to_string();
        let node_d = "d".to_string();
        let node_end = "end".to_string();

        let mut cave_graph = LiteGraph::new();
        cave_graph.add_undirected_edge(&node_start, &node_a);
        cave_graph.add_undirected_edge(&node_start, &node_b);
        cave_graph.add_undirected_edge(&node_a, &node_b);
        cave_graph.add_undirected_edge(&node_a, &node_c);
        cave_graph.add_undirected_edge(&node_a, &node_end);
        cave_graph.add_undirected_edge(&node_b, &node_d);
        cave_graph.add_undirected_edge(&node_b, &node_end);

        return (cave_graph, node_start, node_end);
    }

    #[test]
    fn test_cave_is_end() {
        let (graph, start, end) = make_cave_systems();
        assert_eq!(graph.is_end(&start), false);
        assert_eq!(graph.is_end(&end), true);
    }

    #[test]
    fn test_cave_adj() {
        let (graph, start, _end) = make_cave_systems();
        let start_collected: Vec<&CaveData> = graph.adj(&start).collect();
        assert_eq!(start_collected.len(), 2);
        let node_a = graph.adj(&start).nth(0).unwrap();
        let a_collected: Vec<&CaveData> = graph.adj(&node_a).collect();
        assert_eq!(a_collected.len(), 4);
    }

    #[test]
    fn test_cave_is_large() {
        let (graph, start, _end) = make_cave_systems();
        assert_eq!(graph.is_large(&start), false);
        let node_a = graph.adj(&start).nth(0).unwrap();
        assert_eq!(graph.is_large(&node_a), true);
    }

    #[test]
    fn test_recurse_walk_counting_paths() {
        let (graph, start, _) = make_cave_systems();
        assert_eq!(
            recurse_walk_counting_paths(&graph, &start, FxHashSet::default()),
            10
        );
    }

    #[test]
    fn test_recurse_walk_counting_path_a_doubles() {
        let (graph, start, _) = make_cave_systems();
        assert_eq!(
            recurse_walk_counting_paths_allow_a_double(&graph, &start, FxHashSet::default(), false),
            36
        );
    }
}
