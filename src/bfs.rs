// bfs.rs: Implements Breadth-First Search (BFS) to find the shortest path between two nodes in the graph

use std::collections::{HashSet, VecDeque};

use crate::graph::Graph;

// Finds shortest path between start and goal nodes using BFS.
pub fn shortest_path(graph: &Graph, start: &str, goal: &str) -> Option<Vec<String>> {
    if start == goal {
        return Some(vec![start.to_string()]);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut came_from = std::collections::HashMap::new();

    visited.insert(start.to_string());
    queue.push_back(start.to_string());

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.neighbors(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                    came_from.insert(neighbor.clone(), current.clone());

                    if neighbor == goal {
                        // Reconstruct the path.
                        let mut path = vec![goal.to_string()];
                        let mut prev = goal;
                        while let Some(c) = came_from.get(prev) {
                            path.push(c.clone());
                            prev = c;
                        }
                        path.reverse();
                        return Some(path);
                    }
                }
            }
        }
    }

    // No path found.
    None
}
