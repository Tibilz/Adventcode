use std::fs;
use crate::PosLine::{Same, UpDown};

pub enum PosLine {
    Same,
    UpDown
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("{}", data);
    let mut lines: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let result = task_2(&mut lines);

    println!("{}", result);

}

fn task_2(mut lines: &mut Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    let mut temp_result = 1;

    loop {
        if temp_result == 0 {
            break;
        }
        temp_result = task_1(&mut lines);

        for line in lines.iter_mut() {
            for i in 0..line.len() {
                if line[i] == 'x' {
                    line.remove(i);
                    line.insert(i, '.');
                }
            }
        }
        result += temp_result
    }

    result
}

fn task_1(lines: &mut Vec<Vec<char>>) -> u64 {
    let roll_limit: u8 = 4;
    let mut result: u64 = 0;

    for i in 1..=lines.len() {

        for j in 0..lines[i-1].len() {
            if lines[i-1][j] == '@' {
                let mut adjacent = 0;

                if i > 1 {
                    adjacent += find_adjacent(&lines[i-2], j, UpDown);
                }
                adjacent += find_adjacent(&lines[i-1], j, Same);
                if i < lines.len() {
                    adjacent += find_adjacent(&lines[i], j, UpDown);
                }

                if adjacent < roll_limit {
                    result += 1;
                    lines[i-1].remove(j);
                    lines[i-1].insert(j, 'x');
                }
            }
        }
    }

    result
}

fn find_adjacent (line: &Vec<char>, pos: usize, pos_line: PosLine) -> u8 {
    let mut adjacent = 0;
    if pos > 0 {
        match line.iter().nth(pos - 1) {
            Some('@') | Some('x') => { adjacent += 1; },
            _ => { }
        }
    }
    if let UpDown = pos_line {
        match line.iter().nth(pos) {
            Some('@') | Some('x') => { adjacent += 1;  }
            _ => { }
        }
    }
    match line.iter().nth(pos + 1) {
        Some('@') | Some('x') => { adjacent += 1; }
        _ => { }
    }

    adjacent
}
