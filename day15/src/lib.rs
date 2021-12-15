extern crate boardlib;
use boardlib::{Board, BoardCoordinate, BoardTraversable};
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub use filelib::load;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RiskNode {
    value: u32,
    coordinate: BoardCoordinate,
}

impl Ord for RiskNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return other
            .coordinate
            .x
            .cmp(&self.coordinate.x)
            .then_with(|| self.coordinate.y.cmp(&other.coordinate.y))
            .then_with(|| self.value.cmp(&other.value));
    }
}

impl PartialOrd for RiskNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathFindingState {
    cost: u32,
    node: RiskNode,
}

impl Ord for PathFindingState {
    fn cmp(&self, other: &Self) -> Ordering {
        return other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node));
    }
}

impl PartialOrd for PathFindingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

struct DijkstraGraphMap {
    adjacencies: FxHashMap<RiskNode, Vec<RiskNode>>,
}

impl DijkstraGraphMap {
    fn new() -> Self {
        return Self {
            adjacencies: FxHashMap::default(),
        };
    }

    fn add_str_board(&mut self, input: String) -> (RiskNode, RiskNode) {
        let lines: Vec<&str> = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        let width = lines.first().unwrap().len();
        let height = lines.len();
        let values: Vec<String> = lines
            .into_iter()
            .map(|x| x.chars().map(|y| y.to_string()).collect::<Vec<String>>())
            .flatten()
            .collect();
        let num_values: Vec<u32> = values
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let board: Board<u32> = Board::new(width, height, num_values);

        for cur_coord in board.coord_iter() {
            let cur_value = board.get_value(cur_coord).unwrap();
            let cur_node = RiskNode {
                value: cur_value,
                coordinate: cur_coord,
            };
            for adj_coord in board.get_adjacent_coordinates(cur_coord) {
                let adj_value = board.get_value(adj_coord).unwrap();
                let adj_node = RiskNode {
                    value: adj_value,
                    coordinate: adj_coord,
                };
                self.add_directed_edge(&cur_node, &adj_node);
            }
        }
        // return top left and bottom right Risk Nodes for further operations
        let top_left_coord = BoardCoordinate::new(0, 0);
        let top_left = RiskNode {
            value: board.get_value(top_left_coord).unwrap(),
            coordinate: top_left_coord,
        };
        let bottom_right_coord = BoardCoordinate::new(width - 1, height - 1);
        let bottom_right = RiskNode {
            value: board.get_value(bottom_right_coord).unwrap(),
            coordinate: bottom_right_coord,
        };
        return (top_left, bottom_right);
    }

    fn add_directed_edge(&mut self, src: &RiskNode, dst: &RiskNode) {
        self.adjacencies
            .entry(*src)
            .or_insert_with(Vec::new)
            .push(*dst);
    }

    fn adj(&self, node: &RiskNode) -> impl Iterator<Item = &RiskNode> {
        return self.adjacencies[node].iter();
    }

    fn shortest_path_cost(&self, start: &RiskNode, end: &RiskNode) -> u32 {
        let mut priority_queue = BinaryHeap::new();
        // dist[node] = current shortest distance from 'start' to 'node'
        let mut dist: FxHashMap<RiskNode, u32> = FxHashMap::default();
        for key in self.adjacencies.keys() {
            dist.insert(*key, u32::MAX);
        }

        *dist.entry(*start).or_insert(0) = start.value;
        priority_queue.push(PathFindingState {
            cost: 0,
            node: *start,
        });

        while let Some(PathFindingState { cost, node }) = priority_queue.pop() {
            if node == *end {
                return cost;
            }
            if cost > dist[&node] {
                continue;
            }

            // For each node we can reach, see if we can find a lower cost going through this node
            for adj_node in self.adj(&node) {
                let new_cost = cost + adj_node.value;
                if new_cost < dist[adj_node] {
                    priority_queue.push(PathFindingState {
                        cost: new_cost,
                        node: *adj_node,
                    });
                    *dist.entry(*adj_node).or_insert(0) = new_cost;
                }
            }
        }

        return 0;
    }
}

/// Solve shortest path from top left to bottom right
///
/// ```
/// let risks = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581".to_string();
/// assert_eq!(day15::puzzle_a(risks), 40);
/// ```
pub fn puzzle_a(lines: String) -> u32 {
    let mut graph = DijkstraGraphMap::new();
    let (start, end) = graph.add_str_board(lines);
    return graph.shortest_path_cost(&start, &end);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph() -> DijkstraGraphMap {
        let risks = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581".to_string();
        let mut graph = DijkstraGraphMap::new();
        graph.add_str_board(risks);

        return graph;
    }

    #[test]
    fn test_shortest_path() {
        let graph = make_graph();
        let start = RiskNode {
            value: 1,
            coordinate: BoardCoordinate::new(0, 0),
        };
        let end = RiskNode {
            value: 1,
            coordinate: BoardCoordinate::new(9, 9),
        };
        assert_eq!(graph.shortest_path_cost(&start, &end), 40);
    }
}
