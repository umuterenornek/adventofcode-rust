advent_of_code::solution!(10);
use petgraph::algo::{all_simple_paths, dijkstra};
use petgraph::graph::{Edge, Node, NodeIndex, UnGraph};
use petgraph::visit::IntoNodeReferences;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeType {
    DOT,        // .
    START,      // S
    UPLEFT,     // 7
    UPRIGHT,    // F
    DOWNLEFT,   // J
    DOWNRIGHT,  // L
    HORIZONTAL, // -
    VERTICAL,   // |
    END,        // E
}
impl PipeType {
    fn accepts_from_west(&self) -> Vec<PipeType> {
        match self {
            PipeType::START => vec![
                PipeType::START,
                PipeType::HORIZONTAL,
                PipeType::UPRIGHT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::UPLEFT => vec![
                PipeType::START,
                PipeType::HORIZONTAL,
                PipeType::UPRIGHT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::UPRIGHT => vec![PipeType::START, PipeType::DOWNLEFT, PipeType::DOWNRIGHT],
            PipeType::DOWNLEFT => vec![PipeType::START, PipeType::HORIZONTAL, PipeType::UPRIGHT],
            PipeType::DOWNRIGHT => vec![],
            PipeType::HORIZONTAL => vec![
                PipeType::START,
                PipeType::HORIZONTAL,
                PipeType::UPRIGHT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::VERTICAL => vec![],
            PipeType::END => vec![
                PipeType::START,
                PipeType::HORIZONTAL,
                PipeType::UPRIGHT,
                PipeType::DOWNRIGHT,
            ],
            _ => vec![],
        }
    }
    fn accepts_from_south(&self) -> Vec<PipeType> {
        match self {
            PipeType::START => vec![
                PipeType::START,
                PipeType::VERTICAL,
                PipeType::DOWNLEFT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::UPLEFT => vec![
                PipeType::START,
                PipeType::VERTICAL,
                PipeType::DOWNLEFT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::UPRIGHT => vec![
                PipeType::START,
                PipeType::VERTICAL,
                PipeType::DOWNLEFT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::DOWNLEFT => vec![],
            PipeType::DOWNRIGHT => vec![],
            PipeType::HORIZONTAL => vec![],
            PipeType::VERTICAL => vec![
                PipeType::START,
                PipeType::VERTICAL,
                PipeType::DOWNLEFT,
                PipeType::DOWNRIGHT,
            ],
            PipeType::END => vec![
                PipeType::START,
                PipeType::VERTICAL,
                PipeType::DOWNLEFT,
                PipeType::DOWNRIGHT,
            ],
            _ => vec![],
        }
    }
}
impl From<char> for PipeType {
    fn from(c: char) -> Self {
        match c {
            '.' => PipeType::DOT,
            'S' => PipeType::START,
            '7' => PipeType::UPLEFT,
            'F' => PipeType::UPRIGHT,
            'J' => PipeType::DOWNLEFT,
            'L' => PipeType::DOWNRIGHT,
            '-' => PipeType::HORIZONTAL,
            '|' => PipeType::VERTICAL,
            'E' => PipeType::END,
            _ => panic!("Invalid pipe type"),
        }
    }
}

#[derive(Debug, Clone)]
struct PipeNode {
    pipe_type: PipeType,
    accepts_from_south: Vec<PipeType>,
    accepts_from_west: Vec<PipeType>,
    y_index: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph = UnGraph::<PipeNode, u32>::new_undirected();
    let mut start_index = 0;
    let mut start_y_index = 0;
    let x_len = input.lines().nth(0).unwrap().len();
    let mut y_index = 0;
    for line in input.lines() {
        for c in line.chars() {
            let pipe_type = PipeType::from(c);
            let accepts_from_south = pipe_type.accepts_from_south();
            let accepts_from_west = pipe_type.accepts_from_west();
            let node = PipeNode {
                pipe_type,
                accepts_from_south,
                accepts_from_west,
                y_index,
            };
            graph.add_node(node);
            if c == 'S' {
                start_index = graph.node_count() - 1;
                start_y_index = y_index;
            }
        }
        y_index += 1;
    }
    let node_arr = graph
        .node_references()
        .map(|n| n.1.clone())
        .collect::<Vec<PipeNode>>();
    // let potential_end_edges: Vec<(NodeIndex, NodeIndex)> = Vec::new();
    for (i, cur_node) in node_arr.iter().enumerate() {
        if cur_node.pipe_type == PipeType::DOT {
            continue;
        }
        // println!("-------------------");
        // println!("cur_node: {:?}", cur_node);
        if cur_node.y_index > 0 {
            let north_node = &node_arr.get(i - x_len).unwrap();
            // println!("north_node: {:?}", north_node);
            if north_node.accepts_from_south.contains(&cur_node.pipe_type) {
                graph.update_edge(NodeIndex::new(i), NodeIndex::new(i - x_len), 1);
                // println!("edge {:?} -> {:?} added", i, i + 1);
            }
        }
        if i < node_arr.len() - 1 {
            let east_node = &node_arr.get(i + 1).unwrap();
            // println!("east_node: {:?}", east_node);
            if east_node.accepts_from_west.contains(&cur_node.pipe_type) {
                graph.update_edge(NodeIndex::new(i), NodeIndex::new(i + 1), 1);
                // println!("edge {:?} -> {:?} added", i, i + 1);
            }
        }
    }
    let path_lens = all_simple_paths::<Vec<_>, _>(
        &graph,
        NodeIndex::new(start_index),
        NodeIndex::new(start_index),
        0,
        None
    ).collect::<Vec<_>>();
    let dijkstra_path_lens = dijkstra(&graph, NodeIndex::new(start_index), None, |_| 1);
    // println!("{:?}", path_lens);
    // println!("{:?}", dijkstra_path_lens);
    // println!("{}", start_index);
    println!("{:?}", dijkstra_path_lens.values());

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
