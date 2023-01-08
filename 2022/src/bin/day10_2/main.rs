use std::fs;

fn main() {
    let file_path = "day10_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut x_value = 1;
    let mut x_register: Vec<i32> = vec![];

    // keep track of when we added because when we are adding,
    // we start and stop at the same cycle, effectively saving one cycle
    let mut prev_add = false;

    for line in contents.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [_add, amount] => {
                if !prev_add {
                    x_register.push(x_value);
                }
                x_register.push(x_value);
                x_value += amount.parse::<i32>().unwrap();
                x_register.push(x_value);
                prev_add = true;
            }
            [_noop] => {
                x_register.push(x_value);
            }
            _ => panic!("fail"),
        }
    }

    // Part 1, improved because previous solution sucked
    let result: i32 = [20, 60, 100, 140, 180, 220]
        .map(|cycle| x_register[cycle - 1] * cycle as i32)
        .iter()
        .sum();

    println!("{:?}", result);

    // Part 2
    for row in x_register.chunks(40) {
        let mut row_string = String::new();

        for (position, x_value) in row.iter().enumerate() {
            if position as i32 >= x_value - 1 && position as i32 <= x_value + 1 {
                row_string.push_str("#");
            } else {
                row_string.push_str(".");
            }
        }
        println!("{}", row_string);
    }
}
