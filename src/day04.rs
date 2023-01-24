use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * In "PartOne" we loop through the data. For each line, we split by the ',' to sperate ranges.
 * For each ranges as STR we parse them to a Tuple<i32, i32>
 * We then call the range_contain function that check the boundary of ranges to check if the first fully contain the other.
 * We change the number of overlapp accordingly 
 */
pub fn part_one() {
    let lines = get_data();
    let mut number_of_full_overlap = 0;

    for line in lines {
        let line = line.trim();
        let sections:Vec<&str> = line.split(',').collect();
    
        if sections.len() == 2 {
            let range1 = parse_sections_as_tuple(sections[0]);
            let range2 = parse_sections_as_tuple(sections[1]);
  
            if range_contains(&range1, &range2) || range_contains(&range2, &range1) {
                number_of_full_overlap += 1;
            }
        }
    }

    println!("Number of full overlapping is {number_of_full_overlap}");
}

/**
 * In "PartTwo" we loop through the data. For each line, we split by the ',' to sperate ranges.
 * For each ranges as STR we parse them to a Tuple<i32, i32>
 * We then call the range_distinct function that check the boundary of ranges to find if they do not overlap
 */
pub fn part_two() {
    let lines = get_data();
    let mut number_of_pair_overlap = 0;

    for line in lines {
        let line = line.trim();
        let sections:Vec<&str> = line.split(',').collect();
    
        if sections.len() == 2 {
            let range1 = parse_sections_as_tuple(sections[0]);
            let range2 = parse_sections_as_tuple(sections[1]);
  
            if range_distinct(&range1, &range2) == false {
                number_of_pair_overlap += 1;
            }
        }
    }

    println!("Number of pair overlapping is {number_of_pair_overlap}");
}

/**
 * Parse range with format "start-end" (exemple "4-9") and change it to a tuple<i32,i32>
 */
fn parse_sections_as_tuple(section: &str) -> (i32, i32) {
    let sections:Vec<i32> = section.split('-').map(|x| x.parse().unwrap()).collect();

    if sections.len() == 2 {
        return (sections[0], sections[1]);
    }

    return (0,0)
}

/**
 * Check if the first range fully contain the second one
 * Check if the start of the first range in smaller that the second one et if it end is greater the the second one
 */
fn range_contains(range1 : &(i32, i32), range2 : &(i32, i32)) -> bool {
    return range1.0 <= range2.0 && range1.1 >= range2.1;
}

/**
 * Check if the two range are fully distinct and does not overlap
 * Check if the end of the first range is smaller than the start of the second range
 * Or if the end of the second range is smaller than the start of the first range
 */
fn range_distinct(range1 : &(i32, i32), range2 : &(i32, i32)) -> bool {
    return range1.1 < range2.0 || range2.1 < range1.0;
}

/**
 * Read data file
 */
 fn get_data() -> Vec<String> {
    let file = File::open("Resources\\day04-data.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    return lines;
}