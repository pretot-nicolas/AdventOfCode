use crate::resource_loader;
use std::collections::HashMap;

pub fn part_one() {
   let result = process(4);

   println!("Finale result is {result}");
}

pub fn part_two() {
    let result = process(14);
 
    println!("Finale result is {result}");
 }

 
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

fn has_duplicate_char(str: &str) -> bool {

    let mut char_count = HashMap::new();
    for c in str.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    char_count.values().any(|&x| x > 1)

}