use std::collections::{HashMap, HashSet};

fn part_one() {
    let mut jets = include_str!("input.txt").chars();

    let rocks = vec![
        vec![[2, 0], [3, 0], [4, 0], [5, 0]],
        vec![[2, 1], [3, 1], [3, 2], [3, 0], [4, 1]],
        vec![[2, 0], [3, 0], [4, 0], [4, 1], [4, 2]],
        vec![[2, 0], [2, 1], [2, 2], [2, 3]],
        vec![[2, 0], [3, 0], [2, 1], [3, 1]],
    ];

    let mut peak = 0;
    let mut chamber: HashSet<[u32; 2]> = (0..7).map(|i| [i, 0]).collect();

    for i in 0..2022 {
        // 1. Get a rock from the array
        let mut rock = rocks[i % 5].clone();

        println!("A new rock begins falling");

        // 2. Place it 4 steps above the highest peak
        for [_, y] in rock.iter_mut() {
            *y += peak + 4;
        }

        let mut stopped = false;
        while stopped == false {
            // Move sideways
            let movement = {
                let next_move = match jets.next() {
                    Some('\n') | None => {
                        jets = include_str!("input.txt").chars();
                        jets.next().unwrap()
                    }
                    Some(next_val) => next_val,
                };
                next_move
            };
            match movement {
                '<' => {
                    let mut can_move = true;
                    for [x, y] in rock.iter() {
                        if *x == 0 {
                            can_move = false; // The edge prevents movement
                        } else {
                            match chamber.get(&[*x - 1, *y]) {
                                Some(_) => can_move = false, // Another rock prevents movement
                                _ => {}
                            }
                        }
                    }
                    if can_move {
                        println!("Jet of gas pushes rock left");
                        for [x, _] in rock.iter_mut() {
                            *x -= 1;
                        }
                    } else {
                        println!("Jet of gas pushes rock left but nothing happens");
                    }
                }
                '>' => {
                    let mut can_move = true;
                    for [x, y] in rock.iter() {
                        if *x == 6 {
                            can_move = false; // The edge prevents movement
                        } else {
                            match chamber.get(&[*x + 1, *y]) {
                                Some(_) => can_move = false, // Another rock prevents movement
                                _ => {}
                            }
                        }
                    }
                    if can_move {
                        println!("Jet of gas pushes rock right");
                        for [x, _] in rock.iter_mut() {
                            *x += 1;
                        }
                    } else {
                        println!("Jet of gas pushes rock right but nothing happens");
                    }
                }
                _ => panic!("No jets left"),
            }

            // Check for collision
            stopped = rock
                .iter()
                .any(|[x, y]| chamber.get(&[*x, *y - 1]).is_some());

            // If collision, update peak and chamber
            if stopped {
                for [x, y] in rock.iter() {
                    chamber.insert([*x, *y]);
                }
                peak = std::cmp::max(*rock.iter().map(|[_, y]| y).max().unwrap(), peak);
                println!("Rock falls 1 unit causing it to come to a rest");
                println!("\n\nPeak is now -> {peak}\n\n");
            } else {
                // Move down
                println!("Rock falls 1 unit");
                for [_, y] in rock.iter_mut() {
                    *y -= 1;
                }
            }
        }
    }

    println!("\n\nPeak = {peak}");
}

