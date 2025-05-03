// Module Header
// main.rs: Main for Bitcoin transaction network analysis project
// Will read CSV data, build graph, and call analysis function

use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

mod graph;
mod bfs;

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "txId1")]
    tx_id1: String,
    #[serde(rename = "txId2")]
    tx_id2: String,
}

// Read the CSV and will return vector of the transactions
fn read_csv() -> Result<Vec<Transaction>, Box<dyn Error>> {
    let file = File::open("data/txs_edgelist.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut transactions = Vec::new();

    for result in rdr.deserialize() {
        let record: Transaction = result?;
        transactions.push(record);
    }

    Ok(transactions)
}

fn main() -> Result<(), Box<dyn Error>> {
    // #1 Read CSV
    let transactions = read_csv()?;
    println!("Loaded {} transactions", transactions.len());

    // #2 Build graph
    let mut graph = graph::Graph::new();
    for tx in &transactions {
        graph.add_edge(&tx.tx_id1, &tx.tx_id2);
    }
    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());

    let degree_distribution = graph.degree_distribution();
    println!("Degree distribution (degree → count):");
    for (degree, count) in degree_distribution.iter() {
         println!("{} → {}", degree, count);
        }
    // Step 4: Identify top-10 nodes
    let top_nodes = graph.top_k_nodes(10);
    println!("Top 10 most connected nodes:");
    for (node, degree) in top_nodes {
        println!("Node: {}, Degree: {}", node, degree);
        }

    // #3 Run BFS or analysis
    let shortest_path = bfs::shortest_path(&graph, &transactions[0].tx_id1, &transactions[0].tx_id2);
    println!("Shortest path between first two nodes: {:?}", shortest_path);

    Ok(())
}
