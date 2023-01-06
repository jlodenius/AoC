use std::fs;

fn main() {
    let file_path = "day7_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    println!("{}", contents);
}
