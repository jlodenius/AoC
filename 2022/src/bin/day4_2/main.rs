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

        // create vector from first
        let first_vec: Vec<i32> = (first_1_int..=first_2_int).collect();

        // check if array contains any value from the second
        let second_vec: Vec<i32> = (second_1_int..=second_2_int).collect();

        if first_vec.iter().any(|&x| second_vec.contains(&x)) {
            result += 1
        }
    }

    println!("result: {}", result);
}
