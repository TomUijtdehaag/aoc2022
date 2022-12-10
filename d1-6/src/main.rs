use std::collections::{HashMap, HashSet};

use std::fs;
use utils::read_input;

mod utils;

fn main() {
    d6();
}

fn d6() {
    let buffer = fs::read_to_string("input/6.txt".to_string())
        .expect("Unable to read file.")
        .chars()
        .collect::<Vec<char>>();

    let marker_size = 14;

    let mut start_of_packet = 0;

    for (i, window) in buffer.windows(marker_size).enumerate() {
        let w: HashSet<&char> = HashSet::from_iter(window.clone().iter());

        if w.len() == marker_size {
            start_of_packet = i + marker_size;
            break;
        }
    }

    println!("start of packet: {}", start_of_packet)
}

fn d5() {
    //     [M]             [Z]     [V]
    //     [Z]     [P]     [L]     [Z] [J]
    // [S] [D]     [W]     [W]     [H] [Q]
    // [P] [V] [N] [D]     [P]     [C] [V]
    // [H] [B] [J] [V] [B] [M]     [N] [P]
    // [V] [F] [L] [Z] [C] [S] [P] [S] [G]
    // [F] [J] [M] [G] [R] [R] [H] [R] [L]
    // [G] [G] [G] [N] [V] [V] [T] [Q] [F]
    //  1   2   3   4   5   6   7   8   9

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let stack_contents = vec![
        "GFVHPS", "GJFBVDZM", "GMLJN", "NGZVDWP", "VRCB", "VRSMPWLZ", "THP", "QRSNCHZV", "FLGPVQJ",
    ];

    for stack in stack_contents.iter() {
        stacks.push(stack.chars().collect::<Vec<char>>());
    }

    let moves = utils::read_input("input/5.txt".to_string());

    for move_line in moves.iter() {
        let mut mv = move_line.split(" ");

        mv.next();
        let amount: usize = mv.next().unwrap().parse().unwrap();
        mv.next();
        let from: usize = mv.next().unwrap().parse().unwrap();
        mv.next();
        let to: usize = mv.next().unwrap().parse().unwrap();

        let mut transfer_stack = Vec::new();

        for _ in 0..amount {
            let item = stacks[from - 1].pop().unwrap();
            transfer_stack.push(item);
        }
        for _ in 0..amount {
            let item = transfer_stack.pop().unwrap();
            stacks[to - 1].push(item);
        }
    }

    let mut top_items = "".to_string();

    for stack in stacks.iter_mut() {
        top_items.push(stack.pop().unwrap());
    }

    println!("Top items: {top_items}");
}

fn d4() {
    let pairs = utils::read_input("input/4.txt".to_string());

    let mut fully_contained = 0;
    let mut overlapping = 0;

    for pair in pairs.iter() {
        let ab: Vec<&str> = pair.split(",").collect();

        let a: Vec<u16> = ab[0]
            .split("-")
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let b: Vec<u16> = ab[1]
            .split("-")
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        if (a[0] <= b[0] && a[1] >= b[1]) || (a[0] >= b[0] && a[1] <= b[1]) {
            fully_contained += 1;
        }

        if (a[0] >= b[0] && a[0] <= b[1])
            || (a[1] >= b[0] && a[1] <= b[1])
            || (b[0] >= a[0] && b[0] <= a[1])
            || (b[1] >= a[0] && b[1] <= a[1])
        {
            overlapping += 1;
        }
    }
    println!("Pairs fully contained: {}", fully_contained);
    println!("Pairs overlapping: {}", overlapping);
}

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
