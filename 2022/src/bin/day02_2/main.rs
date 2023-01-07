use std::fs;

fn calc_points(enemy_choice: usize, required_outcome: usize) -> i32 {
    match enemy_choice {
        0 => {
            return match required_outcome {
                0 => 3,
                1 => 1,
                2 => 2,
                _ => panic!("Fail"),
            };
        }
        1 => {
            return match required_outcome {
                0 => 1,
                1 => 2,
                2 => 3,
                _ => panic!("Fail"),
            };
        }
        2 => {
            return match required_outcome {
                0 => 2,
                1 => 3,
                2 => 1,
                _ => panic!("Fail"),
            };
        }
        _ => panic!("Fail"),
    };
}

fn main() {
    let file_path = "day2_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let enemy_input = ["A", "B", "C"];
    let required_outcomes = ["X", "Y", "Z"];

    let mut total_points = 0;

    for line in contents.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let enemy_choice = split[0];
        let required_outcome = split[1];

        let enemy_choice_index = enemy_input
            .iter()
            .position(|&x| x == enemy_choice)
            .expect("Should work");

        let outcome_index = required_outcomes
            .iter()
            .position(|&x| x == required_outcome)
            .expect("Should work");

        let mut _points_from_round = 0;

        _points_from_round += match outcome_index {
            0 => 0,
            1 => 3,
            2 => 6,
            _ => panic!("Should never happen"),
        };
        _points_from_round += calc_points(enemy_choice_index, outcome_index);

        total_points += _points_from_round;
    }

    println!("Total points: {}", total_points);
}
