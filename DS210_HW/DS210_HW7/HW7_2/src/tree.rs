use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use rand::Rng;

pub struct Tree {
    pub adjacency_list: Vec<Vec<usize>>,
}

impl Tree {
    pub fn new(total_nodes: usize) -> Self {
        //make tree with nodes of total_nodes
        Tree { adjacency_list: vec![vec![]; total_nodes] }
    }

    pub fn add_edge(&mut self, parent_node: usize, child_node: usize) {
        self.adjacency_list[parent_node].push(child_node);
        self.adjacency_list[child_node].push(parent_node);
    }
    //integer type??

    pub fn gen_bin_tree(&mut self, total_nodes: usize) /*-> Tree*/ { //Tree is strtuct type I made
        //ONLY make sure total node should exceed 100, so ignore something below
        /*if total_nodes < 100 {
            panic!("Insert nodes larger or equal to 100");
        }*/
        //let mut tree = Tree::new();
        let mut rng = rand::thread_rng();
        //.gen_range(100..) as usize;

        //make adjacency list with total_nodes
        for i in 0..(total_nodes-1) / 2 {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;
            if left_child < total_nodes {
                self.add_edge(i, left_child);
            }
            if right_child < total_nodes {
                self.add_edge(i, right_child);
            }
        }
        for parent in 0..total_nodes {
            if self.adjacency_list[parent].len() > 0 {
            //let Some(left_child)
                let left_child = self.adjacency_list[parent][0];
                if rng.gen_bool(0.3) {
                    self.adjacency_list[parent].retain(|&child| child != left_child);
                    self.adjacency_list[left_child].retain(|&child| child != parent);
                }
            }
            if self.adjacency_list[parent].len() > 1{
            //let Some(right_child) = {
                let right_child = self.adjacency_list[parent][1];
                if rng.gen_bool(0.3) {
                    self.adjacency_list[parent].retain(|&child| child != right_child);
                    self.adjacency_list[right_child].retain(|&child| child != parent);
                }
            }
            /*if 2 * parent + 1 < total_nodes && rng.gen_bool(0.5) {
                self.add_edge(parent as usize, (2 * parent + 1) as usize);
            } //--> left child
            if 2 * parent + 2 < total_nodes && rng.gen_bool(0.5) {
                self.add_edge(parent as usize, (2 * parent + 2) as usize);
            } //--> right child*/
        }
        //tree ; no longer returning
    }
    pub fn gen_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "{}", self.adjacency_list.len())?;
        for (parent_node, child_nodes) in self.adjacency_list.iter().enumerate() {
            for &child in child_nodes {
                writeln!(file, "{}, {}", parent_node, child)?;
            }
        }
        //yellow () will be displayed
        //HOPEFULLY
        Ok(())
    }
    pub fn read(filename: &str/*, total_nodes: usize*/) -> io::Result<Self> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        //new
        let mut line = String::new();

        //read first line for total number of nodes
        buf_reader.read_line(&mut line)?;
        let total_nodes: usize = line.trim().parse().unwrap();
        //And below will make a tree based on the total_nodes
        let mut tree = Tree::new(total_nodes);
        for n_line in buf_reader.lines() {
            let n_line = n_line?;
            let parts: Vec<&str> = n_line.split(',').collect();
            //split_whitespace().collect();
            if parts.len() == 2 {
                let parent_node: usize = parts[0].trim().parse().unwrap();
                let child_node: usize = parts[1].trim().parse().unwrap();
                tree.add_edge(parent_node, child_node);
            }
        }
        Ok(tree)
    }
}