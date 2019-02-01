use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("20.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let regex = contents.lines().next().unwrap();
    // Strip ^ and $
    let regex = &regex[1..regex.len()];
    let mut grid = HashMap::new();
    let mut room_distances = HashMap::new();
    let mut positions = HashSet::new();
    grid.insert((0, 0), '.');
    room_distances.insert((0, 0), 0);
    positions.insert((0, 0));
    explore_grid(&mut grid, &mut room_distances, regex, &positions);
    println!("{:?}", room_distances);
    print_grid(&grid);
    println!("{}", room_distances.values().max().unwrap());
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("20.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let regex = contents.lines().next().unwrap();
    // Strip ^ and $
    let regex = &regex[1..regex.len()];
    let mut grid = HashMap::new();
    let mut room_distances = HashMap::new();
    let mut positions = HashSet::new();
    grid.insert((0, 0), '.');
    room_distances.insert((0, 0), 0);
    positions.insert((0, 0));
    explore_grid(&mut grid, &mut room_distances, regex, &positions);
    let mut num_at_least_1000 = 0;
    for &val in room_distances.values() {
        if val >= 1000 {
            num_at_least_1000 += 1;
        }
    }
    println!("{}", num_at_least_1000);
    f
    Ok(())
}

fn print_grid(grid: &HashMap<Coord, char>) {
    let mut min_x = std::isize::MAX;
    let mut max_x = std::isize::MIN;
    let mut min_y = std::isize::MAX;
    let mut max_y = std::isize::MIN;
    for &(x, y) in grid.keys() {
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
    }
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("X");
            } else {
                print!("{}", grid.get(&(x, y)).unwrap_or(&'#'));
            }
        }
        println!();
    }
}

fn explore_grid(
    grid: &mut HashMap<Coord, char>,
    room_distances: &mut HashMap<Coord, usize>,
    regex: &str,
    current_positions: &HashSet<Coord>,
) -> HashSet<Coord> {
    if regex.len() == 0 {
        return HashSet::clone(current_positions);
    }
    let mut new_positions = HashSet::new();
    let mut remainder = &regex[1..];
    match regex.chars().next().unwrap() {
        'N' => {
            for current_pos in current_positions.iter() {
                let above = (current_pos.0, current_pos.1 + 1);
                let above_left = (current_pos.0 - 1, current_pos.1 + 1);
                let above_right = (current_pos.0 + 1, current_pos.1 + 1);
                let next_room = (current_pos.0, current_pos.1 + 2);
                grid.insert(above, '-');
                grid.insert(above_left, '#');
                grid.insert(above_right, '#');
                grid.insert(next_room, '.');
                let dist_to_current_room = *room_distances.get(&current_pos).unwrap();
                let dist = std::cmp::min(
                    *room_distances.get(&next_room).unwrap_or(&std::usize::MAX),
                    dist_to_current_room + 1,
                );
                room_distances.insert(next_room, dist);
                new_positions.insert(next_room);
            }
        }
        'E' => {
            for current_pos in current_positions.iter() {
                let right = (current_pos.0 + 1, current_pos.1);
                let right_up = (current_pos.0 + 1, current_pos.1 + 1);
                let right_down = (current_pos.0 + 1, current_pos.1 - 1);
                let next_room = (current_pos.0 + 2, current_pos.1);
                grid.insert(right, '|');
                grid.insert(right_up, '#');
                grid.insert(right_down, '#');
                grid.insert(next_room, '.');
                let dist_to_current_room = *room_distances.get(&current_pos).unwrap();
                let dist = std::cmp::min(
                    *room_distances.get(&next_room).unwrap_or(&std::usize::MAX),
                    dist_to_current_room + 1,
                );
                room_distances.insert(next_room, dist);
                new_positions.insert(next_room);
            }
        }
        'W' => {
            for current_pos in current_positions.iter() {
                let left = (current_pos.0 - 1, current_pos.1);
                let left_up = (current_pos.0 - 1, current_pos.1 + 1);
                let left_down = (current_pos.0 - 1, current_pos.1 - 1);
                let next_room = (current_pos.0 - 2, current_pos.1);
                grid.insert(left, '|');
                grid.insert(left_up, '#');
                grid.insert(left_down, '#');
                grid.insert(next_room, '.');
                let dist_to_current_room = *room_distances.get(&current_pos).unwrap();
                let dist = std::cmp::min(
                    *room_distances.get(&next_room).unwrap_or(&std::usize::MAX),
                    dist_to_current_room + 1,
                );
                room_distances.insert(next_room, dist);
                new_positions.insert(next_room);
            }
        }
        'S' => {
            for current_pos in current_positions.iter() {
                let down = (current_pos.0, current_pos.1 - 1);
                let down_left = (current_pos.0 - 1, current_pos.1 - 1);
                let down_right = (current_pos.0 + 1, current_pos.1 - 1);
                let next_room = (current_pos.0, current_pos.1 - 2);
                grid.insert(down, '-');
                grid.insert(down_left, '#');
                grid.insert(down_right, '#');
                grid.insert(next_room, '.');
                let dist_to_current_room = *room_distances.get(&current_pos).unwrap();
                let dist = std::cmp::min(
                    *room_distances.get(&next_room).unwrap_or(&std::usize::MAX),
                    dist_to_current_room + 1,
                );
                room_distances.insert(next_room, dist);
                new_positions.insert(next_room);
            }
        }
        '(' => {
            let (branches, rest) = parse_branch(&regex[1..]);
            remainder = rest;
            for &branch in branches.iter() {
                let positions = explore_grid(grid, room_distances, branch, &current_positions);
                new_positions.extend(positions);
            }
        }
        _ => (),
    }
    explore_grid(grid, room_distances, remainder, &new_positions)
}

fn parse_branch(regex: &str) -> (Vec<&str>, &str) {
    let mut branches = vec![];
    let mut nesting_level = 1;
    let mut branch_begin = 0;
    for (idx, c) in regex.chars().enumerate() {
        match c {
            '(' => {
                nesting_level += 1;
            }
            ')' => {
                nesting_level -= 1;
                if nesting_level == 0 {
                    branches.push(&regex[branch_begin..idx]);
                    return (branches, &regex[idx + 1..]);
                }
            }
            '|' if nesting_level == 1 => {
                branches.push(&regex[branch_begin..idx]);
                branch_begin = idx + 1;
            }
            _ => (),
        }
    }
    panic!("Did not find end of branch!");
}

type Coord = (isize, isize);

fn main() -> Result<(), Box<dyn Error>> {
    let part = env::args()
        .nth(1)
        .expect("Expected argument specifying problem part `a` or `b`");
    if part == "a" {
        a()
    } else {
        b()
    }
}
