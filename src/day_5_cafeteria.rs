use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
pub fn solve_day_5() {
    println!("-------- Day 5 --------");

    get_fresh_ingredients();
    get_range_count()
}

pub fn get_fresh_ingredients() {
    println!("-------- Part A --------");
    let content = File::open("day_5_cafeteria.txt").unwrap();

    let lines = BufReader::new(content).lines();

    let mut empty_line = 0;
    let mut has_crossed_empty_line = false;

    let mut id_ranges: Vec<String> = Vec::new();
    let mut ingridient_ids: Vec<u64> = Vec::new();

    let mut valid_ids = 0;

    for line in lines.map_while(Result::ok) {
        // println!("{}", line);

        if line.is_empty() {
            has_crossed_empty_line = true;
            continue;
        }

        if !has_crossed_empty_line {
            id_ranges.push(line);
        } else {
            ingridient_ids.push(line.trim().parse().unwrap());
        }
    }

    // println!("ID ranges: {:?}", id_ranges);
    // println!("Ingredient: {:?}", ingridient_id);

    for ingredient_idx in 0..ingridient_ids.len() {
        let mut is_valid_id = false;
        for range_idx in 0..id_ranges.len() {
            let (start, end) = id_ranges[range_idx].split_once('-').unwrap();

            if is_valid_id {
                continue;
            }

            if ingridient_ids[ingredient_idx] >= start.trim().parse().unwrap()
                && ingridient_ids[ingredient_idx] <= end.trim().parse().unwrap()
            {
                is_valid_id = true;
            }
        }

        if is_valid_id {
            valid_ids += 1;
        }
    }

    println!("{}", valid_ids);
}

pub fn get_range_count() {
    println!("-------- Part B --------");

    let content = File::open("day_5_cafeteria.txt").unwrap();

    let lines = BufReader::new(content).lines();

    let mut id_ranges: Vec<(u64, u64)> = Vec::new();

    for line in lines.map_while(Result::ok) {
        // println!("{}", line);

        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        id_ranges.push((start.trim().parse().unwrap(), end.trim().parse().unwrap()));
    }

    // let mut id_ranges = [(3, 5), (10, 14), (16, 20), (12, 18)];

    id_ranges.sort_by_key(|&(start, _)| start);

    let mut range_spread: Vec<(u64, u64)> = Vec::new();

    for (mut start, mut end) in id_ranges {
        if let Some((last_start, last_end)) = range_spread.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                range_spread.push((start, end));
            }
        } else {
            range_spread.push((start, end));
        }
    }

    let mut total: u64 = 0;

    for (start, end) in range_spread {
        total += end - start + 1;
    }

    println!("{}", total);
}
