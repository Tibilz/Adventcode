use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut res= 0;
    if let Ok(lines) = read_lines("../puzzle_inputs/1.txt"){
        for line in lines.flatten() {

            let firstDigit = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let lastDigit = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            let lineres = line.chars().nth(firstDigit).unwrap().to_string() + &line.chars().nth(lastDigit).unwrap().to_string();
            res += lineres.parse::<i32>().unwrap();
        }
    }

    println!("{}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}