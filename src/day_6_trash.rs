use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub fn get_trash_compactor() {
    part_1();
    part_2();
}

pub fn part_1() {
    let file = File::open("day_6_trash.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut input_list: Vec<Vec<String>> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let line_list: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        input_list.push(line_list);
    }

    let rows = input_list.len();
    let cols = input_list[0].len();

    let mut total = 0;
    for col in 0..cols {
        let operation = &input_list[rows - 1][col];

        /// get all numbers above the last row of this column
        let column_values =
            (0..rows - 1).map(|row| input_list[row][col].trim().parse::<u64>().unwrap());

        let result: u64 = if operation == "*" {
            column_values.product()
        } else if operation == "+" {
            column_values.sum()
        } else {
            panic!("Something fishy");
        };

        total += result;
    }

    println!("here o- {}", total);
}

pub fn part_2() {
    let file = File::open("day_6_trash.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut input_list: Vec<Vec<String>> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let line_list: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        input_list.push(line_list);
    }

    let rows = input_list.len();
    let cols = input_list[0].len();

    let mut total = 0;

    for col in (0..cols).rev() {
        let operator = &input_list[rows - 1][col];

        /// get all numbers above the last row of this column
        let column_values: Vec<u64> = (0..rows - 1)
            .map(|row| input_list[row][col].parse::<u64>().unwrap())
            .collect();

        let mut max_char_count = &column_values
            .iter()
            .max_by_key(|item| item.to_string().len())
            .unwrap();

        println!("{:?}", max_char_count);

        for (idx, mut col) in column_values.iter().enumerate() {}
    }
}
