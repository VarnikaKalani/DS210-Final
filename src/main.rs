use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn load_csv(file_path: &str, size: u32) -> Result<Vec<Vec<u32>>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut adjacency_matrix = vec![vec![0; size as usize]; size as usize];

    for line in reader.lines() {
        let line = line?;
        let mut values = line.split(",");
        let i = values.next().unwrap().parse::<u32>().unwrap();
        let j = values.next().unwrap().parse::<u32>().unwrap();
        let weight = values.next().unwrap().parse::<u32>().unwrap();
        adjacency_matrix[i as usize][j as usize] = weight;
        adjacency_matrix[j as usize][i as usize] = weight;
    }

    Ok(adjacency_matrix)
}


fn dijkstra(graph: &Vec<Vec<u32>>, source: usize) -> Vec<u32> {
    let num_nodes = graph.len();
    let mut distances = vec![u32::MAX; num_nodes];
    distances[source] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, source)));

    while let Some(Reverse((current_distance, current_node))) = priority_queue.pop() {
        if current_distance > distances[current_node] {
            continue;
        }

        for (neighbor, &weight) in graph[current_node].iter().enumerate() {
            if weight == 0 {
                continue;
            }
            let new_distance = current_distance.saturating_add(weight);
            if new_distance < distances[neighbor] {
                distances[neighbor] = new_distance;
                priority_queue.push(Reverse((new_distance, neighbor)));
            }
        }
    }

    distances
}



fn main() {
    let file_path = "./high-school-contacts.csv";
    let graph_size = 327;
    match load_csv(file_path, graph_size) {
        Ok(adjacency_matrix) => {
            let mut distance_lists = Vec::new();
            for node in 0..graph_size {
                let distances = dijkstra(&adjacency_matrix, node as usize);
                distance_lists.push(distances);
            }
            let avg_distance = calculate_average_distance(distance_lists);
            println!("Average distance: {}", avg_distance);

        }
        Err(err) => {
            eprintln!("Error loading CSV file: {}", err);
        }
    }
}
