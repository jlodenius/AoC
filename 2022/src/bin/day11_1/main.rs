use std::{cmp::Reverse, fs, str::Lines};

#[derive(Debug, Clone)]
struct Monkey<'a> {
    items: Vec<u32>,
    times_inspected: u32,
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
        // Really ugly parser, but it works
        match the_split[..] {
            ["  Starting items", rest] => {
                items = rest
                    .split_terminator(",")
                    .map(|num_str| num_str.trim().parse::<u32>().unwrap())
                    .collect();
            }
            ["  Operation", rest] => {
                operation = rest.split_whitespace().rev().take(2).collect::<Vec<&str>>();
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
        times_inspected: 0,
        operation,
        divide_by,
        if_true,
        if_false,
    };
}

fn do_round(monkeys: &mut [Monkey]) {
    for idx in 0..monkeys.len() {
        // Have to copy the monkey to be able to iter over the items
        // and still have access to values inside monkeys
        let monkey_copy;
        let monkey = &mut monkeys[idx];
        monkey_copy = monkey.clone();
        monkey.times_inspected += monkey.items.len() as u32;

        for item_to_inspect in monkey_copy.items.iter().copied() {
            let mut item_worry_level = match monkey_copy.operation[..] {
                ["old", "*"] => item_to_inspect * item_to_inspect,
                [x, "+"] => item_to_inspect + x.parse::<u32>().unwrap(),
                [x, "*"] => item_to_inspect * x.parse::<u32>().unwrap(),
                _ => {
                    panic!("Fail");
                }
            };
            item_worry_level /= 3;

            let passes_test = item_worry_level % monkey_copy.divide_by == 0;

            let floored_value = item_worry_level as f32;
            item_worry_level = floored_value.floor() as u32;

            let throw_to = match passes_test {
                true => monkey_copy.if_true,
                false => monkey_copy.if_false,
            };

            monkeys[throw_to].items.push(item_worry_level);

            println!(
                "Monkey {} throws item with worry level {} to Monkey {}",
                idx, item_worry_level, throw_to
            )
        }

        monkeys[idx].items.clear();
    }
}

fn main() {
    let file_path = "day11_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let iterator = &mut contents.lines();
    let mut monkeys: Vec<Monkey> = vec![];
    for _ in 0..=7 {
        monkeys.push(create_monkey(iterator));
    }

    for _round in 0..20 {
        do_round(&mut monkeys);
    }

    let mut inspected_times_vec = monkeys
        .iter()
        .map(|monkey| monkey.times_inspected)
        .collect::<Vec<u32>>();

    inspected_times_vec.sort_by_key(|&c| Reverse(c));
    let monkey_business = inspected_times_vec.into_iter().take(2).product::<u32>();

    println!("Level of monkey business: {}", monkey_business);
}
