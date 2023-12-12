use std::error::Error;

// extern crate csv;
use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> { // Box<dyn Error> is a trait object, dyn is short for dynamic, Box is a smart pointer, Result is an enum that will return either Ok or Err
    let mut reader = csv::Reader::from_path(path)?; // ? is a shortcut for try!
    for result in reader.records(){ // iterate over the records
        let record = result?; // ? is a shortcut for try!
        println!("{:?}", record); // print the record

    }
    Ok(()) // return Ok if everything went fine
}

pub fn csv_reader(){
    let path: &str = "src/rustling/data/"; // path to the data folder
    if let Err(err) = read_from_file(&(path.to_owned() + "customers.csv")){ // if read_from_file returns an error, print it, to_owned() is a method that return an owned version of the string
        // /Users/aashishtangnami/Documents/Projects/RUST/Rust/rusty_nlp/src/rustling/data/customers.csv
        println!("error running example: {}", err); // println! is a macro
        eprintln!("error: {}", err); // eprintln! is like println! but prints to stderr
        std::process::exit(1); 
    }

}