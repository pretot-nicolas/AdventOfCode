use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * In part one, we loop through the data. For each line, we take both letters separately. 
 * In the first version, we used these letters (as specified at the end of the file), but it was too verbose and lacked readability. 
 * Therefore, this code was refactored when creating part two.
 * After the rework, we use the letters to change the letter variable to an enumeration value of Shape. 
 * We then call a function, get_outcome, to determine the outcome of the round. 
 * Afterwards, we retrieve the score for the shape and outcome and add it to the score variable.
*/
pub fn part_one() {
    let lines = get_data();

    let mut score : i32 = 0;

    for line in lines {
        let line = line.trim();
        let shapes: Vec<char> = line.split(' ').map(|s| s.chars().next().unwrap()).collect();

        if shapes.len() == 2 {
            let opponent_shape = letter_to_shape_part1(&shapes[0]);
            let my_shape = letter_to_shape_part1(&shapes[1]);

            let outcome = get_outcome(&my_shape, &opponent_shape);
            
            score += get_shape_score(&my_shape);
            score += get_outcome_score(&outcome);
        }
    }

    println!("Finale score is {score}");
}

/**
 * Most of the code is duplicated for sepration of the "parts" and readability for theses tests
 * As the first part, we loop through the data
 * For each line we take the first letter and use this as the opponent shape.
 * Then the second letter is used as the outcom that we whant to obtain.
 * We then convert these two char as an enum. ( shape & Outcome )
 * Then we get the correct letter to play on this round and obtain the wanted outcome.
 * Like part1 the score is then computed.
 */
pub fn part_two() {
    let lines = get_data();

    let mut score : i32 = 0;

    for line in lines {
        let line = line.trim();
        let shapes: Vec<char> = line.split(' ').map(|s| s.chars().next().unwrap()).collect();

        if shapes.len() == 2 {
            let opponent_shape = letter_to_shape_part2(&shapes[0]);
            let target_outcome = letter_to_outcome(&shapes[1]);

            let my_shape = get_shape_for_outcome(&opponent_shape, &target_outcome);

            score += get_shape_score(&my_shape);
            score += get_outcome_score(&target_outcome);
        }
    }

    println!("Finale score is {score}");
}

/**
 * Enum that represent shapes
 */ 
enum Shape {
    None,
    Rock,
    Paper,
    Cisor
}

/**
 * Enum that represent Outcomes
 */
enum Outcome {
    None,
    Loose,
    Win,
    Draft
}

/**
 * Convert char to a Shape enum value. In part 1 we use X, Y, Z as Shapes 
 */
fn letter_to_shape_part1(letter: &char) -> Shape {
    match letter {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Cisor,
        _ => Shape::None
    }
}

/**
 * Convert char to a Shape enum value.
 */
fn letter_to_shape_part2(letter: &char) -> Shape {
    match letter {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Cisor,
        _ => Shape::None
    }
}

/**
 * Convert char to Outcome enum value
 */
fn letter_to_outcome(letter: &char) -> Outcome {
    match letter {
        'X' => Outcome::Loose,
        'Y' => Outcome::Draft,
        'Z' => Outcome::Win,
        _ => Outcome::None
    }
}

/**
 * Return correct score value for each shapes
 */
fn get_shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Cisor => 3,
        _ => 0
    }
}

/**
 * Get the outcome of a round using both shapes
 */
fn get_outcome(my_shape: &Shape, opponent_shape : &Shape) -> Outcome {
    match (my_shape, opponent_shape) {
        (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Cisor, Shape::Cisor) => Outcome::Draft,
        (Shape::Rock, Shape::Cisor) | (Shape::Paper, Shape::Rock) | (Shape::Cisor, Shape::Paper) => Outcome::Win,
        _ => Outcome::Loose
    }
}

/**
 * Return the correct score value for each outcome
 */
fn get_outcome_score(outcom: &Outcome) -> i32 {
    match outcom {
        Outcome::Loose => 0,
        Outcome::Draft => 3,
        Outcome::Win => 6,
        _ => 0
    }
}

/**
 * return the correct shape to play on a round to match the target outcome
 */
fn get_shape_for_outcome(opponent_shape: &Shape, target_outcome: &Outcome) -> Shape {
    match (opponent_shape, target_outcome) {
        (Shape::Rock, Outcome::Draft) | (Shape::Paper, Outcome::Loose) | (Shape::Cisor, Outcome::Win) => Shape::Rock,
        (Shape::Rock, Outcome::Win) | (Shape::Paper, Outcome::Draft) | (Shape::Cisor, Outcome::Loose) => Shape::Paper,
        (Shape::Rock, Outcome::Loose) | (Shape::Paper, Outcome::Win) | (Shape::Cisor, Outcome::Draft) => Shape::Cisor,
        _ => Shape::None
    }
}

/**
 * Read data file
 */
fn get_data() -> Vec<String> {
    let file = File::open("Resources\\day02-data.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    return lines;
}


// pub fn part_one() {
//     let data = get_test_data();

//     let mut score : i32 = 0;

//     for line in data.lines() {
//         let line = line.trim();
//         let shapes: Vec<char> = line.split(' ').map(|s| s.chars().next().unwrap()).collect();

//         if shapes.len() == 2 {
//             let opponent_shape = shapes[0];
//             let my_shape = shapes[1];
            
//             score += get_shape_score(my_shape);
//             score += get_outcome_score(opponent_shape, my_shape);
//         }
//     }

//     println!("Finale score is {score}");
// }


// fn get_shape_score(shape: char) -> i32 {
//     match (shape)  {
//         // Rock
//         'X' => 1,
//         // Paper
//         'Y' => 2,
//         // Cisor
//         'Z' => 3,
//         _ => 0
//     }
// }

// fn get_outcome_score(opponent_shape: char, my_shape: char) -> i32 {
//     match (opponent_shape, my_shape)  {
//         // Rock vs Cisor || Paper vs Rock || Cisor vs Paper => Loose
//         ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
//         // Rock vs Rock || Paper vs Paper || Cisor vs Cisor => Draft  
//         ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
//         // Cisor vs Rock || Rock vs Paper || Paper vs Cisor => Win  
//         ('C', 'X') | ('A', 'Y') | ('B', 'Z') => 6,
//         _ => 0
//     }
// }