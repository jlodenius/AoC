use std::fs;

fn main() {
    let file_path = "day4_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut result = 0;

    for line in contents.lines() {
        let (first, second) = line.split_once(",").expect("Fail");

        let (first_1, first_2) = first.split_once("-").expect("Fail");
        let (second_1, second_2) = second.split_once("-").expect("Fail");

        let first_1_int: i32 = first_1.parse().unwrap();
        let first_2_int: i32 = first_2.parse().unwrap();

        let second_1_int: i32 = second_1.parse().unwrap();
        let second_2_int: i32 = second_2.parse().unwrap();

        let mut line_value = 0;

        if first_1_int >= second_1_int && first_2_int <= second_2_int {
            line_value = 1;
        } else if second_1_int >= first_1_int && second_2_int <= first_2_int {
            line_value = 1;
        }

        result += line_value;
    }

    println!("result: {}", result);
}
