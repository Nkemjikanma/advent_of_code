mod day_1_secret_entrance;
mod day_2_gift_shop;
mod day_3_lobby;
mod day_4_printing_department;
mod day_5_cafeteria;
mod day_6_trash;
fn main() {
    println!("Hello, welcome to Advent of Rust!");

    // day_1_secret_entrance::get_password();
    // day_2_gift_shop::get_valid_ids();
    // day_3_lobby::get_joltage_part_1();
    // day_3_lobby::get_joltage_part_2();
    // day_4_printing_department::get_accessible_rolls();
    day_5_cafeteria::solve_day_5();
    day_6_trash::get_trash_compactor();
}
