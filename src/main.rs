
//Sets up CSV parsing 
//Loads txs)edgelist.csv file into data structure (vector and edge)
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
struct Transaction {
    #[serde(rename = "t xId1")]
    tx_id1: String,

    #[serde(rename = "txId2")]
    tx_id2: String,
}

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
    let transactions = read_csv()?;
    
    // For now, just print the first transaction to check it
    println!("{:?}", transactions[0]);
    
    Ok(())
}
