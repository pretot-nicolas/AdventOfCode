use crate::resource_loader;

pub fn part_one() {
    let lines: Vec<String> = resource_loader::load_resource("day11-data.txt");

    let all_mokey_data: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|block| block.to_vec())
        .collect();

    for monkey_data in all_mokey_data {
        parse_monkey(monkey_data);
    }
}

pub fn part_two() {}

fn parse_monkey(data: Vec<String>) {
    for line in data {
        match line.trim() {
            l if l.starts_with("Monkey") => {}
            l if l.starts_with("Starting items") => {}
            l if l.starts_with("Operation") => {}
            l if l.starts_with("Test:") => {}
            l if l.starts_with("If true") => {}
            l if l.starts_with("If false") => {}
            _ => {}
        }
    }
}

// struct Monkey {
//     items: Vec<i32>,
//     test: TestOperation,
//     monkey_test_true: i32,
//     monkey_test_false: i32,
// }

// struct TestOperation {
//     operator: Operator,
//     operand: Operand,
// }

// enum Operator {
//     Add,
//     Multiply,
// }

// enum Operand {
//     Number(i32),
//     OldValue
// }
