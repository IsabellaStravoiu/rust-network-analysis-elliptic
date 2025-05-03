//Graph.rs: Defines the Graph struct and methods for building and analayzing transaction network using adjancey list representation. 
//Functions to allow for adding edges, computing node and edge counts, degree distribution, and finding hubs are all located here.

use std::collections::{HashMap, HashSet};
use std::collections::BTreeMap;

// Graph struct:
// This struct represents graphof bitcoin wallets as nodes and the transactions are the edges.
pub struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    // New empty graph is created to be able to graph with an empty adjacency list.
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Adds an undirected edge between two nodes
    // Inputs: node1, node2: wallet addresses  Outputs: None
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

    // Returns the total number of nodes in the graph
    // Outputs: usize 
    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    // Returns the total number of edges in the graph. 
    // Outputs: usize 
    // Sums all neighbor counts and divides in half
    pub fn edge_count(&self) -> usize {
        self.adjacency_list.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
    }

    // Returns reference to the neighbor set for a specific node 
    pub fn neighbors(&self, node: &str) -> Option<&HashSet<String>> {
        self.adjacency_list.get(node)
    }

    // Computes the degree distribution 
    pub fn degree_distribution(&self) -> BTreeMap<usize, usize> {
        let mut distribution = BTreeMap::new();

        for neighbors in self.adjacency_list.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }

        distribution
    }
    //Finds the top-k most connected 
    pub fn top_k_nodes(&self, k: usize) -> Vec<(String, usize)> {
        let mut node_degrees: Vec<(String, usize)> = self.adjacency_list
            .iter()
            .map(|(node, neighbors)| (node.clone(), neighbors.len()))
            .collect();
    
        node_degrees.sort_by(|a, b| b.1.cmp(&a.1));
        node_degrees.truncate(k);
        node_degrees
    }

    //Returns list of all node names
    pub fn nodes(&self) -> Vec<String> {
        self.adjacency_list.keys().cloned().collect()
    }

}

