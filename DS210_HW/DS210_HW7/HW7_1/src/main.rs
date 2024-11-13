mod graph;
use crate::graph::Graph;

fn main() {
    println!("Hello, world!");
    let graph_example = Graph::read_file("pagerank_data.txt");
    let pageranks = compute_pagerank(&graph_example, 80, 80);
    top_five(pageranks, 5);
    //let mut graph = Graph::new();
}

use std::collections::HashMap;
use std::fs::File;
use rand::{Rng, thread_rng};
//use rand::thread_rng;
use rand::seq::SliceRandom;

//graph running in graph.rs

pub fn compute_pagerank(graph: &Graph, passed_num: usize, steps: usize) -> HashMap<usize, usize> {
    let mut pagerank_counter = HashMap::new();
    //lecture 12 (thread_rng)
    let mut rng = thread_rng();
    for nth_vertex in 0..graph.node_and_vertices {
        for _ in 0..passed_num {
            let mut current_vertex = nth_vertex;
            for _ in 0..steps {
                if let Some(neighbors) = graph.adjacency_list.get(&current_vertex) {
                    if !neighbors.is_empty() && rng.gen::<f64>() < 0.9 {
                        current_vertex = neighbors[rng.gen_range(0..neighbors.len())]
                        //nth_vertex
                        //*neighbors[rng.gen_range(0..neighbors.len())]
                        //.as_slice().choose(&mut rng).unwrap();
                    } else {
                        current_vertex = rng.gen_range(0..graph.node_and_vertices);
                    }
                } else {
                    current_vertex = rng.gen_range(0..graph.node_and_vertices);
                }
            }
            *pagerank_counter.entry(current_vertex).or_insert(0) += 1;
        }
    }
    pagerank_counter
}

pub fn top_five(pageranks: HashMap<usize, usize>, top_node: usize) {
    let mut pageranks_vec: Vec<_> = pageranks.iter().collect();
    pageranks_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (vertex, counts) in pageranks_vec.iter().take(top_node) {
        let pagerank = *(*counts) as f64 / (80.0 * pageranks.len() as f64);
        println!("Vertex {}: approximate PageRank {:.3}", vertex, pagerank);
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pagerank() {
        let graph_example = Graph::read_file("pagerank_data.txt");
        let pageranks = compute_pagerank(&graph_example, 5, 5);

        println!("{:?}", pageranks);
        assert!(pageranks.get(&0).unwrap_or(&0) > &0);
        /*let do_positive_pr = pagerank.values().any(|&v| v > 0);
        assert!(do_positive_pr);*/
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_read_file() {
        let graph = Graph::read_file("pagerank_data.txt");
        assert!(graph.node_and_vertices > 0, "Graph need vertices.");
        assert!(!graph.adjacency_list.is_empty(), "Empty adjacency list found.");
    }

    #[test]
    fn test_basic_pagerank() {
        let graph = Graph::read_file("pagerank_data.txt");
        let pageranks = compute_pagerank(&graph, 5, 5);
        assert!(!pageranks.is_empty(), "PageRank can't be empty");
    }
}