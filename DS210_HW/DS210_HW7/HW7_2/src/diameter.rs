use crate::tree::Tree;
use std::collections::VecDeque;

pub fn bfs(tree: &Tree, start_node: usize) -> (usize, i32) {
    let mut queue = VecDeque::new();
    //VecDeque<Vertex> 
    let mut traveled = vec![false; tree.adjacency_list.len()];
    queue.push_back((start_node, 0));
    traveled[start_node] = true;

    let mut farthest_node = start_node;
    let mut farthest_dist = 0;

    while let Some((node, dist)) = queue.pop_front() {
        if dist > farthest_dist {
            farthest_dist = dist;
            farthest_node = node;
        }

        for &child_node in &tree.adjacency_list[node] {
            if !traveled[child_node] {
                traveled[child_node] = true;
                queue.push_back((child_node, dist + 1));
            }
        }
    }
    (farthest_node, farthest_dist)
}

pub fn diameter(tree: &Tree, root: usize) -> i32 {
    let (farthest_node, _) = bfs(tree, root);
    let(_, diameter) = bfs(tree, farthest_node);
    diameter
}