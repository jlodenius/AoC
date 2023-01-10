use std::{fs, str::Lines};

#[derive(Debug)]
struct Monkey<'a> {
    items: Vec<u32>,
    operation: Vec<&'a str>,
    divide_by: u32,
    if_true: usize,
    if_false: usize,
}

fn create_monkey<'a>(iterator: &mut Lines<'a>) -> Monkey<'a> {
    let mut items: Vec<u32> = vec![];
    let mut operation: Vec<&str> = vec![];
    let mut divide_by = 0;
    let mut if_true = 0;
    let mut if_false = 0;

    for line in iterator.take(7) {
        if line.starts_with("Monkey") || line == "" {
            continue;
        }
        let the_split: Vec<&str> = line.split_terminator(":").collect();
        match the_split[..] {
            ["  Starting items", rest] => {
                items = rest
                    .split_terminator(",")
                    .map(|num_str| num_str.trim().parse::<u32>().unwrap())
                    .collect();
            }
            ["  Operation", rest] => {
                operation = rest.split_whitespace().rev().take(2).collect::<Vec<&str>>();
                // match operations[..] {
                //     ["old", "*"] => {
                //         operation = Some(Box::new(|item_worry: u32| item_worry * item_worry));
                //     }
                //     [x, "+"] => {
                //         operation = Some(Box::new(|item_worry: u32| {
                //             item_worry + x.parse::<u32>().unwrap()
                //         }));
                //     }
                //     [x, "*"] => {
                //         operation = Some(Box::new(|item_worry: u32| {
                //             item_worry * x.parse::<u32>().unwrap()
                //         }));
                //     }
                //     _ => {
                //         panic!("Fail");
                //     }
                // }
            }
            ["  Test", rest] => {
                let test: u32 = rest
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .parse()
                    .unwrap();
                divide_by = test;
            }
            ["    If true", rest] => {
                let throw_to: usize = rest
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .parse()
                    .unwrap();
                if_true = throw_to;
            }
            ["    If false", rest] => {
                let throw_to: usize = rest
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .parse()
                    .unwrap();
                if_false = throw_to;
            }
            _ => {
                println!("NONE");
            }
        }
    }

    return Monkey {
        items,
        operation,
        divide_by,
        if_true,
        if_false,
    };
}

fn main() {
    let file_path = "day11_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let iterator = &mut contents.lines();
    let mut monkeys: Vec<Monkey> = vec![];

    for _ in 0..7 {
        monkeys.push(create_monkey(iterator));
    }

    println!("{:?}", monkeys);
}
