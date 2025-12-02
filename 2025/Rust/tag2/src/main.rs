use std::fs;

#[derive(Debug)]
struct Range {
    start: String,
    end: String,
}

impl Range {
    fn new(start: String, end: String) -> Self {
        Range { start, end }
    }

    fn check_repeated_seq(&self, task1: bool) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let start_range = self.start.parse::<u64>().unwrap();
        let end_range = self.end.parse::<u64>().unwrap();

        // Numbers like 555555, 565656, 567567, ...

        for i in start_range..=end_range {
            let number = &i.to_string();
            for j in 0..number.len()/2 {
                let slice = &number[..=j];
                let mut our_pattern = slice.repeat(number.len()/(j+1));

                // Task 1 : only twice
                // (if-case is obviously very inperformant, especially in 3 for loops - but looks much nicer than two copies of the same method)
                if (task1 == true) {
                    our_pattern = slice.repeat(2);
                }

                if our_pattern.eq(number){
                    result.push(our_pattern);
                    break;
                }
            }
        }
        //println!("{:?}", result);
        //result.dedup();
        result
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("{}", data);
    let lines: Vec<&str> = data.split(",").collect();

    let mut results: Vec<String> = Vec::new();
    let mut results2: Vec<String> = Vec::new();

    for line in lines {
        //println!("{}", line);
        let start = line.split('-').collect::<Vec<&str>>()[0].to_string();
        let end = line.split('-').collect::<Vec<&str>>()[1].to_string();
        let range: Range = Range::new(start, end);
        //print!("{:?}", range);
        results.append(&mut range.check_repeated_seq(true));
        results2.append(&mut range.check_repeated_seq(false));

    }

    let result = results.iter().fold(0, |acc, x| acc + x.parse::<u64>().unwrap());
    let result2 = results2.iter().fold(0, |acc, x| acc + x.parse::<u64>().unwrap());
    println!("Task 1 : {}", result);
    println!("Task 2 : {}", result2);
    // ! 37031985703
    // ? 737373, 808080 - wanted for task 2 ;)
}
