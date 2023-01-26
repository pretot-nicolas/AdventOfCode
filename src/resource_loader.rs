use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * Read data file
 */
pub fn load_resource(resource_name: &str) -> Vec<String> {
    let file = File::open(format!("Resources\\{resource_name}")).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    return lines;
}