fn part_two() {
    let mut jets = include_str!("input.txt").chars();
    const MAX_ROCKS: usize = 1000000000000;
    const INPUT_LEN: usize = include_str!("input.txt").len() - 1; // -1 to remove new line char

    let rocks = vec![
        vec![[2, 0], [3, 0], [4, 0], [5, 0]],
        vec![[2, 1], [3, 1], [3, 2], [3, 0], [4, 1]],
        vec![[2, 0], [3, 0], [4, 0], [4, 1], [4, 2]],
        vec![[2, 0], [2, 1], [2, 2], [2, 3]],
        vec![[2, 0], [3, 0], [2, 1], [3, 1]],
    ];

    let mut peak: usize = 0;
    let mut chamber: HashSet<[usize; 2]> = (0..7).map(|i| [i, 0]).collect();
    let mut states: HashMap<(u8, [usize; 7], usize), [usize; 2]> = HashMap::new();
    let mut cycle_found = false;
    let mut peak_map: HashMap<usize, usize> = HashMap::new();

    let mut i = 0;
    while i < MAX_ROCKS {
        // 1. Get a rock from the array
        let mut rock = rocks[i % 5].clone();

        // 2. Place it 4 steps above the highest peak
        for [_, y] in rock.iter_mut() {
            *y += peak + 4;
        }

        let mut stopped = false;
        while stopped == false {
            // Move sideways
            let movement = {
                let next_move = match jets.next() {
                    Some('\n') | None => {
                        jets = include_str!("input.txt").chars();
                        jets.next().unwrap()
                    }
                    Some(next_val) => next_val,
                };
                next_move
            };
            match movement {
                '<' => {
                    let mut can_move = true;
                    for [x, y] in rock.iter() {
                        if *x == 0 {
                            can_move = false; // The edge prevents movement
                        } else {
                            match chamber.get(&[*x - 1, *y]) {
                                Some(_) => can_move = false, // Another rock prevents movement
                                _ => {}
                            }
                        }
                    }
                    if can_move {
                        for [x, _] in rock.iter_mut() {
                            *x -= 1;
                        }
                    }
                }
                '>' => {
                    let mut can_move = true;
                    for [x, y] in rock.iter() {
                        if *x == 6 {
                            can_move = false; // The edge prevents movement
                        } else {
                            match chamber.get(&[*x + 1, *y]) {
                                Some(_) => can_move = false, // Another rock prevents movement
                                _ => {}
                            }
                        }
                    }
                    if can_move {
                        for [x, _] in rock.iter_mut() {
                            *x += 1;
                        }
                    }
                }
                _ => panic!("No jets left"),
            }

            // Check for collision
            stopped = rock
                .iter()
                .any(|[x, y]| chamber.get(&[*x, *y - 1]).is_some());

            // If collision, update peak and chamber
            if stopped {
                for [x, y] in rock.iter() {
                    chamber.insert([*x, *y]);
                }
                let new_peak = *rock.iter().map(|[_, y]| y).max().unwrap();
                let old_peak = peak;
                peak = std::cmp::max(new_peak, old_peak);

                // Find cycles (THIS IS MOST OF THE PART 2 STUFF)
                if !cycle_found {
                    let current_rock = i % 5;
                    let jet_pos = i % INPUT_LEN;
                    let mut state: [usize; 7] = [0; 7];
                    for chamber_i in 0..7 {
                        let max_y = chamber
                            .iter()
                            .filter(|[x, _]| *x == chamber_i)
                            .map(|[_, y]| y)
                            .max()
                            .unwrap();

                        let value = old_peak + 4 - *max_y;
                        state[chamber_i as usize] = value;
                    }
                    match states.get(&(current_rock as u8, state, jet_pos)) {
                        Some([prev_idx, prev_peak]) => {
                            // Found prev state like this
                            cycle_found = true;
                            println!("Found cycle from index {prev_idx} to index {i}");
                            let cycle_length = i - prev_idx;
                            let cycle_height = peak - prev_peak;
                            println!("Cycle length {cycle_length}\nCycle height {cycle_height}");
                            let rocks_left = MAX_ROCKS - i;
                            println!("Rocks left {rocks_left}");
                            let cycles_to_jump = rocks_left / cycle_length;
                            println!("Cycles to jump {cycles_to_jump}");
                            let cycles_left = rocks_left % cycle_length;
                            println!("Cycles left {cycles_left}");

                            let height_after_cycles_left =
                                peak_map.get(&(prev_idx + (cycles_left - 1))).unwrap(); // Not sure
                                                                                        // why -1 is necessary here
                                                                                        // but it works for sample input
                            println!(
                                "Height between idx {prev_idx} and {} = {height_after_cycles_left}",
                                cycles_left - 1
                            );

                            // End the while, everything is done here
                            i = MAX_ROCKS;
                            peak += cycles_to_jump * cycle_height;
                            peak += height_after_cycles_left - prev_peak;
                            continue;
                        }
                        _ => {
                            states.insert((current_rock as u8, state, jet_pos), [i, peak]);
                            // This makes things much slower than necessary
                            // but cba to refactor atm
                            peak_map.insert(i, peak);
                        }
                    }
                }
            } else {
                // Move down
                for [_, y] in rock.iter_mut() {
                    *y -= 1;
                }
            }
        }
        i += 1;
    }

    println!("\n\nPeak = {peak}");
}

fn main() {
    // println!("--- PART ONE ---");
    // part_one();
    println!("--- PART TWO ---");
    part_two();
}
