use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

struct Edge {
    name: String,
    distance: i32
}

struct Vertex {
    rate: i32,
    name: String,
    neighbours: Vec<Edge>
}

/*
Distance from each valve to any neighbour is always 1 minute.
To open a valve it takes always 1 minute as well.
 */

fn main() {
    let path = "E:\\source\\Rust\\AoC 2022\\day_16\\input.txt";
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

        let (vertex_info, neighbourhood_info) = buffer
            .trim_end()
            .split_once("; tunnels lead to valves ")
            .or_else(|| buffer.split_once("; tunnel leads to valve "))
            .unwrap();
        let mut vertex_name = String::new();
        let mut rate = 0i32;

        let _ = sscanf!(vertex_info, "Valve {} has flow rate={}", vertex_name, rate);
        let edges: Vec<Edge> = neighbourhood_info.split(',').map(|s| Edge {
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
}
