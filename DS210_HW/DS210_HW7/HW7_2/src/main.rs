mod tree;
mod diameter;

fn main() {
    println!("Hello, world!");
    let mut ex_tree = Tree::new(120);
    ex_tree.gen_bin_tree(120);
    
    ex_tree.gen_file("example.txt").expect("Conflict");
    println!("File written.txt");

    let loaded_tree = Tree::read("example.txt").expect("conflict");
    //println!("Tree loaded from tree.txt");

    let ex_diameter = diameter(&loaded_tree, 0);
    println!("The longest diameter is: {}", ex_diameter);

    //Ok(())
}
use std::io;
use crate::tree::Tree;
use crate::diameter::diameter;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
//lecture 14
use std::collections::VecDeque;
use std::io::BufReader;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_diameter_tree() {
        let mut tree = Tree::new(7);

        tree.add_edge(0, 1);
        tree.add_edge(0, 2);
        tree.add_edge(1, 3);
        tree.add_edge(1, 4);
        tree.add_edge(2, 5);
        tree.add_edge(5, 6);

        let diameter_test = diameter(&tree, 0);
        let expected_diameter = 5;

        assert_eq!(diameter_test, expected_diameter, 
            "Result: Expected diameter: {}, Actual: {}", 
            expected_diameter, diameter_test);
    
    }
}

//compute the longest path between two nodes in the tree
//tree: 100 nodes; each node randomly assigned 1 or 2 children
//until the tree has been fully built

//make function that writes the file,
//make function that reads the file and calculate the tree,
//and actually compute the diameter


        /*self.adjacency_list = vec![vec![]; total_nodes];*/

//from lecture 12
/*fn generate_file(path: &str, n: usize) {
    // Generate a random file of edges for vertices 0.n
    let mut file = File::create(path).expect("Unable to create file");
    for i in 0..n {
        // How many neighbors will this node have
        let rng = rand::thread_rng().gen_range(0..20) as usize;
        for _j in 0..rng {
            // Randomly select a neighbor (even with duplicates but not to ourselves)
            let neighbor = rand::thread_rng().gen_range(0..n) as usize;
            if neighbor != i {
                let s: String = format!("{} {}\n", i, neighbor);
                file.write_all(s.as_bytes()).expect("Unable to write file");
            }
        }
    }
}

//from lecture 12
fn read_file(path: &str) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines(); //take that file and bring different strings(?)
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<u32>().unwrap();
        let y = v[1].parse::<u32>().unwrap();
        result.push((x, y));
    }
    return result;
}*/