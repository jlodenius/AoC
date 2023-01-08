use std::fs;

fn get_value_at_cycle(cycle: i32, cycles: &[i32]) -> i32 {
    let mut value = 1;
    let mut command_executed_at = 0;

    for command in cycles {
        if command == &0 {
            command_executed_at += 1;
        } else {
            command_executed_at += 2;
        }
        if command_executed_at > cycle {
            break;
        }
        value += command;
    }

    println!("cycle is {} and value is {}", cycle, value);

    return value * cycle;
}

fn main() {
    let file_path = "day10_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut commands: Vec<i32> = vec![];

    for line in contents.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [_add, amount] => {
                commands.push(amount.parse::<i32>().unwrap());
            }
            [_noop] => {
                commands.push(0);
            }
            _ => panic!("fail"),
        }
    }

    let mut cycles: Vec<i32> = vec![0];

    for value in commands {
        if value == 0 {
            cycles.push(value);
        } else {
            cycles.push(value);
        }
    }

    let result: i32 = [20, 60, 100, 140, 180, 220]
        .map(|s| get_value_at_cycle(s, &cycles))
        .iter()
        .sum();

    println!("The value: {}", result);
}
