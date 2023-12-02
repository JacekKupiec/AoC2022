use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

#[derive(Debug)]
struct Edge {
    name: String,
    distance: i32
}

#[derive(Debug)]
struct Vertex {
    rate: i32,
    name: String,
    neighbours: Vec<Edge>
}

/*
Distance from each valve to any neighbour is always 1 minute.
To open a valve it takes always 1 minute as well.
 */

fn bfs(graph: &HashMap<String, Vertex>, name: &str) -> Vec<(String, i32)> {
    let start_vertex_name = name.to_owned();
    let mut distances = vec![(start_vertex_name, 0)];
    let mut fifo = VecDeque::from([(name, 0)]);
    let mut visited = HashSet::from([name]);

    while !fifo.is_empty() {
        let (vertex_name, distance) = fifo.pop_front().unwrap();
        let vertex = graph.get(vertex_name).unwrap();

        for neighbour in &vertex.neighbours {
            if !visited.contains(neighbour.name.as_str()) {
                let distance_pair = (neighbour.name.clone(), distance + 1);
                distances.push(distance_pair);

                let fifo_pair = (neighbour.name.as_str(), distance + 1);
                fifo.push_back(fifo_pair);

                visited.insert(&neighbour.name);
            }
        }
    }

    return distances;
}

fn get_distances(graph: &HashMap<String, Vertex>) -> HashMap<(String, String), i32> {
    let mut distances: HashMap<(String, String), i32> = HashMap::new();

    for (_, vertex) in graph {
        let distances_from_vertex = bfs(graph, vertex.name.as_str());

        for (distance_to, distance) in distances_from_vertex {
            let distance_from = vertex.name.clone();
            distances.insert((distance_from, distance_to), distance);
        }
    }

    return distances;
}

fn dfs(graph: &HashMap<String, Vertex>, neighbours: &Vec<String>, distances: &HashMap<(String, String), i32>, visited: &mut HashSet<String>, start: &str, time: i32) -> i32 {
    if time == 0 {
        return 0;
    }

    let mut results = Vec::new();

    for n in neighbours {
        let key = (start.to_string(), n.to_string());
        let time_left = time - distances[&key] - 1;
        let vertex_rate = graph[n].rate;

        if !visited.contains(n) && time_left >= 0 {
            visited.insert(n.to_string());
            let result = time_left* vertex_rate + dfs(graph, neighbours, distances, visited, n, time_left);
            visited.remove(n);
            results.push(result);
        }
    }

    if let Some(max_result) = results.iter().max() {
        *max_result
    } else {
        0
    }
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut graph: HashMap<String, Vertex> = HashMap::new();


    // read the input graph
    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        // EOF when 0 bytes read
        if bytes_read == 0 {
            break;
        }

        let trimmed_buffer = buffer.trim_end();
        let (vertex_info, neighbourhood_info) = trimmed_buffer
            .split_once("; tunnels lead to valves ")
            .or_else(|| trimmed_buffer.split_once("; tunnel leads to valve "))
            .unwrap();
        let mut vertex_name = String::new();
        let mut rate = 0i32;

        let _ = sscanf!(vertex_info, "Valve {} has flow rate={}", vertex_name, rate);
        let edges: Vec<Edge> = neighbourhood_info.split(", ").map(|s| Edge {
            name: String::from(s),
            distance: 1
        })
        .collect();

        let vertex = Vertex {
            rate,
            name: vertex_name.clone(),
            neighbours: edges,
        };

        graph.insert(vertex_name, vertex);

        buffer.clear();
    }

    let distances = get_distances(&graph);
    let neighbours: Vec<_> = graph.values()
        .filter_map(|v| if v.rate > 0 { Some(v.name.clone()) } else { None })
        .collect();
    let mut visited = HashSet::new();
    let result = dfs(&graph, &neighbours, &distances, &mut visited,"AA", 30);

    println!("{}", result);
}
