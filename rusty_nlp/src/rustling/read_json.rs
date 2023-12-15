use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)] 

// derive is a macro that implements the trait for the struct, 
// traits are like interfaces in other languages, 
// interfaces are a way to define behavior,
// behavior is what a type can do.

pub struct Customer {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub address: String,
}

pub fn read_json() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "src/rustling/data/"; // path to the data folder
    let file = std::fs::File::open(&(path.to_owned() + "customers.json"))?; // open the file
    let customers: Vec<Customer> = serde_json::from_reader(file)?; // deserialize the file
    println!("{:?}", customers); // print the customers
    Ok(())
}
