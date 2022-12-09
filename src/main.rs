use std::collections::{HashMap, HashSet};

mod utils;

fn main() {
    d3();
}

fn d4() {}

fn d3() {
    let rucksacks = utils::read_input("input/3.txt".to_string());

    let mut values = HashMap::new();

    for (i, c) in ('a'..='z').enumerate() {
        values.insert(c, (i + 1) as u128);
    }

    for (i, c) in ('A'..='Z').enumerate() {
        values.insert(c, (i + 27) as u128);
    }

    // part 1
    let mut total_priority = 0u128;

    for rucksack in rucksacks.iter() {
        let size = rucksack.len();

        let (a, b) = (
            HashSet::<char>::from_iter(rucksack[0..(size / 2)].chars()),
            HashSet::<char>::from_iter(rucksack[(size / 2)..size].chars()),
        );

        let common = a.intersection(&b);

        for &c in common.into_iter() {
            total_priority += values[&c];
        }
    }

    println!("Total priority: {}", total_priority);

    // part 2
    let mut total_group_priority = 0;

    for group in rucksacks.chunks(3) {
        for (c, v) in values.iter() {
            let mut n = 0;
            for sack in group {
                if sack.contains(*c) {
                    n += 1
                }

                if n == 3 {
                    total_group_priority += v;
                    break;
                }
            }
        }
    }
    println!("Total group priority: {}", total_group_priority)
}

fn d2() {
    let strategy = utils::read_input("input/2.txt".to_string());

    let mut shape_points: HashMap<String, u8> = HashMap::new();
    let mut outcome_points: HashMap<String, u8> = HashMap::new();

    let opponent_shapes = ["A", "B", "C"];
    let my_shapes = ["X", "Y", "Z"];

    for (i, &my_shape) in my_shapes.iter().enumerate() {
        shape_points.insert(my_shape.to_string(), (i + 1) as u8);

        for (j, &opponent_shape) in opponent_shapes.iter().enumerate() {
            let outcome = (i as i8 - j as i8 + 1).rem_euclid(3) * 3;
            let key = [opponent_shape, my_shape].join(" ");

            outcome_points.insert(key, outcome as u8);
        }
    }

    let mut total_points = 0;
    for line in strategy.iter() {
        total_points += shape_points[&line.chars().nth(2).unwrap().to_string()] as u64;
        total_points += outcome_points[line] as u64;
    }

    println!("{}", total_points);

    let mut total_points_part_2 = 0;
    for line in strategy.iter() {
        let outcome = &line.chars().nth(2).unwrap().to_string();
        let opponent_shape = &line.chars().nth(0).unwrap().to_string();

        let offset = match outcome.as_str() {
            "X" => -1i8,
            "Y" => 0i8,
            "Z" => 1i8,
            _ => -128,
        };

        let opponent_shape_idx = opponent_shapes
            .iter()
            .position(|&x| x == opponent_shape.as_str())
            .unwrap() as i8;
        let my_shape_idx = (opponent_shape_idx + offset).rem_euclid(3);
        let my_shape = my_shapes[my_shape_idx as usize];

        total_points_part_2 += shape_points[my_shape] as u64;

        let new_match = [opponent_shape, my_shape].join(" ");
        total_points_part_2 += outcome_points[&new_match] as u64;
    }

    println!("{}", total_points_part_2);
}

fn d1() {
    let lines = utils::read_input("input/1.txt".to_string());
    let mut elves: Vec<i32> = Vec::new();
    let mut calories: i32 = 0;

    for line in lines.iter() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>().unwrap();
        }
    }

    // part 1
    let biggest_elve = elves.iter().max().unwrap();
    println!("The biggest elve has: {}", biggest_elve);

    // part 2
    elves.sort();
    let top3_elves: i32 = elves[elves.len() - 3..elves.len()].iter().sum();
    println!("The top 3 elves have: {}", top3_elves);
}
