// main.rs: Main for Bitcoin transaction network analysis project
// Will read CSV data, build graph, and call analysis function

use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

fn calculate_average_shortest_path(graph: &graph::Graph, sample_size: usize) -> Option<f64> {
    let mut rng = thread_rng();
    let nodes = graph.nodes();
    let mut pairs: Vec<(String, String)> = Vec::new();


    // Randomly select node pairs
    for _ in 0..sample_size {
        let node1 = nodes.choose(&mut rng)?;
        let node2 = nodes.choose(&mut rng)?;
        if node1 != node2 {
            pairs.push((node1.clone(), node2.clone()));
        }
    }

    let mut total_length = 0;
    let mut count = 0;

    for (start, end) in pairs {
        if let Some(path) = bfs::shortest_path(graph, &start, &end) {
            total_length += path.len() - 1; // edges, not nodes
            count += 1;
        }
    }

    if count > 0 {
        Some(total_length as f64 / count as f64)
    } else {
        None
    }
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
    println!("Degree distribution:");
    for (degree, count) in degree_distribution.iter() {
         println!("{} → {}", degree, count);
        }
    // #3 Identify top-10 nodes
    let top_nodes = graph.top_k_nodes(10);
    println!("Top 10 most connected nodes:");
    for (node, degree) in top_nodes {
        println!("Node: {}, Degree: {}", node, degree);
        }

    if let Some(avg_length) = calculate_average_shortest_path(&graph, 1000) {
        println!("Average shortest path length (six degrees estimate): {:.2}", avg_length);
        } else {
            println!("Could not compute average shortest path (no valid pairs found)");
                }
        

    // #4 Run BFS/analysis
    let shortest_path = bfs::shortest_path(&graph, &transactions[0].tx_id1, &transactions[0].tx_id2);
    println!("The Shortest Path in between Nodes: {:?}", shortest_path);

    Ok(())
}
