use crate::resource_loader;

/**
 * In PartOne we loop thought the data
 * For each line we check if it's a "noop" or "addx" instruction.
 * If it's a "noop" we call the process_cycle_and_get_signal_strength one time
 * If it's a "addx" we call the process_cycle_and_get_signal_strength two time and then change x value
 * Each time process_cycle_and_get_signal_strength is called we sum it's return with the previous value of sum_of_signal_strengths
 * sum_of_signal_strengths represent the sum of all signal at 40 cycle interval starting at 20
 */
pub fn part_one() {
    let data = resource_loader::load_resource("day10-data.txt");
    let mut cycle = 0;
    let mut x = 1;
    let mut sum_of_signal_strengths = 0;

    for line in data {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["addx", value] => {
                sum_of_signal_strengths += process_cycle_and_get_signal_strength(&mut cycle, &x);
                sum_of_signal_strengths += process_cycle_and_get_signal_strength(&mut cycle, &x);
                x += value.parse::<i32>().unwrap();
            },
            ["noop"] => {
                sum_of_signal_strengths += process_cycle_and_get_signal_strength(&mut cycle, &x);
            },
            _ => panic!("Unknown CPU instruction"),
        }
    }

    dbg!(sum_of_signal_strengths);
}

/**
 * Helper function that add one cycle and check if the value of x need to be kept
 */
fn process_cycle_and_get_signal_strength(cycle: &mut i32, x: &i32) -> i32 {
    *cycle += 1;

    if *cycle % 40 == 20 {
        return (*cycle) * (*x);
    }

    return 0;
}

pub fn part_two() {
    let data = resource_loader::load_resource("day10-data.txt");

    for line in data {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[..] {
            ["addx", _] => {

            }
            ["noop"] => {},
            _ => panic!("Unknown CPU instruction"),
        }
    }
}
