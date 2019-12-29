//Notes:
//Create a struct to represent a row
//row to vector of values? vector of enum type for a row, transaction?

use std::path::Path;
use std::error::Error;
use serde::Deserialize;
use std::process;

#[derive(Debug,Deserialize)]
struct Transaction {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = " Transaction Details")]
    transaction_details: String,
    #[serde(rename = " Funds Out")]
    funds_out:Option<f64>,
    #[serde(rename = " Funds In")]
    funds_in: Option<f64>,
}

fn print_me(path: &Path) -> Result<(),Box< Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}",record)
    }
    Ok(())
}

fn main() {
    let path = Path::new("/home/dan/financial/simplii/SIMPLII.csv");

        if let Err(err) = print_me(path) {
            println!("error running: {}", err);
            process::exit(1);
        }

}
