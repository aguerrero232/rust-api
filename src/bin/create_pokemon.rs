// use rust_api::models::*;
// use serde_json;

// use rust_api::establish_connection;
use std::io::{Read}; // stdin, 

fn main() {
    // let connection = &mut establish_connection();
    // read in pokemon data from file
    let file_loc = "/data/pokemon_data.json";

    // open file at file_loc and read in contents

    let mut file = std::fs::File::open(file_loc).expect("File not found.");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Something went wrong reading the file.");

    print!("contents: {}", contents);

}

// #[cfg(not(windows))]
// const EOF: &str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";