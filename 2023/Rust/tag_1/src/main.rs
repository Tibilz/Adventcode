use std::fs::File;
use std::io::prelude::*;
// use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../puzzle_inputs/1.txt");
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    //io::BufReader::new(file).lines().;

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

}
