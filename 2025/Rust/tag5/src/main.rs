use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("{}", data);
    //let lines: Vec<String> = data.lines().map(|x| x.to_string()).collect();
    let lines: Vec<&str> = data.lines().collect();

    let res1 = task1(lines.clone());
    let res2 = task2(lines.clone());

    println!("task1 : {}", res1);
    println!("task2 : {}", res2);

}

fn task2 (lines: Vec<&str>) -> u64 {
    let mut lines_split = lines.split(|x| x.eq(&""));

    // Convert String_Range into Vector_Range
    let mut valid_id_ranges: Vec<Vec<u64>> = Vec::new();
    for element in lines_split.next().unwrap().iter() {
        let ranges = element.split('-').collect::<Vec<&str>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        valid_id_ranges.push(ranges);
    }

    // Merge overlapping IDs
    valid_id_ranges.sort_by(|a, b| a.first().unwrap().cmp(&b.first().unwrap()));

    let mut merged_valid_id_ranges: Vec<Vec<u64>> = Vec::new();
    merged_valid_id_ranges.push(valid_id_ranges.first().unwrap().to_vec());

    for i in 1..valid_id_ranges.len() {
        if merged_valid_id_ranges.last().unwrap()[1] >= valid_id_ranges[i][0] {
            if valid_id_ranges[i][1] > merged_valid_id_ranges.last().unwrap()[1] {
            merged_valid_id_ranges.last_mut().unwrap()[1] = valid_id_ranges[i][1];
            }
        } else {
            merged_valid_id_ranges.push(valid_id_ranges[i].clone());
        }
    }

    let mut fresh_ingredients_amount: u64 = 0;
    for ranges in merged_valid_id_ranges.iter() {
        fresh_ingredients_amount += ranges[1] - ranges[0] + 1;
    }

    fresh_ingredients_amount
}

fn task1 (lines: Vec<&str>) -> u64{
    let mut lines_split = lines.split(|x| x.eq(&""));

    // Convert String_Range into Vector_Range
    let mut valid_id_ranges: Vec<Vec<u64>> = Vec::new();
    for element in lines_split.next().unwrap().iter() {
        let ranges = element.split('-').collect::<Vec<&str>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        valid_id_ranges.push(ranges);
    }

    // Merge overlapping IDs
    // Theoretically we don't need this step, we could just check for every ingredient if its fresh -> check if it is in any fresh_range
    // NOTE: Ok, we need it for task2
    /*
    valid_id_ranges.sort_by(|a, b| a.first().unwrap().cmp(&b.first().unwrap()));

    let mut merged_valid_id_ranges: Vec<Vec<u64>> = Vec::new();
    merged_valid_id_ranges.push(valid_id_ranges.first().unwrap().to_vec());

    for i in 1..valid_id_ranges.len() {
        if merged_valid_id_ranges.last().unwrap()[1] >= valid_id_ranges[i][0] {
            if valid_id_ranges[i][1] > merged_valid_id_ranges.last().unwrap()[1] {
                merged_valid_id_ranges.last_mut().unwrap()[1] = valid_id_ranges[i][1];
            }
        } else {
            merged_valid_id_ranges.push(valid_id_ranges[i].clone());
        }
    }
    */

    // Find fresh IDs
    let mut fresh_ingredients: u64 = 0;
    let ingredient_list = lines_split.next().unwrap().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for element in ingredient_list {
        //println!("{:?}", element);
        for valid_ingredient in valid_id_ranges.iter() {
            if element >= valid_ingredient[0] {
                if element <= valid_ingredient[1] {
                    fresh_ingredients += 1;
                    break;
                }
            }
        }

        //actual_range = merged_valid_id_ranges.next().unwrap();
    }

    fresh_ingredients
}

