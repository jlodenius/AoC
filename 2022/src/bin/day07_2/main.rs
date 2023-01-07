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

    let sizes_copy = sizes.clone();
    let mut all_sizes: Vec<u32> = sizes.into_values().collect();
    all_sizes.sort();

    let used_space = all_sizes.iter().rev().next().unwrap();
    let total_space = 70_000_000;
    let required_free_space = 30_000_000;
    let space_to_free = required_free_space - (total_space - used_space);

    println!(
        "Total space {}\nRequired space {}\nUsed space {}\nSpace to free {}",
        total_space, required_free_space, used_space, space_to_free,
    );

    let folder_to_delete = all_sizes
        .iter()
        .find(|&size| size >= &space_to_free)
        .unwrap();

    for (key, value) in &sizes_copy {
        if value == folder_to_delete {
            println!("delete folder {}, with size {}", key, value);
        }
    }
}
