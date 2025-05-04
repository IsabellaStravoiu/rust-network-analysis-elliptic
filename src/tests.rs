// Module Header:
// tests.rs: Provides unit tests for Graph and BFS modules using small example graphs.


#[cfg(test)]
mod tests {
    use super::super::graph::Graph;
    use super::super::bfs;
    use std::collections::HashSet;
//verify edges are added correctly 
    #[test]
    fn test_add_edge_and_counts() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    //verify nodes are added correctly
    #[test]
    fn test_neighbors() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");

        let neighbors = graph.neighbors("A").unwrap();
        let expected: HashSet<String> = ["B", "C"].iter().cloned().map(String::from).collect();

        assert_eq!(neighbors, &expected);
    }
    //verify neighbors are added correctly 
    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B");
        graph.add_edge("B", "C");

        let path = bfs::shortest_path(&graph, "A", "C");
        assert_eq!(path, Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]));
    }

    /// Test_top_k_nodes
    /// Test that top_k_nodes correctly identify the most connected wallets
    #[test]
    fn test_top_k_nodes() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("A", "D");
        graph.add_edge("B", "C");

        let top = graph.top_k_nodes(1);
        // Node A should be the top node with degree 3
        assert_eq!(top[0], ("A".to_string(), 3));
    }
}