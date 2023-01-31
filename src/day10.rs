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
            }
            ["noop"] => {
                sum_of_signal_strengths += process_cycle_and_get_signal_strength(&mut cycle, &x);
            }
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

/**
 * In PartTwo we add a variable crt which contain a Vec of string.
 * For each line we check if it's a "noop" or "addx" instruction.
 * If it's a "noop" we call the process_cycle one time
 * If it's a "addx" we call the process_cycle two time and then change x value
 * Each time process_cycle is called the crt is changed
 * At the end of the function we print the crt on the console
 */
pub fn part_two() {
    let data = resource_loader::load_resource("day10-data.txt");
    let mut crt = Vec::<String>::new();
    let mut cycle = 0;
    let mut x = 1;

    crt.push(String::new());

    for line in data {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[..] {
            ["addx", value] => {
                process_cycle(&mut cycle, x, &mut crt);
                process_cycle(&mut cycle, x, &mut crt);
                x += value.parse::<i32>().unwrap();
            }
            ["noop"] => {
                process_cycle(&mut cycle, x, &mut crt);
            }
            _ => panic!("Unknown CPU instruction"),
        }
    }

    for crt_line in crt {
        if crt_line.is_empty() == false {
            println!("{}", crt_line);
        }
    }
}

/**
 * Process a cycle
 * Cycle is incremented
 * The current crt_index is the same number as the cycle but the cycle start at 1 eand the crt_index is starting at 0 ( and so remove 1 )
 * Then we match on the crt_index % 40 because a line is 40 char long and we want to match the crt_index within the current line not all the crt
 * If the "line crt_index" is between x - 1 and x + 1 it mean that at this current position there is one of the "3 pixel long sprite" then we add a '#'
 * If the "line crt_index" is not between x - 1 and x + 1 it mean that there is no sprite pixel at this position then add a '.'
 * Then if the cycle % 40 == 0 it mean that the current line if filled and a new line on the crt need to be added
 *
 * Note : First two line are like this to keep a logic with the exercice.
 */
fn process_cycle(cycle: &mut i32, x: i32, crt: &mut Vec<String>) {
    *cycle += 1;

    let crt_index = *cycle - 1;

    match crt_index % 40 {
        n if n >= x - 1 && n <= x + 1 => crt.last_mut().unwrap().push('#'),
        _ => crt.last_mut().unwrap().push('.'),
    }

    if *cycle % 40 == 0 {
        crt.push(String::new());
    }
}
