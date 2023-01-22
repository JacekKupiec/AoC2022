use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Edge {
    weight: u32,
    target: usize
}

#[derive(Debug, PartialEq, Eq)]
struct PriorityQueueItem {
    distance: u32,
    vertex: usize
}

// remember that the Heap is maximizing by default, not minimizing

impl PartialOrd for PriorityQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.distance.cmp(&other.distance).reverse())
    }
}

impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

const E_CODE: u8 = b'E';
const S_CODE: u8 = b'S';

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<u8>> = reader.lines()
        .map(|line| {
            line.unwrap()
                .trim_end()
                .bytes()
                .collect()
            })
        .collect();

    let (source, target) = find_start_end_vertices(&map);

    map.iter_mut().flatten().for_each(|x| {
        *x = match *x {
            c @ b'a'..=b'z' => c,
            b'S' => b'a',
            b'E' => b'z',
            _ => panic!("Character out of range {}", *x as char)
        }
    });

    let graph = build_graph(&map);

    // solution part 1
    // let result = calculate_distance(&graph, source, target);

    // solution part 2 - naive aproach
    let result = map.iter().flatten().enumerate().filter_map(|(idx, height)| {
        if *height == b'a' {
            let distance = calculate_distance(&graph, idx, target);
            Some(distance)
        } else {
            None
        }
    })
    .min()
    .unwrap();

    // best solution: https://www.reddit.com/r/adventofcode/comments/zjnruc/2022_day_12_solutions/

    println!("{}", result);
}

fn build_graph(board: &Vec<Vec<u8>>) -> Vec<Vec<Edge>> {
    let mut graph = Vec::new();
    let rows_num = board.len();
    let columns_num = board[0].len();

    let make_edge = |row_idx, col_idx| {
        Edge {
            weight: 1,
            target: row_idx*columns_num + col_idx
        }
    };

    for y_idx in 0..rows_num {
        for x_idx in 0..columns_num {
            let mut neighbours = Vec::with_capacity(4);
            let current_height = board[y_idx][x_idx];

            if x_idx > 0 && current_height + 1 >= board[y_idx][x_idx - 1] {
                neighbours.push(make_edge(y_idx, x_idx - 1));
            }

            if x_idx + 1 < columns_num && current_height + 1 >= board[y_idx][x_idx + 1] {
                neighbours.push(make_edge(y_idx, x_idx + 1));
            }

            if y_idx > 0 && current_height + 1 >= board[y_idx - 1][x_idx] {
                neighbours.push(make_edge(y_idx - 1, x_idx));
            }

            if y_idx + 1 < rows_num && current_height + 1 >= board[y_idx + 1][x_idx] {
                neighbours.push(make_edge(y_idx + 1, x_idx));
            }

            graph.push(neighbours);
        }
    }

    return graph;
}

fn find_start_end_vertices(board: &Vec<Vec<u8>>) -> (usize, usize) {
    let mut start_position = 0;
    let mut end_position = 0;
    let rows_count = board.len();
    let columns_count = board[0].len();

    for row_idx in 0..rows_count {
        for column_idx in 0..columns_count {
            if board[row_idx][column_idx] == S_CODE {
                start_position = row_idx*columns_count + column_idx;
            }

            if board[row_idx][column_idx] == E_CODE {
                end_position = row_idx*columns_count + column_idx;
            }
        }
    }

    return (start_position, end_position);
}

fn calculate_distance(graph: &Vec<Vec<Edge>>, source: usize, target: usize) -> u32 {
    let mut distances = vec![u32::MAX; graph.len()];
    let mut visit_statuses = vec![false; graph.len()];
    let mut priority_queue = BinaryHeap::with_capacity(graph.len());

    distances[source] = 0;
    priority_queue.push(PriorityQueueItem {
        distance: 0,
        vertex: source
    });

    while !priority_queue.is_empty() {
        let current_vertex = priority_queue.pop().expect("Pop must work on not empty queue");

        if visit_statuses[current_vertex.vertex] {
            continue;
        }

        if current_vertex.vertex == target {
            return current_vertex.distance;
        }

        visit_statuses[current_vertex.vertex] = true;

        for edge in &graph[current_vertex.vertex] {
            let current_distance = distances[edge.target];
            let new_distance = distances[current_vertex.vertex] + edge.weight;

            distances[edge.target] = min(current_distance, new_distance);

            if !visit_statuses[edge.target] {
                priority_queue.push(PriorityQueueItem {
                    distance: distances[edge.target],
                    vertex: edge.target
                });
            }
        }
    }

    return u32::MAX;
}