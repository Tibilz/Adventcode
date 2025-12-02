use std::fs;

const DIAL_MAX: i32 = 99 + 1;

struct Dial {
    number: i32
}

impl Dial {
    fn new(number: i32) -> Dial {
        Dial { number }
    }

    fn wrap_sub(&mut self, rotation: i32) {
        if (self.number - rotation) < 0 {
            self.number = ((self.number - rotation) % DIAL_MAX) + DIAL_MAX;
            return;
        }
        self.number -= rotation;
    }

    fn wrap_add(&mut self, rotation: i32) {
        if self.number + rotation >= DIAL_MAX {
            self.number = (self.number + rotation) % DIAL_MAX;
            return;
        }
        self.number += rotation;
    }
}

fn main() {

    // READ INPUT
    let data = fs::read_to_string("src/input.txt").unwrap();
    //println!("{}", data);
    let lines: Vec<&str> = data.lines().collect();


    let mut dial = Dial::new(50);
    let mut result: u32 = 0;

    let mut dial2 = Dial::new(50);
    let mut result2: u32 = 0;

    for line in lines.clone() {
        //print!("{}", line);
        result += task1(&mut dial, line);
        result2 += task2(&mut dial2, line);

    }

    println!("Task 1 : {}", result);
    println!("Task 2 : {}", result2);
}

fn task1(dial: &mut Dial, line: &str) -> u32 {
    if line.starts_with("R") {
        dial.wrap_add(line[1..].parse::<i32>().unwrap() % DIAL_MAX);
    } else if line.starts_with("L") {
        dial.wrap_sub(line[1..].parse::<i32>().unwrap() % DIAL_MAX);
    }

    if dial.number == 0 {
        return 1
    }
    0
}

fn task2(dial: &mut Dial, line: &str) -> u32 {
    let mut result: u32 = 0;
    let line_number: i32 = line[1..].parse::<i32>().unwrap();
    let passed_zeros = (&line_number / DIAL_MAX) as u32;
    result += passed_zeros;

    let mod_line_number = line_number % DIAL_MAX;
    if line.starts_with("L") {
        if dial.number != 0 && dial.number - mod_line_number < 0 {
            result += 1;
        }

        dial.wrap_sub(mod_line_number);
    } else if line.starts_with("R") {
        if (dial.number + mod_line_number) > DIAL_MAX {
            result += 1;
        }

        dial.wrap_add(mod_line_number);
    }

    if dial.number == 0 {
        result += 1;
    }

    result
}

