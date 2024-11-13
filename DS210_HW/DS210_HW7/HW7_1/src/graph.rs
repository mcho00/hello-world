fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<usize>>,
    pub node_and_vertices: usize,
}

impl Graph {
    //write the functions needed
    pub fn read_file(file_txt: &str) -> Self {
        let path = Path::new(file_txt);
        let file = File::open(&path).expect("Check the file again.");
        let mut lines = io::BufReader::new(file).lines();
        //by parsing and unwrapping, return that unwrapped string to num.
        let node_and_vertices: usize = lines.next().unwrap().expect("Unavailable").parse().unwrap();
        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

        for line in lines {
            if let Ok(line) = line{
                let nodes: Vec<usize> = line.split_whitespace()
                .map(|node| node.parse().unwrap()).collect();
            adjacency_list.entry(nodes[0]).or_default().push(nodes[1]);
            }
            /*let nodes: Vec<usize> = line.trim()
            .map(|node| node.parse().unwrap()).collect();*/
        //this will store each node to the vector by collect()
            
            //adjacency_list.entry(nodes[0]).or_insert().push(parts[1]);
        }

        Graph {adjacency_list, node_and_vertices}
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_read_file() {
        let graph = Graph::read_file("pagerank_data.txt");

        // Verify that the graph has vertices and an adjacency list.
        assert!(graph.node_and_vertices > 0, "Graph should have vertices.");
        assert!(!graph.adjacency_list.is_empty(), "Graph adjacency list should not be empty.");
    }
 }*/