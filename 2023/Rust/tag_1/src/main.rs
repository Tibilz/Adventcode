use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // Lese Zeilen aus der Datei
    if let Ok(lines) = read_lines("../puzzle_inputs/1.txt") {
        // Mapping der Zahlwörter
        let words_to_numbers: HashMap<&str, u8> = [
            ("one", 1), ("two", 2), ("three", 3), ("four", 4),
            ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ]
        .iter()
        .cloned()
        .collect();

        let mut res1: u32 = 0;
        let mut res2: u32 = 0;

        for line in lines.flatten() {
            // Part 1: Berechne `res1`
            let v1: Vec<&str> = line.matches(char::is_numeric).collect();
            if let (Some(first), Some(last)) = (v1.first(), v1.last()) {
                res1 += format!("{}{}", first, last)
                    .parse::<u32>()
                    .expect("String konnte nicht in Zahl umgewandelt werden");
            }

            // Part 2: Berechne `res2`
            let mut occurence: Vec<(i32, u8)> = Vec::new();

            // Wörter zu Zahlen hinzufügen
            for (&key, &value) in words_to_numbers.iter() {
                if let Some(pos) = line.find(key) {
                    occurence.push((pos as i32, value));
                }
            }

            // Numerische Zeichen hinzufügen
            line.char_indices()
                .filter_map(|(index, c)| c.to_digit(10).map(|digit| (index as i32, digit as u8)))
                .for_each(|entry| occurence.push(entry));

            // Minimum und Maximum berechnen
            let min = occurence.iter().min_by_key(|&(first, _)| first);
            let max = occurence.iter().max_by_key(|&(first, _)| first);

            if let (Some(&(min_first, min_value)), Some(&(max_first, max_value))) = (min, max) {
                if let Ok(value) = format!("{}{}", min_value, max_value).parse::<u32>() {
                    res2 += value;
                } else {
                    eprintln!("Fehler: Konkatenierter String konnte nicht in u32 umgewandelt werden.");
                }
            }

            // `occurence` für die nächste Zeile leeren
            occurence.clear();
        }

        // Ergebnisse ausgeben
        println!("res1: {}", res1);
        println!("res2: {}", res2);
    }
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}