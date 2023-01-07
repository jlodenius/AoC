use std::fs;

fn main() {
    let file_path = "day3_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();

    static STATIC_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut the_value = 0;

    // read 3 lines at a time, could be done (cleaner?) with itertools
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        println!("{} - {} - {}", first, second, third);

        let mut same_char: Option<char> = None;

        'first_loop: for b_first in first.chars() {
            for b_second in second.chars() {
                if b_first == b_second {
                    for b_third in third.chars() {
                        if b_third == b_second {
                            same_char = Some(b_third);
                            break 'first_loop;
                        }
                    }
                }
            }
        }

        let the_char = same_char.expect("should have a value");

        for (i, byte) in STATIC_LOWER.iter().enumerate() {
            if *byte == the_char {
                the_value += i + 1;
                break;
            }

            // to_uppercase returns an iterator, we extract the first value from it
            // which will always be our character (since we only use [a-zA-Z])
            let upper_case: char = byte.to_uppercase().next().expect("fail");
            if upper_case == the_char {
                the_value += (i + 1) + 26;
                break;
            }
        }

        println!("{}", the_value);
    }
}
