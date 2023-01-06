use std::collections::HashMap;
use std::fs;

fn move_amount_from_to(
    amount: &i32,
    from_key: &i32,
    to_key: &i32,
    stacks: &mut HashMap<i32, Vec<char>>,
) {
    let from = stacks.get_mut(from_key).unwrap();
    let new_len = from.len() - *amount as usize;
    let mut tail = from.split_off(new_len);

    // comment this line out was the only change necessary from part 1
    // tail.reverse();

    let to = stacks.get_mut(to_key).unwrap();
    to.append(&mut tail);
}

fn main() {
    let file_path = "day5_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut is_instructions = false;
    let mut instructions: Vec<&str> = Vec::new();

    // Create 9 stacks in a HashMap
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
    for number in 1..=9 {
        stacks.insert(number, Vec::new());
    }

    for line in contents.lines() {
        // Do this once, on the empty line
        if line == "" {
            is_instructions = true;

            let stack_numbers = instructions.pop().unwrap();

            for layer in instructions.iter() {
                for (pos, character) in layer.char_indices() {
                    if character.is_uppercase() {
                        let stack_key = stack_numbers
                            .chars()
                            .nth(pos)
                            .unwrap()
                            .to_digit(10)
                            .unwrap();
                        let the_stack = stacks.get_mut(&(stack_key as i32)).unwrap();
                        the_stack.insert(0, character);
                    }
                }
            }
            continue;
        }

        // Build initial stacks
        if !is_instructions {
            instructions.push(line);
        } else {
            // Move blocks around according to instructions
            let mut line_instructions: Vec<i32> = Vec::new();
            for maybe_a_number in line
                .split_whitespace()
                .map(|x| x.parse::<i32>())
                .into_iter()
            {
                match maybe_a_number {
                    // push numbers to line_instructions
                    Ok(a) => {
                        line_instructions.push(a);
                    }
                    // ignore errors from non-numbers
                    _ => {}
                }
            }
            if let [amount, from_key, to_key] = &line_instructions[..] {
                println!("moving {} from {} to {}", amount, from_key, to_key);
                move_amount_from_to(amount, from_key, to_key, &mut stacks);
            }
        }
    }

    for stack_key in 1..=9 {
        let stack = stacks.get_mut(&stack_key).unwrap();
        println!("{} - {:?}", stack_key, stack);
    }
}
