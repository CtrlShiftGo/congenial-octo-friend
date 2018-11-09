extern crate chrono;

use std::env;
use std::vec;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use chrono::prelude::*;

//
fn main() {
    let args: Vec<String> = env::args().collect();
    // Parse arguments
    // Open CSV
    Csv::read("friends.csv".to_string());
    // Perform operation
    // Close CSV
}

struct Row {
    name: String,
    date: chrono::DateTime<chrono::Local>,
    description: String
}
struct Csv {
    rows: Vec<Row>
}
// Initialize CSV
impl Csv {
    fn read(filename: String) {
        let mut file = File::open(filename).expect("File not found.\n");
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let mut split_contents = contents.split("\n");
        let rows : Vec<&str> = split_contents.collect();
        let rows = rows.clone();
    }
}