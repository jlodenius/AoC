use std::fs;

fn main() {
    let file_path = "day3_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    static STATIC_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut the_value = 0;

    for line in contents.lines() {
        let first: &str = &line[0..line.len() / 2]; // &str = string slice
        let second: &str = &line[line.len() / 2..];

        let mut same_char: Option<char> = None;

        'first_loop: for b_first in first.chars() {
            for b_second in second.chars() {
                if b_first == b_second {
                    same_char = Some(b_first);
                    break 'first_loop;
                }
            }
        }

        let the_char = same_char.expect("should have a value");

        for (i, byte) in STATIC_LOWER.iter().enumerate() {
            println!("{} - {}", i, byte);

            if *byte == the_char {
                the_value += i + 1;
                break;
            }

            let test: char = byte.to_uppercase().next().expect("fail");
            println!("{} uppercase -> {}", byte, test);
            if test == the_char {
                the_value += (i + 1) + 26;
                break;
            }
        }

        println!("the_value {}", the_value);
    }
}
