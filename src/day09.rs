use crate::resource_loader;
use std::collections::HashSet;

/**
 * In PartOne we parse all the data into a struct of "Direction" & "Count".
 * Then we loop thought all the comands and for each of them we move the Head represented by a struct Coordinate ( x, y )
 * Each time the head move the tail is also moved to follow the head then the position is added to a Hahset
 * Thanks to the hashset that remove duplicate, the len of this HashSet is the number of unique position.
 */
pub fn part_one() {
    let data = resource_loader::load_resource("day09-data.txt");

    let command_list = parse_data(data);
    let mut head = Coordinate { x: 0, y: 0 };
    let mut tail = Coordinate { x: 0, y: 0 };
    let mut history: HashSet<Coordinate> = HashSet::new();

    for command in command_list {
        for _ in 0..command.count {
            move_head(&command.direction, &mut head);
            move_tail(&head, &mut tail);

            history.insert(tail);
        }
    }

    let number_of_position = history.len();
    println!("The number of unique position that the tail visited is {number_of_position}");
}

/**
 * In PartTwo, we do the same as in the PartOne expect the tail is no more a Coordinate but a Vec<Coordinate>.
 * The tail vector is populated with 9 elements and each element is a knot.
 * Each time the head moves, the first tails' knot is move to follow the head .
 * Then we use the same function to move the next knot considering the previous knot as the Head
 * The position of the last knot is then stored in HashSet to know the number of unique position it moved to
 *
 * Note : As we pass the tails vector to the move_tail function, it is borrowed both as mutable and imutable.
 * => To keep this function as we did it in PartOne we clone the current tail coordinate value
 */
pub fn part_two() {
    let data = resource_loader::load_resource("day09-data.txt");

    let command_list = parse_data(data);
    let mut head = Coordinate { x: 0, y: 0 };
    let mut tails: Vec<Coordinate> = Vec::new();
    let mut history: HashSet<Coordinate> = HashSet::new();

    for _ in 0..9 {
        tails.push(Coordinate { x: 0, y: 0 })
    }

    for command in command_list {
        for _ in 0..command.count {
            move_head(&command.direction, &mut head);
            move_tail(&head, &mut tails[0]);

            for y in 1..9 {
                move_tail(&tails[y - 1].clone(), &mut tails[y])
            }

            history.insert(tails[8]);
        }
    }

    let number_of_position = history.len();
    println!("The number of unique position that the last knot on the tail visited is {number_of_position}");
}

/**
 * Move the tail according to the head position.
 * In PartTwo we use a knot as the head position.
 */
fn move_tail(head: &Coordinate, tail: &mut Coordinate) {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;

    if diff_x.abs() > 1 || diff_y.abs() > 1 {
        if diff_x > 0 {
            tail.x += 1;
        } else if diff_x < 0 {
            tail.x -= 1;
        }

        if diff_y > 0 {
            tail.y += 1;
        } else if diff_y < 0 {
            tail.y -= 1;
        }
    }
}

/**
 * Move the head according to the data parsed from the file
 */
fn move_head(direction: &Direction, head: &mut Coordinate) {
    match direction {
        Direction::Up => head.y -= 1,
        Direction::Right => head.x += 1,
        Direction::Down => head.y += 1,
        Direction::Left => head.x -= 1,
    }
}

/**
 * Store coordinate.
 * Must use Eq, Hash,PartialEq for the HashSet insert
 * Must use  Clone, Copy for the move_tail function
 */
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

/**
 * Store Commands parsed from data
 */
struct Command {
    direction: Direction,
    count: i32,
}

/**
 * Store direction
 */
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

/**
 * Construct Direction from chart
 */
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            _ => panic!("Invalid direction character"),
        }
    }
}

/**
 * Parse the data from the input file as a Vector of commands
 */
fn parse_data(data: Vec<String>) -> Vec<Command> {
    data.iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let direction = Direction::from(line.chars().next().unwrap());
            let count = line[1..].trim().parse().unwrap();
            Command { direction, count }
        })
        .collect()
}
