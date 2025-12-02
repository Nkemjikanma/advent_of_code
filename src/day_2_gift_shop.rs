use std::fs;
pub fn get_valid_ids() {
    println!("-------- Day 2 --------");

    let mut part_a_invalid_ids_sum = 0;
    let mut part_b_invalid_ids_sum = 0;
    let content = fs::read_to_string("day_2_ids.txt").expect("Failed to read file");

    let id_ranges: Vec<(u64, u64)> = content
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|part| {
            let (start, end) = part.split_once('-').unwrap();
            println!("{}-{}", start, end);
            (start.trim().parse().unwrap(), end.trim().parse().unwrap())
        })
        .collect::<Vec<_>>();

    for (start, end) in &id_ranges {
        'next_id: for id in *start..=*end {
            let id_as_string = id.to_string();
            let length = id_as_string.len();

            if id_as_string.starts_with('0') || length == 1 {
                continue;
            }

            // println!("******** A ********");
            // if id_as_string.len() % 2 == 0 {
            //     let mid = id_as_string.len() / 2;
            //
            //     let (a, b) = id_as_string.split_at(mid);
            //
            //     if let (Ok(a), Ok(b)) = (a.trim().parse::<u64>(), b.trim().parse::<u64>()) {
            //         if a == b {
            //             part_a_invalid_ids_sum += id;
            //         }
            //     }
            // }

            // println!("******** B ********");
            for block_len in 1..=(length / 2) {
                if length % block_len != 0 {
                    continue;
                }

                let id_substring = &id_as_string[0..block_len];

                let mut valid_pattern = true;

                for pos in (block_len..length).step_by(block_len) {
                    let chunk = &id_as_string[pos..pos + block_len];

                    if chunk != id_substring {
                        valid_pattern = false;
                        break;
                    }
                }

                if valid_pattern {
                    part_b_invalid_ids_sum += id;
                    continue 'next_id;
                }
            }
        }
    }

    // println!("{}", part_a_invalid_ids_sum);
    println!("{}", part_b_invalid_ids_sum);
}
