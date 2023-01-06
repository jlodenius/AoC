use std::fs;

fn main() {
    let file_path = "day2_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let enemy_input = ["A", "B", "C"];
    let my_input = ["X", "Y", "Z"];

    let mut total_points = 0;
    for line in contents.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let enemy_choice = split[0];
        let my_choice = split[1];

        let enemy_choice_index = enemy_input
            .iter()
            .position(|&x| x == enemy_choice)
            .expect("Should work");

        let my_choice_index = my_input
            .iter()
            .position(|&x| x == my_choice)
            .expect("Should work");

        let mut _points_from_round = 0;
        // if draw
        if enemy_choice_index == my_choice_index {
            _points_from_round = 3;
        } else {
            _points_from_round = match my_choice_index {
                // I chose rock
                0 => {
                    if enemy_choice_index == 2 {
                        6
                    } else {
                        0
                    }
                }
                // I chose anything else
                _ => {
                    if enemy_choice_index + 1 == my_choice_index {
                        6
                    } else {
                        0
                    }
                }
            }
        }

        total_points += my_choice_index + 1;
        total_points += _points_from_round;
    }

    println!("Total points: {}", total_points);
}
