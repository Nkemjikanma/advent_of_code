use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn get_joltage_part_1() {
    println!("-------- Day 3 --------");
    let mut total_joltage_voltage = 0;

    if let Ok(lines) = read_file("day_3_lobby.txt") {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            let mut joltage_value = 0;

            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                continue;
            }

            let digits: Vec<u32> = trimmed_line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            if digits.len() < 2 {
                continue;
            }

            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let value = 10 * digits[i] as u64 + digits[j] as u64;

                    if value > joltage_value {
                        joltage_value = value
                    }
                }
            }

            // for (index, item) in trimmed_line.chars().enumerate() {
            //     if item.to_string().trim().parse::<u64>().unwrap()
            //         >= joltage_value_1.trim().parse::<u64>().unwrap()
            //     {
            //         joltage_value_1 = item.to_string();
            //         max_index = index;
            //     } else {
            //         continue;
            //     }
            //
            //     // let (_, rest_of_trimmed_line) = trimmed_line.split_at(index);
            //
            //     // for idx in index..trimmed_line.len() {
            //     //     let character = trimmed_line.chars().nth(idx).unwrap();
            //     //
            //     //     let candidate = format!("{}{}", joltage_value, character);
            //     //     println!("{}", candidate);
            //     //     if candidate.parse::<u64>().unwrap()
            //     //         > joltage_value.trim().parse::<u64>().unwrap()
            //     //     {
            //     //         joltage_value = candidate;
            //     //     }
            //     // }
            // }

            // for (index, item) in trimmed_line.chars().enumerate().skip(max_index + 1) {
            //     if item.to_string().trim().parse::<u64>().unwrap()
            //         > joltage_value_2.trim().parse::<u64>().unwrap()
            //     {
            //         joltage_value_2 = item.to_string();
            //         // println!("{joltage_value_2}");
            //     } else {
            //         continue;
            //     }
            // }

            // println!("values per line: {}", joltage_value);

            total_joltage_voltage += joltage_value;
        }
    }
    println!("{}", total_joltage_voltage);
}

pub fn get_joltage_part_2() {
    let mut total_joltage_voltage = 0;

    if let Ok(lines) = read_file("day_3_lobby.txt") {
        for line in lines.map_while(Result::ok) {
            let mut joltage_value_2: String = String::from("");

            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                continue;
            }

            let mut digits: Vec<char> = trimmed_line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();

            if digits.len() < 2 {
                continue;
            }

            // part 2

            let mut picks_remaining = 12;
            let mut start = 0;

            while picks_remaining > 0 {
                // let mut max_index = 0;
                let end = digits.len() - picks_remaining;
                println!("digits {} - {}", digits.len(), picks_remaining);
                println!("end {}", end);

                let mut best = '0';
                let mut best_index = start;

                for i in start..=end {
                    println!("start {}", start);
                    if digits[i] > best {
                        best = digits[i];
                        best_index = i;
                    }
                }

                joltage_value_2.push(best);
                start = best_index + 1;
                picks_remaining -= 1;
                // let start_length = digits.len() - 12;

                // for i in start..=start_length {
                //     for i in joltage_value_2.chars() {}
                //     if digits[i] >= best {
                //         best = digits[i];
                //     }
                // }
                // joltage_value_2.push(best);
                // start = best_index + 1;
                //
                // for i in digits.iter().skip(start_length) {
                //     if digits[*i] >= best {
                //         best = digits[*i];
                //     }
                // }

                // joltage_value_2.push(best);
                println!("{}", joltage_value_2);
                // remaining_picks -= 1;
            }

            total_joltage_voltage += joltage_value_2.trim().parse::<u64>().unwrap();
        }
    }
    println!("{}", total_joltage_voltage);
}
pub fn read_file<T>(filename: T) -> Result<io::Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}
