use crate::resource_loader;
use std::collections::HashMap;

/**
 * Day07 context :
 *
 * In Day07 my first choice was to use a Tree structure to store all directory and calculate size of each nodes
 * I first tried that then I realized that there was other ways much more simpler to do it.
 * So I ended up using a simple HashMap to store each full path and their size.
 */

/**
 * In "PartOne" we use the get_all_path_size function to generate the HashMap<String, I32> to store all paths and their size
 * Then we loop  on all paths and if their size is lesser than 100000 then their size are added
 */
pub fn part_one() {
    let all_path_size = get_all_path_size();
    let mut final_size = 0;

    for path in all_path_size {
        if path.1 <= 100000 {
            final_size += path.1;
        }
    }

    println!("Sum of all directory lesser than 100000 is {final_size}");
}

/**
 * In "PartTwo" we use the get_all_path_size function to generate the HashMap<String, I32> to store all paths and their size
 * Then we find the amount of size to free from disk
 * Then the list of paths is filtered to get all paths that are bigger than the size_to_free
 * The smallest of these directory is then the smallest directory to remove to have 30000000 free space
 */
pub fn part_two() {
    let all_path_size = get_all_path_size();

    let max_size = 70000000;
    let needed_size = 30000000;
    let current_used_size = all_path_size.get(&"/".to_string()).unwrap();
    let available_size = max_size - current_used_size;
    let size_to_free = needed_size - available_size;

    let candidate = all_path_size.iter().filter(|x| x.1 >= &size_to_free);
    let folder_to_remove = candidate
        .min_by(|(_, size_a), (_, size_b)| size_a.cmp(size_b))
        .unwrap();
    let size_of_folder = folder_to_remove.1;

    println!("Size of directory that need to be removed is {size_of_folder}");
}

/**
 * Here we create the Hashmap of all path and their size.
 * To keep track of this, we use :
 *  - the "paths" variable that store all path encountered and their size.
 *  - the "current_path" variable that keep track of all folder that build this current path
 * Then :
 *  - Each line are divided in tokens exemple a line change from "$ cd folderA" to ["$", "cd", "folderA"]
 *  - Revelent token are checked (other one are not giving the context of "execice") :
 *    - "cd /" clear the current_path. Then restart path building by adding the "/" in the current_path
 *    - "cd .." remove the last element added in the "current_path"
 *    - "cd <name>" add a new part in the current_path
 *    - Everything that is not a command and not a "dir" are interpreted as a file in this "exercice" context
 *      It's size need to be added to the current folder so the "current_folder" variable is used to retrieve the entry in the HashMap
 *      The value of the entry is changed to it's old value + the size of the current folder.
 */
fn get_all_path_size() -> HashMap<String, i32> {
    let lines = resource_loader::load_resource("day07-data.txt");

    let mut paths: HashMap<String, i32> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();

    for line in lines {
        let line = line;
        let tokens: Vec<&str> = line.split(' ').collect();

        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == "/" {
                    current_path.clear();
                    current_path.push("/".to_string());
                } else if tokens[2] == ".." {
                    current_path.pop();
                } else {
                    current_path.push(tokens[2].to_string());
                }
            }
        } else if tokens[0] != "dir" {
            let size = tokens[0].parse::<i32>().unwrap();

            for i in 0..current_path.len() {
                let full_path = current_path[0..=i].join("/");

                *paths.entry(full_path).or_insert(0) += size;
            }
        }
    }

    return paths;
}
