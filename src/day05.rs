use crate::resource_loader;
extern crate regex;
use regex::Regex;

/**
 * In "Day05" we use the same function to do "PartOne" and "PartTwo"
 * First we set the "is_parsing_dock" as true. This will keep track of the current state of parsing.
 * Then we loop through the data :
 * If "is_parsing_dock" is true :
 * - we check if it should be false or if the current line is the numeric line.
 * - we fill each stack with vlaue of these line (see fill_stack)
 * If "is_parsing_dock" is false :
 * - we read a "move" line and perform the according move operation
 *
 * To switch from "PartOne" to "PartTwo", the function called in "parse_and_perform_move" need to be changed accordingly
 */
pub fn part_one_and_two() {
    let lines = resource_loader::load_resource("day05-data.txt");
    let mut dock: Vec<Vec<char>> = Vec::new();
    let mut dock1: Vec<Vec<char>> = Vec::new();
    let mut dock2: Vec<Vec<char>> = Vec::new();
    let mut is_parsing_dock = true;

    for line in lines {
        if is_parsing_dock {
            if line.is_empty() {
                is_parsing_dock = false;
                dock1 = dock.clone();
                dock2 = dock.clone();
                continue;
            } else if line.contains("1") {
                continue;
            } else {
                fill_stacks(&line, &mut dock);
            }
        } else {
            parse_and_perform_move(&line, &mut dock1, &mut dock2);
        }
    }

    // Part 1
    let mut final_str1 = String::new();

    for stacks in dock1 {
        final_str1.push(*stacks.first().unwrap());
    }

    // Part 2
    let mut final_str2 = String::new();

    for stacks in dock2 {
        final_str2.push(*stacks.first().unwrap());
    }

    println!("Finale sentence for part one is {final_str1}");
    println!("Finale sentence for part two is {final_str2}");
}

/**
 * Parsing a line of dock definition and adding crate to the according stack
 * The regex "((\\[[A-Z]{1}\\]|[ ]{3})[ ]{0,1})" create a cature group for each 3 char seprarated by a space.
 * For each match we add the second digit ( the letter ) inside the correct stack.
 * As the absence of crate on a stack produce a match group of "   ", we can use the index of the group match to find the correct stack.
 * Trimming this "   " will produce an is_empty and this "absence of crate" value will nott be added to any stack
 */
fn fill_stacks(line: &String, dock: &mut Vec<Vec<char>>) {
    let regex = Regex::new("((\\[[A-Z]{1}\\]|[ ]{3})[ ]{0,1})").unwrap();
    let matches: Vec<regex::Match> = regex.find_iter(line).collect();
    let mut stack_id = 0;

    while stack_id < matches.len() {
        if dock.len() <= stack_id {
            dock.push(Vec::new());
        }

        let current_match = matches[stack_id].as_str().trim();

        if current_match.is_empty() == false {
            let letter = current_match.chars().nth(1).unwrap();
            dock[stack_id].push(letter);
        }

        stack_id += 1;
    }
}

/**
 * Parse and perform crate moves.
 * Regex "(\\d+)" only match numeric values.
 * So in order the count value, the from stack id and the to stack id
 * As stack id start at 1 in dataset we substract 1 to match vector's id
 */
fn parse_and_perform_move(line: &String, dock1: &mut Vec<Vec<char>>, dock2: &mut Vec<Vec<char>>) {
    let regex = Regex::new("(\\d+)").unwrap();
    let matches: Vec<regex::Match> = regex.find_iter(line).collect();

    if matches.len() == 3 {
        let count: usize = matches[0].as_str().parse().unwrap();
        let from: usize = matches[1].as_str().parse().unwrap();
        let to: usize = matches[2].as_str().parse().unwrap();

        perform_move_part_one(dock1, from - 1, to - 1, count);
        perform_move_part_two(dock2, from - 1, to - 1, count);
    }
}

/**
 * Perform move according to parameters
 * Remove a range from 0 to count crate from the "from stack"
 * Add this new crates vector to the start of the "to stack"
 * As the crane can only move one crate at a time, the vector need to be reversed to have correct crate order
 */
fn perform_move_part_one(dock: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    let moved_crates = dock[from].drain(0..count).collect::<Vec<_>>();
    dock[to].splice(0..0, moved_crates.into_iter().rev());
}

/**
 * Perform move according to parameters
 * Remove a range from 0 to count crate from the "from stack"
 * Add this new crates vector to the start of the "to stack"
 * As the crane can move multiple crate at a time, the vector can be kept like that
 */
fn perform_move_part_two(dock: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    let moved_crates = dock[from].drain(0..count).collect::<Vec<_>>();
    dock[to].splice(0..0, moved_crates.into_iter());
}

