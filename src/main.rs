// main.rs: Main for Bitcoin transaction network analysis project
// In this main it will read CSV data, build graph of bitcoin wallets, run analysis function, and generate bar chart

use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use rand::seq::SliceRandom;
use rand::thread_rng;
use plotters::prelude::*;

#[cfg(test)]
mod tests;

mod graph;
mod bfs;

// Transaction Struct: Will represent bitcoin transaction between wallet addresses
#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "txId1")]
    tx_id1: String,
    #[serde(rename = "txId2")]
    tx_id2: String,
}

// Read CSV file and sort through the vector of Transaction Struct.
//Inputs: None Outputs: Result<Vec<Transaction> and Box<dyn Error>
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
// Calculate shortest length between two random Bitcoin wallets. 
// Input: A reference to the graph and number of random samples Output: Average path length or none if there are no valid points
// This function will randomly select pairs and run BFS. Then it will average the path lengths to return the average path length.
fn calculate_average_shortest_path(graph: &graph::Graph, sample_size: usize) -> Option<f64> {
    let mut rng = thread_rng();
    let nodes = graph.nodes();
    let mut pairs: Vec<(String, String)> = Vec::new();

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

//Plot Degree Distribution 
//This function will create and save a historgram representing the Bitcoin wallet degree distribution 
//Inputs: degree distribution count Outputs: The graph saved as the title described 
//Will plot if degrees are less than or equal to 30, also specified labels for diagram
fn plot_degree_distribution(degree_distribution: &std::collections::BTreeMap<usize, usize>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("degree_distribution.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_degree = 30;
    let max_count = degree_distribution.iter()
    .filter(|(deg, _)| **deg <= 30)
    .map(|(_, count)| *count)
    .max()
    .unwrap_or(0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Distribution", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0usize..max_degree, 0usize..(max_count + 10))?;


    chart.configure_mesh().draw()?;

    chart.draw_series(
        degree_distribution.iter()
            .filter(|(deg, _)| **deg <= 30)
            .map(|(degree, count)| {
                Rectangle::new(
                    [(*degree, 0), (*degree + 1, *count)],
                    BLUE.filled(),
                )
            }),
    )?;
    

    Ok(())
}

//Main: First loads transaction data, builds graphs, calcuates and displays degree distribution, and Identifies top 10 most connected wallets.
//Then will estimate shortest path, compute specific path between first two nodes, and finally generates the distribution plot.
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

    // #3 Degree Distribution
    let degree_distribution = graph.degree_distribution();
    println!("Degree distribution:");
    for (degree, count) in degree_distribution.iter() {
         println!("{} → {}", degree, count);
        }

    // #4 Identify top-10 nodes
    let top_nodes = graph.top_k_nodes(10);
    println!("Top 10 most connected nodes:");
    for (node, degree) in top_nodes {
        println!("Node: {}, Degree: {}", node, degree);
        }

    // #5 Estimates average shortest length (Six Degrees of Seperation)
    if let Some(avg_length) = calculate_average_shortest_path(&graph, 1000) {
        println!("Average shortest path length (Six Degrees of Seperation): {:.2}", avg_length);
        } else {
            println!("No Valid Pairs found ");
                }
    
    // #6 Compute specific shortest path 
    let shortest_path = bfs::shortest_path(&graph, &transactions[0].tx_id1, &transactions[0].tx_id2);
    println!("The Shortest Path in between Nodes: {:?}", shortest_path);

    // #7 Plot histogram 
    plot_degree_distribution(&degree_distribution)?;
    println!("Degree distribution plot");
    Ok(())
}