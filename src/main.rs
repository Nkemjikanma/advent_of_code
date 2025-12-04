mod day_1_secret_entrance;
mod day_2_gift_shop;
mod day_3_lobby;

fn main() {
    println!("Hello, welcome to Advent of Rust!");

    day_1_secret_entrance::get_password();
    day_2_gift_shop::get_valid_ids();
    // day_3_lobby::get_joltage_part_1();
    day_3_lobby::get_joltage_part_2();
}
