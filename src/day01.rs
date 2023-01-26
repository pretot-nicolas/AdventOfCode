use crate::resource_loader;

/*
 * In part1 we loop throught the data and sum all line until we found an empty line
 * If an Empty line is found, check if the current sum is gretter than the max already stored.
 */
pub fn part_one() {
    let lines = resource_loader::load_resource("day01-data.txt");
    let mut max_count: i32 = 0;
    let mut current_count: i32 = 0;

    // Computings sums & Max
    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            if current_count > max_count {
                max_count = current_count;
            }

            current_count = 0;
        } else {
            current_count += line.parse::<i32>().expect("Error parsing number");
        }
    }

    // Last Elf
    if current_count > max_count {
        max_count = current_count;
    }

    println!("Max calories on one single elf is : {max_count}");
}

/*
 * In part2 we use a vector to store all sum calories of all elf.
 * Then we sort the list, reverse it and take the 3 first element.
 */
pub fn part_two() {
    let lines = resource_loader::load_resource("day01-data.txt");
    let mut vec = Vec::new();
    let mut current_count = 0;

    // Fillings vector with sum of all elf distincs by elfs
    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            vec.push(current_count);
            current_count = 0;
        } else {
            current_count += line.parse::<i32>().expect("Error parsing number");
        }
    }

    // last sum
    vec.push(current_count);

    // Sort + reverse (for convenience)
    vec.sort();
    vec.reverse();

    // The three first element ( if they exist ) are the top 3 carrying elfs
    if Vec::len(&vec) >= 3 {
        let total = vec[0] + vec[1] + vec[2];

        println!("Top3 carrying elf are carrying {total} calories");
    }
}
