use crate::resource_loader;
use std::{cmp::max };

/**
 * In "PartOne" we parse the map as a Vec<Vec<u32>>
 * Visibility is stored inside a struct. By default all trees are visible.
 * Then we loop thought all the map and we check the visibility form the edges to the current tree :
 *  - From the edges of the map we check each trees in the direction of the current trees.
 *  - If any tree is higher then we set the visibility to false for this direction.
 *
 * The two first for loop start at 1 and ends a len -1 so we remove the outer trees
 * Outer trees are added with the formula : 4 * len - 4 ( where len is the len of the map)
 *
 * For this exercice, and according to the data. We assume that the map is a perfect square.
 */
pub fn part_one() {
    let data = resource_loader::load_resource("day08-data.txt");

    let tree_map = parse(data);
    let mut visible_tree_count = 0;

    for row in 1..tree_map.len() - 1 {
        for column in 1..tree_map[row].len() - 1 {
            let tree_height = tree_map[row][column];
            let mut visibility = Visibility {
                left: true,
                right: true,
                top: true,
                bottom: true,
            };

            for i in 0..tree_map.len() {
                for j in 0..tree_map[i].len() {
                    if i == row && j == column {
                        continue;
                    }
                    if i == row {
                        if j < column && tree_map[i][j] >= tree_height {
                            visibility.left = false;
                        } else if j > column && tree_map[i][j] >= tree_height {
                            visibility.right = false;
                        }
                    } else if j == column {
                        if i < row && tree_map[i][j] >= tree_height {
                            visibility.top = false;
                        } else if i > row && tree_map[i][j] >= tree_height {
                            visibility.bottom = false;
                        }
                    }
                }
            }

            if visibility.left || visibility.right || visibility.top || visibility.bottom {
                visible_tree_count += 1;
            }
        }
    }

    visible_tree_count += 4 * tree_map.len() - 4;

    dbg!(visible_tree_count);
}

/**
 * Parse each line and maps them as a Vec<Vec<u32>>
 */
fn parse(lines: Vec<String>) -> Vec<Vec<u32>> {
    let data = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    return data;
}

/**
 * visibility struct used to store visibility on each direction.
 */
struct Visibility {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

/**
 * Scenic score struct used to store scenic score for each direction
 */
struct ScenicScore {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

/**
 * In "PartTwo" we check scenic score for each trees.
 * As for PartOne we loop thought all the map after parsing the raw data.
 * The main difference heere is that we check height from the trees to the edge
 * Mostly boilerplate code to check in each direction.
 */
pub fn part_two() {
    let data = resource_loader::load_resource("day08-data.txt");

    let tree_map = parse(data);
    let mut max_scenic_score = 0;

    for row in 0..tree_map.len() {
        for column in 0..tree_map[row].len() {
            let tree_height = tree_map[row][column];
            let mut scenic_score = ScenicScore {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            };

            for i in (0..row).rev() {
                scenic_score.top += 1;
                if tree_height <= tree_map[i][column] {
                    break;
                }
            }

            for i in row + 1..tree_map.len() {
                scenic_score.bottom += 1;
                if tree_height <= tree_map[i][column] {
                    break;
                }
            }

            for j in (0..column).rev() {
                scenic_score.left += 1;
                if tree_height <= tree_map[row][j] {
                    break;
                }
            }

            for j in column + 1..tree_map[row].len() {
                scenic_score.right += 1;
                if tree_height <= tree_map[row][j] {
                    break;
                }
            }
            let tree_scenic_score =
                scenic_score.left * scenic_score.right * scenic_score.top * scenic_score.bottom;
            max_scenic_score = max(tree_scenic_score, max_scenic_score);
        }
    }

    dbg!(max_scenic_score);
}
