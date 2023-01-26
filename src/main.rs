mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod resource_loader;

fn main() {
    println!("Running Day01 Part 1 :");
    day01::part_one();

    println!("Running Day01 Part 2 :");
    day01::part_two();

    println!("Running Day02 Part 1 :");
    day02::part_one();

    println!("Running Day02 Part 2 :");
    day02::part_two();

    println!("Running Day03 Part 1 :");
    day03::part_one();

    println!("Running Day03 Part 2 :");
    day03::part_two();

    println!("Running Day04 Part 1 :");
    day04::part_one();

    println!("Running Day04 Part 2 :");
    day04::part_two();

    println!("Running Day05 Part 1 & 2 :");
    day05::part_one_and_two();
}
