// Defines the Graph struct and methods to add edges, count nodes and edges

use std::collections::{HashMap, HashSet};
use std::collections::BTreeMap;

// Graph struct that uses adjacency list
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

    // Returns  HashMap with degree counts (Each number of node is a degree)
    pub fn degree_distribution(&self) -> BTreeMap<usize, usize> {
        let mut distribution = BTreeMap::new();

        for neighbors in self.adjacency_list.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }

        distribution
    }

    pub fn top_k_nodes(&self, k: usize) -> Vec<(String, usize)> {
        let mut node_degrees: Vec<(String, usize)> = self.adjacency_list
            .iter()
            .map(|(node, neighbors)| (node.clone(), neighbors.len()))
            .collect();
    
        node_degrees.sort_by(|a, b| b.1.cmp(&a.1));
        node_degrees.truncate(k);
        node_degrees
    }
    
}

