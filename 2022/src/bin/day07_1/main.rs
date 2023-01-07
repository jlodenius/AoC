use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let file_path = "day7_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut current_path = Vec::new();

    for line in contents.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let command: Vec<_> = line.split_whitespace().collect();

        match command[..] {
            ["$", "cd", ".."] => {
                current_path.pop();
            }
            ["$", "cd", name] => {
                current_path.push(name);
            }
            [size, name] => {
                let size: u32 = size.parse().unwrap();
                println!("{:?} includes {} ({})", current_path, name, size);
                for path_depth in 0..current_path.len() {
                    let path_buf = PathBuf::from_iter(&current_path[..=path_depth]);
                    match path_buf.to_str() {
                        Some(the_path) => {
                            println!("the_path {}", the_path);
                            *sizes.entry(the_path.to_string()).or_insert(0) += size;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    for (key, val) in &sizes {
        println!("{} -> {}", key, val);
    }

    let result: u32 = sizes.into_values().filter(|&size| size <= 100_000).sum();
    println!("Result -> {}", result)
}
