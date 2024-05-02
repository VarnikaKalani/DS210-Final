use std::fs::File;
use std::io::{self, BufRead};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn load_csv(file_path: &str, size: u32) -> Result<Vec<Vec<u32>>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut adjacency_matrix = vec![vec![size, 0]; size.try_into().unwrap()];

    for line in reader.lines() {
        let row: Vec<u32> = line?
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        adjacency_matrix.push(row);
    }

    Ok(adjacency_matrix)
}

fn select_random_sample<T: Clone>(nodes: &[T], sample_size: usize) -> Vec<T> {
    let mut rng = thread_rng();
    nodes.choose_multiple(&mut rng, sample_size).cloned().collect()
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

        for &(neighbor, weight) in &graph[current_node] {
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
    let file_path = "./contacts-dublin.csv";
    match load_csv(file_path, 10972) {
        Ok(adjacency_matrix) => {
            let num_nodes = adjacency_matrix.len();
            let sample_size = (num_nodes as f64 * 0.1) as usize; // 10% sample size
            let sample_nodes = select_random_sample(&(0..num_nodes).collect::<Vec<_>>(), sample_size);
            
            let mut distance_lists: Vec<Vec<u32>> = Vec::new();
            for &node in &sample_nodes {
                let distances = dijkstra(&adjacency_matrix, node);
                distance_lists.push(distances);
            }
        }
        Err(err) => {
            eprintln!("Error loading CSV file: {}", err);
        }
    }
}
