mod graph {
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    pub fn load_adjacency_matrix(file_path: &str, size: u32) -> Result<Vec<Vec<u32>>, io::Error> {
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

    pub fn save_adjacency_matrix(file_path: &str, adjacency_matrix: &Vec<Vec<u32>>) -> Result<(), io::Error> {
        let mut file = File::create(file_path)?;
        for row in adjacency_matrix {
            let line = row.iter().map(|weight| weight.to_string()).collect::<Vec<_>>().join(",");
            file.write_all(line.as_bytes())?;
        }

        Ok(())

    }
}

mod distance_processing {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    use std::io::Write;
    /// Performs Dijkstra's algorithm to calculate the shortest distances from a source node to all other nodes in a graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The adjacency matrix representing the graph.
    /// * `source` - The index of the source node.
    ///
    /// # Returns
    ///
    /// A vector containing the shortest distances from the source node to all other nodes.
    pub fn dijkstra(graph: &Vec<Vec<u32>>, source: usize) -> Vec<u32> {
        let num_nodes = graph.len();
        let mut distances = vec![u32::MAX; num_nodes]; // Initialize distances to infinity
        distances[source] = 0; // Set the distance of the source node to 0
        
        // Create a priority queue to store nodes based on their distances
        let mut priority_queue = BinaryHeap::new(); 
        // Push the source node with distance 0 to the priority queue
        priority_queue.push(Reverse((0, source))); 
    
        while let Some(Reverse((current_distance, current_node))) = priority_queue.pop() {
            if current_distance > distances[current_node] {
                // Skip this iteration if the current distance is greater than the distance stored for the current node
                continue; 
            }
    
            for (neighbor, &weight) in graph[current_node].iter().enumerate() {
                if weight == 0 {
                    // Skip this neighbor if the weight is 0 (indicating no edge between the nodes)
                    continue;
                }
                // Calculate the new distance to the neighbor node
                let new_distance = current_distance.saturating_add(weight); 
                if new_distance < distances[neighbor] {
                    // Update the distance if the new distance is smaller
                    distances[neighbor] = new_distance; 
                    // Push the neighbor node with the new distance to the priority queue
                    priority_queue.push(Reverse((new_distance, neighbor))); 
                }
            }
        }
    
        distances // Return the vector of shortest distances
    }
    
    pub fn calculate_average_distance(distance_lists: Vec<Vec<u32>>) -> f64 {
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

    pub fn save_distance_list(file_path: &str, distance_list: &Vec<u32>) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(file_path)?;
        let line = distance_list.iter().map(|distance| distance.to_string()).collect::<Vec<_>>().join(",");
        file.write_all(line.as_bytes())?;
        Ok(())
    }
}

mod dataset_sampling {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    pub fn select_random_sample<T: Clone>(nodes: &[T], sample_size: usize) -> Vec<T> {
        let mut rng = thread_rng();
        nodes.choose_multiple(&mut rng, sample_size).cloned().collect()
    }
}

mod average_distances {
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::distance_processing;

    pub fn get_distance_lists<'a>(adjacency_matrix: &'a Vec<Vec<u32>>, sample_nodes: Vec<usize>) -> Vec<Vec<u32>> {
        let distance_lists: Arc<Mutex<Vec<Vec<u32>>>> = Arc::new(Mutex::new(Vec::new()));
        let handles: Vec<_> = sample_nodes.into_iter().map(|node| {
            let adjacency_matrix = adjacency_matrix.clone();
            let distance_lists = distance_lists.clone();
            thread::spawn(move || {
                println!("Calculating distances from node {}", node);
                let distances = distance_processing::dijkstra(&adjacency_matrix, node);
                let mut distance_lists = distance_lists.lock().unwrap();
                distance_lists.push(distances);
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let distance_lists = Arc::try_unwrap(distance_lists).unwrap().into_inner().unwrap();
        distance_lists
    }
}

fn main() {
    let file_path = "./reality-call.csv";
    let graph_size = 6810;
    match graph::load_adjacency_matrix(file_path, graph_size) {
        Ok(adjacency_matrix) => {
            let num_nodes = adjacency_matrix.len();
            let sample_size = (num_nodes as f64 * 0.1) as usize; // 10% sample size
            let sample_nodes = dataset_sampling::select_random_sample(&(0..num_nodes).collect::<Vec<_>>(), sample_size);

            let distance_lists = average_distances::get_distance_lists(&adjacency_matrix, sample_nodes);
            let avg_distance = distance_processing::calculate_average_distance(distance_lists.clone());

            graph::save_adjacency_matrix("./reality-call-adjacency-matrix.csv", &adjacency_matrix).unwrap();
            distance_processing::save_distance_list("./reality-call-distance-list.csv", &distance_lists[0]).unwrap();

            println!("Average distance: {}", avg_distance);

        }
        Err(err) => {
            eprintln!("Error loading CSV file: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance_list() {
        let result = average_distances::get_distance_lists(&vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]], vec![0, 1, 2]);
        assert_eq!(result, vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]]);
    }
    #[test]
    fn test_average_distance() {
        let distance_lists = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
        let result = distance_processing::calculate_average_distance(distance_lists);
        assert_eq!(result, 4.0);
    }
    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![0, 1, 2],
            vec![1, 0, 3],
            vec![2, 3, 0]
        ];
        let result = distance_processing::dijkstra(&graph, 0);
        assert_eq!(result, vec![0, 1, 2]);
    }
    #[test]
    fn test_select_random_sample() {
        let nodes = vec![1, 2, 3, 4, 5];
        let result = dataset_sampling::select_random_sample(&nodes, 3);
        assert_eq!(result.len(), 3);
    }
    #[test]
    fn test_load_adjacency_matrix() {
        let result = graph::load_adjacency_matrix("./reality-call.csv", 6810);
        assert!(result.is_ok());
    }
    #[test]
    fn test_save_adjacency_matrix() {
        let adjacency_matrix = vec![
            vec![0, 1, 2],
            vec![1, 0, 3],
            vec![2, 3, 0]
        ];
        let result = graph::save_adjacency_matrix("./test-adjacency-matrix.csv", &adjacency_matrix);
        assert!(result.is_ok());
    }
}