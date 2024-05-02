use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex};
use std::thread;

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

fn calculate_average_distance(distance_lists: Vec<Vec<u32>>) -> f64 {
    let num_pairs = distance_lists.len() * (distance_lists.len() - 1) / 2;
    let mut total_distance: f64 = 0.0;

    for distances in distance_lists {
        for &distance in &distances {
            if distance != u32::MAX {
                total_distance += distance as f64;
            }
        }
    }

    total_distance / num_pairs as f64
}

fn select_random_sample<T: Clone>(nodes: &[T], sample_size: usize) -> Vec<T> {
    let mut rng = thread_rng();
    nodes.choose_multiple(&mut rng, sample_size).cloned().collect()
}

fn main() {
    let file_path = "./reality-call.csv";
    let graph_size = 6810;
    match load_csv(file_path, graph_size) {
        Ok(adjacency_matrix) => {
            let num_nodes = adjacency_matrix.len();
            let sample_size = (num_nodes as f64 * 0.1) as usize; // 10% sample size
            let sample_nodes = select_random_sample(&(0..num_nodes).collect::<Vec<_>>(), sample_size);

            let distance_lists: Arc<Mutex<Vec<Vec<u32>>>> = Arc::new(Mutex::new(Vec::new()));
            let handles: Vec<_> = sample_nodes.into_iter().map(|node| {
                let adjacency_matrix = adjacency_matrix.clone();
                let distance_lists = distance_lists.clone();
                thread::spawn(move || {
                    println!("Calculating distances from node {}", node);
                    let distances = dijkstra(&adjacency_matrix, node as usize);
                    let mut distance_lists = distance_lists.lock().unwrap();
                    distance_lists.push(distances);
                })
            }).collect();

            for handle in handles {
                handle.join().unwrap();
            }

            let distance_lists = distance_lists.lock().unwrap();
            let avg_distance = calculate_average_distance(distance_lists.clone());
            println!("Average distance: {}", avg_distance);

        }
        Err(err) => {
            eprintln!("Error loading CSV file: {}", err);
        }
    }
}
