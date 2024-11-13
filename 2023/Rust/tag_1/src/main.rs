use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("../puzzle_inputs/1.txt") {
        // Consumes the iterator, returns an (Optional) String

        let words_to_numbers: HashMap<&str, u32> = [
            ("one", 1), ("two", 2), ("three", 3), ("four", 4),
            ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
        ].iter().cloned().collect();
        
        let mut res1: u32 = 0;
        let mut res2: u32 = 0;

        for line in lines.flatten() {
            // Part1
            let v1: Vec<&str> = line.matches(char::is_numeric).collect();
            res1 += format!("{}{}",v1.first().unwrap(), v1.last().unwrap()).parse::<u32>().expect("String konnte nicht in Zahl umgewandelt werden");
            
            // Part2
        }


        println!("{}", res1);
        println!("{}", res2);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}