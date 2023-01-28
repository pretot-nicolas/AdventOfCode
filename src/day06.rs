use crate::resource_loader;
use std::collections::HashMap;

/**
 * Run Day06 with the size_of_delimiter equal to 4
 */
pub fn part_one() {
    let result = process(4);

    println!("Finale result is {result}");
}

/**
 * Run Day06 with the size_of_delimiter equal to 14
 */
pub fn part_two() {
    let result = process(14);

    println!("Finale result is {result}");
}

/**
 * Load the resource, loop on the string.
 * Each loop we take a part of the string and check if it has duplicate char.
 * The len of the string is of size "size_of_delimiter" and the part of the string that is checked
 * is the current position in the string - size_of_delimiter
 */
fn process(len_of_delimiter: usize) -> usize {
    let data = resource_loader::load_resource("day06-data.txt");
    let data_stream = data.first().unwrap();
    let mut i: usize = 0;

    while i < data_stream.len() {
        if i >= (len_of_delimiter - 1) {
            let start_range = i - (len_of_delimiter - 1);

            if has_duplicate_char(&data_stream[start_range..=i]) == false {
                return i + 1;
            }
        }

        i += 1;
    }

    return 0;
}

/**
 * Check if a string has duplicate chars by creating a HashMap that will have each unique char
 * that are in the string as the key and the number of time they appear as the value.
 *
 * If any key has a value greater than 1 it mean that their is duplicate chars
 */
fn has_duplicate_char(str: &str) -> bool {
    let mut char_count = HashMap::new();
    for c in str.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    char_count.values().any(|&x| x > 1)
}
