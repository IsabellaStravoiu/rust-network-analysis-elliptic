// Module Header:
// graph.rs: Defines the Graph struct and basic methods for adding edges, counting nodes and edges.

use std::collections::{HashMap, HashSet};

// One-sentence description: Graph struct using adjacency list representation.
pub struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    // Creates a new, empty Graph.
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Adds an edge between two nodes.
    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        self.adjacency_list
            .entry(node1.to_string())
            .or_insert_with(HashSet::new)
            .insert(node2.to_string());
        self.adjacency_list
            .entry(node2.to_string())
            .or_insert_with(HashSet::new)
            .insert(node1.to_string());
    }

    // Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    // Returns the number of edges.
    pub fn edge_count(&self) -> usize {
        self.adjacency_list.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
    }

    // Returns neighbors of a node.
    pub fn neighbors(&self, node: &str) -> Option<&HashSet<String>> {
        self.adjacency_list.get(node)
    }
}
