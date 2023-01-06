use std::fs;

fn main() {
    let file_path = "day1_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut results: Vec<i32> = Vec::new();
    let mut sum = 0;

    for line in contents.lines() {
        if line.is_empty() {
            results.push(sum);
            sum = 0;
            println!("\n");
        } else {
            println!("{line}");
            sum += line.parse::<i32>().unwrap();
        }
    }

    results.sort();
    results.reverse();
    let top3 = results[0..3].to_vec();

    let mut top3_combined = 0;
    for val in &top3 {
        top3_combined += val;
    }

    println!("{:?}", top3);
    println!("{top3_combined}");
}
