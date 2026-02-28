use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Customer {
    pub name: String,
    pub request: Vec<String>,
    pub keywords: Vec<String>,
    pub thankyou: Vec<String>,
    pub bonus_thankyou: Vec<String>,
    pub hints: Vec<String>,
}

pub(crate) fn read_customer_from_json(filepath: PathBuf) -> Result<Customer, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let customer: Customer = serde_json::from_reader(reader)?;
    Ok(customer)
}
