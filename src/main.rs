extern crate chrono;

use std::env;
use std::vec;
use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;

//
fn main() {
    let args: Vec<String> = env::args().collect();
    // Parse arguments
    // Open CSV
    LCsv::read("friends.csv".to_string());
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
    // Read a CSV from disk
    fn read(filename: String) -> Csv {
        let mut file = File::open(filename).expect("File not found.\n");
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        // Split rows by newline
        let mut split_contents = contents.split("\n");

        // Collect the newly split rows into a vector
        let rows : Vec<&str> = split_contents.collect();
        let mut csv : Csv = Csv {rows: Vec::new()};
        // Split rows and add to struct.
        for row in rows {
            csv.rows.push(Row {name: "name".to_string(),
                date: Local::now(),
                description: "description".to_string()})
        }
        return csv
    }
}