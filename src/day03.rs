use crate::resource_loader;

/*
 * In Part1 we loop thought all lines and for each line we split it into part by half
 * Then we convert each part to a vector of unique char
 * Then we found the duplicate ( based on the fact that there can onnly be one )
 * And we get it priority value.
 * Then we sum all of them.
 */
pub fn part_one() {
    let lines = resource_loader::load_resource("day03-data.txt");
    let mut sum_of_priority = 0;

    for line in lines {
        let line = line.trim();
        let first_comp = &line[..line.len() / 2];
        let second_comp = &line[line.len() / 2..];

        let first_vec = string_to_vec(first_comp);
        let second_vec = string_to_vec(second_comp);

        let unique_duplicate = get_unique_duplicate_item(&first_vec, &second_vec);

        sum_of_priority += get_priority_value(unique_duplicate)
    }

    println!("Sum of all priority is {sum_of_priority}");
}

/*
 * In Part2 we loop thought all lines and we store each line as a vector of unique char in a "working_group" variable.
 * When the group reach 3 elfs, we call a new function "get_duplicates_item" which return a list of duplicate between 2 vectors
 * That way we can reuse the function from part1 'get_unique_duplicate_item'.
 * Then we reuse the function to get an unique duplicate between the list of duplicate (elf1 againt elf2) and the elf3 of the group.
 * And we get it priority value.
 * Then we sum all of them.
 * Then the group is cleared to start another one.
 */
pub fn part_two() {
    let lines = resource_loader::load_resource("day03-data.txt");
    let mut working_group: Vec<Vec<char>> = Vec::new();
    let mut sum_of_priority = 0;

    for line in lines {
        let unique_vec = string_to_vec(line.trim());
        working_group.push(unique_vec);

        if working_group.len() == 3 {
            let first_second_duplicate = get_duplicates_item(&working_group[0], &working_group[1]);
            let unique_duplicate =
                get_unique_duplicate_item(&first_second_duplicate, &working_group[2]);

            sum_of_priority += get_priority_value(unique_duplicate);

            working_group.clear();
        }
    }

    println!("Sum of all priority is {sum_of_priority}");
}

/**
 * Convert &str to vector of unique char.
 */
pub fn string_to_vec(str: &str) -> Vec<char> {
    let mut vec: Vec<char> = Vec::new();

    for c in str.chars() {
        if vec.iter().any(|x| c == *x) == false {
            vec.push(c)
        }
    }

    return vec;
}

/**
 * Return the first duplicate item that exist in the two vector.
 */
fn get_unique_duplicate_item(vec1: &Vec<char>, vec2: &Vec<char>) -> char {
    for c in vec1 {
        if vec2.contains(c) {
            return *c;
        }
    }

    return '_';
}

/**
 * Return the vector of duplicate items between two vectors
 */
fn get_duplicates_item(vec1: &Vec<char>, vec2: &Vec<char>) -> Vec<char> {
    let mut duplicates: Vec<char> = Vec::new();

    for c in vec1 {
        if vec2.contains(c) {
            duplicates.push(*c);
        }
    }

    return duplicates;
}

/**
 * Return the correct priority value for a specifique item
 */
fn get_priority_value(item: char) -> i32 {
    let lower_normalize = 'a' as u8;
    let upper_normalize = 'A' as u8;

    match item {
        'a'..='z' => (item as u8 - lower_normalize + 1) as i32,
        'A'..='Z' => (item as u8 - upper_normalize + 27) as i32,
        _ => 0,
    }
}
