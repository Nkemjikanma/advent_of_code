use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX: u8 = 99;
const SIZE: u32 = 100;

pub fn get_password() {
    let mut current_position: u32 = 50;
    let mut password: u32 = 0;

    if let Ok(lines) = read_lines("day_1_rotations.txt") {
        for line in lines.map_while(Result::ok) {
            // println!("{}", line);

            if line.is_empty() {
                continue;
            }

            let (direction, number) = line.split_at(1);
            let raw_steps: u32 = number.trim().parse().expect("Invalid number found");
            let steps: u32 = raw_steps;

            match direction {
                "R" => {
                    for _ in 0..steps {
                        current_position = (current_position + 1) % SIZE;

                        if current_position == 0 {
                            // println!("The dail rotated {} to point at {}", line, current_position);
                            password += 1;
                        }
                    }
                }
                "L" => {
                    for _ in 0..steps {
                        current_position = (current_position + (SIZE - 1)) % SIZE;

                        if current_position == 0 {
                            // println!("The dail rotated {} to point at {}", line, current_position);
                            password += 1;
                        }
                    }
                }
                _ => {
                    println!("SOMETHING HAPPENED");
                }
            }
        }
    }

    println!("{}", password);
}

fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}
