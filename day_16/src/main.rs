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
Optimisation tricks: https://www.youtube.com/watch?v=bLMj50cpOug
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

fn dfs<'a>(
    graph: &HashMap<String, Vertex>,
    distances: &HashMap<(String, String), i32>,
    translation: &'a HashMap<i32, String>,
    all_neighbours_count: i32,
    start: &'a str,
    time: i32,
    neighbours: i32,
    cache: &mut HashMap<(i32, &'a str, i32), i32>) -> i32
{
    if let Some(result) = cache.get(&(time, start, neighbours)) {
        return *result;
    }

    let mut results = Vec::new();

    for neighbour_idx in 0..all_neighbours_count {
        let not_visited = neighbours & (1 << neighbour_idx) != 0;

        let neighbour_name = &translation[&neighbour_idx];
        let key = (start.to_string(), neighbour_name.to_string());
        let time_left = time - distances[&key] - 1;

        if not_visited && time_left > 0 {
            let neighbours_after_pick_new_start = neighbours & !(1 << neighbour_idx);
            let vertex_rate = graph[neighbour_name].rate;

            let result = time_left*vertex_rate
                + dfs(graph,
                      distances,
                      translation,
                      all_neighbours_count,
                      neighbour_name,
                      time_left,
                      neighbours_after_pick_new_start,
                      cache);

            results.push(result);
        }
    }

    let result = if let Some(max_result) = results.iter().max() {
        *max_result
    } else {
        0
    };

    cache.insert((time, start, neighbours), result);

    return result;
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
        let edges: Vec<Edge> = neighbourhood_info.split(", ")
            .map(|s|
                Edge {
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

    let mut cache = HashMap::new();
    let distances = get_distances(&graph);
    let translation: HashMap<_,_> = graph.values()
        .filter_map(|v| if v.rate > 0 { Some(v.name.clone()) } else { None })
        .enumerate()
        .map(|(idx, name)| (idx as i32, name))
        .collect();
    let neighbours_bitmask = (1 << translation.len()) - 1;

    // Solution part 1
    let result = dfs(
        &graph,
        &distances,
        &translation,
        translation.len() as i32,
        "AA",
        30,
        neighbours_bitmask,
        &mut cache);

    println!("{}", result);

    //Solution part 2
    let result = (0..=neighbours_bitmask/2)
        .map(|combination_bitmask|
            dfs(&graph, &distances, &translation, translation.len() as i32, "AA", 26, combination_bitmask, &mut cache)
            + dfs(&graph, &distances, &translation, translation.len() as i32, "AA", 26, neighbours_bitmask ^ combination_bitmask, &mut cache))
        .max()
        .unwrap();

    println!("{}", result);
}
