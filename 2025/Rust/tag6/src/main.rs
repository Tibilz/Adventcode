use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("{}", data);
    //let lines: Vec<String> = data.lines().map(|x| x.to_string()).collect();
    let lines: Vec<&str> = data.lines().collect();

    let res = task1(lines.clone());
    println!("Task1 : {:?}", res);

    let lines: Vec<String> = data.lines().map(|x| x.to_string()).collect();
    let res = task2(lines);
    println!("Task2 : {:?}", res);
}

fn task2(lines: Vec<String>) -> u64 {
    //println!("lines: : {:?}", lines);
    let operators = lines.last().unwrap().split_whitespace().collect::<Vec<&str>>();
    let mut operands: Vec<u64> = Vec::new();
    for operator in operators.clone() {
        if operator == "*" {
            operands.push(1);
        } else {
            operands.push(0);
        }
    }

    // Find out, how many digits each number has
    let space_between = lines.last().unwrap().split("").collect::<Vec<&str>>();
    let mut number_length: Vec<u64> = vec![0; operators.len()];

    let mut j = 0;
    for i in 2..space_between.len() {
        if space_between[i] == " " {
            number_length[j] += 1;
        } else { j += 1; }
    }

    let temp = number_length.len();
    number_length[temp-1] += 1;

    // Get every number with whitespaces
    let mut better_numbers: Vec<Vec<String>> = Vec::new();

    for i in 0..lines.len()-1 {
        let mut temp = lines[i].clone();
        let mut temp2 = temp.split_off(*number_length.first().unwrap() as usize);
        let mut better_numbers_temp = Vec::new();
        for j in 1..number_length.len() {
            better_numbers_temp.push(temp.clone());
            temp2.remove(0);
            temp = temp2;
            temp2 = temp.split_off(number_length[j] as usize);
        }
        better_numbers_temp.push(temp);
        better_numbers.push(better_numbers_temp.clone());

    }

    // Create "right" numbers
    // 1 0 0
    // 0 2 4
    // 3 5 6
    let mut even_better_numbers: Vec<Vec<String>> = vec![vec![String::from(""); better_numbers.len()]; better_numbers[0].len()];
    for i in 0..better_numbers.len() {
        for j in 0..better_numbers[i].len() {
            for k in 0..better_numbers[i][j].len() {
                even_better_numbers[j][k].push_str(&*better_numbers[i][j].get(k..=k).unwrap().to_string());
            }
        }
    }

    for i in 0..even_better_numbers.len() {
        let temp = &mut even_better_numbers[i];
        for j in 0..temp.len() {
            temp[j] = temp[j].replace(' ', "");
            //println!("{:?}", temp[j]);
            if operators[i] == "*" {
                match temp[j].parse::<u64>() {
                    Ok(num) => operands[i] *= num,
                    Err(_) => continue,
                }
            } else {
                match temp[j].parse::<u64>() {
                    Ok(num) => operands[i] += num,
                    Err(_) => continue,
                }
            }
        }
    }

    operands.iter().sum()
}

fn task1(lines: Vec<&str>) -> u64 {
    let operators = lines.last().unwrap().split_whitespace().collect::<Vec<&str>>();
    let mut operands: Vec<u64> = Vec::new();
    for operator in operators.clone() {
        if operator == "*" {
            operands.push(1);
        } else {
            operands.push(0);
        }
    }

    for i in 0..lines.len()-1 {
        let temp = lines[i].split_whitespace().collect::<Vec<&str>>();
        for j in 0..temp.len() {
            if operators[j] == "*" {
                operands[j] *= temp[j].parse::<u64>().unwrap();
            } else {
                operands[j] += temp[j].parse::<u64>().unwrap();
            }
        }
    }

    operands.iter().sum()
}

