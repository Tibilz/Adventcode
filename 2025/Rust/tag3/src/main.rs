use std::fs;

struct Bank {
    bank: String,
}

impl Bank {
    fn new(bank: String) -> Bank {
        Self { bank }
    }

    fn find_highest_number(self) -> u64 {

        let bank: Vec<u32> = self.bank.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        //println!("{:?}", bank);

        let max_1 = bank.iter().max().unwrap().clone();
        let highest_1 = bank.iter().position(|x| x == &max_1).unwrap();

        let mut split_bank = bank.clone();
        let mut splitted_len = 0;
        //println!("{:?}", bank);
        if highest_1 < bank.len() - 1 {
            let (left, right) = split_bank.split_at(highest_1 + 1);

            splitted_len = left.len();
            split_bank = right.to_vec();
        } else {
            split_bank.remove(highest_1);
        }
        //println!("{:?}", split_bank);

        let max_2 = split_bank.iter().max().unwrap().clone();
        let highest_2 = split_bank.iter().position(|x| x == &max_2).unwrap();

        let string_result;
        if highest_1 <= (highest_2 + splitted_len) {
            string_result = max_1.to_string() + &*max_2.to_string();
        } else {
            string_result = max_2.to_string() + &*max_1.to_string();
        }
        string_result.parse::<u64>().unwrap()
    }

    fn find_highest_number2(self) -> u64 {
        let amount_of_batteries: u8 = 12;
        let mut max_pairs = Vec::new();

        let bank: Vec<u32> = self.bank.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut split_bank = bank.clone();

        let mut skip_pos = 0;
        let mut count_zeros = 0;

        for _i in 0..amount_of_batteries {


            if split_bank.contains(&0) {



                let mut index = 0;
                for i in split_bank.clone().iter() {
                    if count_zeros <= 0 { break; }
                    let left_places = split_bank.len() - (index+1) - (count_zeros-1);
                    let missing_batteries = amount_of_batteries as usize - max_pairs.len();

                    if index >= missing_batteries { break; }

                    if *i == 0 && left_places >= missing_batteries {

                        let (left, right) = split_bank.split_at(index + 1);
                        skip_pos += left.len() as u64;

                        split_bank = right.to_vec();
                        count_zeros -= 1;

                        index = 0;
                    } else {
                        index += 1;
                    }
                }
            }

            let max = split_bank.iter().max().unwrap().clone();
            let pos = split_bank.iter().position(|x| x == &max).unwrap() as u64;

            max_pairs.push((max, pos + skip_pos));

            let batteries_left: u8 = amount_of_batteries - max_pairs.len() as u8;
            if (pos + batteries_left as u64) <= (&split_bank.len() - 1 - count_zeros) as u64 {
                let (left, right) = split_bank.split_at((pos + 1) as usize);

                skip_pos += left.len() as u64;
                split_bank = right.to_vec();

                count_zeros = 0;
                split_bank.iter().for_each(|x| if *x == 0 { count_zeros+=1 } );
            } else {
                split_bank.remove(pos as usize);
                split_bank.insert(pos as usize, 0);
                count_zeros += 1;
            }
        }

        let mut string_result: String = String::new();

        max_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        max_pairs.iter().for_each(|x| string_result.push_str(x.0.to_string().as_str()));
        //println!("{:?}", max_pairs);

        //println!("{}", string_result);
        string_result.parse::<u64>().unwrap()
    }

    fn find_highest_number3(self) -> u64 {
        let amount_of_batteries: u8 = 12;
        let mut max_pairs:Vec<(u32, u64)> = Vec::new();

        let bank: Vec<u32> = self.bank.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut split_bank = bank.clone();

        max_pairs = recursive_handler(amount_of_batteries, max_pairs, split_bank, 0);

        let mut string_result: String = String::new();

        max_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        max_pairs.iter().for_each(|x| string_result.push_str(x.0.to_string().as_str()));
        println!("{}", string_result);
        string_result.parse::<u64>().unwrap()
    }

    fn find_highest_number4(self) -> u64 {
        let mut amount_of_batteries: u8 = 12;
        let mut missing_batteries = amount_of_batteries.clone();
        let mut skip_pos= 0;
        let mut max_pairs:Vec<(u32, u64)> = Vec::new();

        let bank: Vec<u32> = self.bank.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut split_bank = bank.clone();

        for _i in 0..amount_of_batteries {

            let max = split_bank.iter().max().unwrap().clone();
            let pos = split_bank.iter().position(|x| x == &max).unwrap() as u64;

            max_pairs.push((max, pos + skip_pos));
            missing_batteries -= 1;

            let left_places = split_bank.len() - (pos+1) as usize;

            split_bank.remove(pos as usize);
            let (left, right) = split_bank.split_at(pos as usize);

            if left_places < missing_batteries as usize {
                split_bank = left.to_vec();
                missing_batteries = missing_batteries - left_places as u8;

                if left_places > 0 {

                }

            } else {
                skip_pos += left.len() as u64 + 1;
                split_bank = right.to_vec();

            }
        }


        let mut string_result: String = String::new();

        max_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        max_pairs.iter().for_each(|x| string_result.push_str(x.0.to_string().as_str()));
        println!("{}", string_result);
        string_result.parse::<u64>().unwrap()
    }

}


pub fn recursive_handler(mut missing_batteries: u8, mut max_pairs: Vec<(u32, u64)>, mut split_bank: Vec<u32>, mut skip_pos: u64) -> Vec<(u32, u64)> {
    if missing_batteries == 0 {
        return max_pairs;
    }

    let max = split_bank.iter().max().unwrap().clone();
    let pos = split_bank.iter().position(|x| x == &max).unwrap() as u64;

    max_pairs.push((max, pos + skip_pos));
    missing_batteries -= 1;

    let left_places = split_bank.len() - (pos+1) as usize;

    split_bank.remove(pos as usize);
    let (left, right) = split_bank.split_at(pos as usize);

    if left_places < missing_batteries as usize {
        split_bank = left.to_vec();
        missing_batteries = missing_batteries - left_places as u8;

    } else {
        skip_pos += left.len() as u64 + 1;
        split_bank = right.to_vec();

    }
    recursive_handler(missing_batteries, max_pairs, split_bank, skip_pos)
}

fn main() {
    let data = fs::read_to_string("test_input.txt").unwrap();
    //println!("{}", data);
    let lines: Vec<&str> = data.lines().collect();

    let mut result: u64 = 0;

    for line in lines {
        let bank = Bank::new(line.to_string());
        //println!("{}",bank.find_highest_number2());
        result += bank.find_highest_number4();
    }

    println!("{}", result);
}
// 234234234234278
