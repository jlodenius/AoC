use std::collections::HashSet;

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
            for [x, y] in rock.iter() {
                match chamber.get(&[*x, *y - 1]) {
                    Some(_) => {
                        stopped = true;
                    }
                    _ => {}
                }
            }

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

fn main() {
    part_one();
}
