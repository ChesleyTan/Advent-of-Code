use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("6.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let coords: Vec<_> = contents.lines().map(parse_coordinate).collect();
    let mut min_x = 999;
    let mut min_y = 999;
    let mut max_x = 0;
    let mut max_y = 0;
    for coord in coords.iter() {
        min_x = std::cmp::min(min_x, coord.x);
        min_y = std::cmp::min(min_y, coord.y);
        max_x = std::cmp::max(max_x, coord.x);
        max_y = std::cmp::max(max_y, coord.y);
    }
    let mut grid: Vec<Vec<isize>> = vec![];
    for r in min_y..max_y + 1 {
        grid.push(vec![]);
        for c in min_x..max_x + 1 {
            let target = Coordinate { x: c, y: r };
            let closest_coord = closest_coordinate(&coords, &target);
            if let Some(n) = closest_coord {
                grid[(r - min_y) as usize].push(n);
            } else {
                grid[(r - min_y) as usize].push(0);
            }
        }
    }
    let mut unique_values = HashSet::new();
    for row in grid.iter() {
        for &val in row {
            unique_values.insert(val);
        }
    }
    let mut border_values = HashSet::new();
    for &val in grid[0].iter() {
        border_values.insert(val);
    }
    for &val in grid[grid.len() - 1].iter() {
        border_values.insert(val);
    }
    for row in grid.iter() {
        border_values.insert(row[0]);
        border_values.insert(row[row.len() - 1]);
    }
    let interior_values: HashSet<_> = unique_values.difference(&border_values).collect();
    let mut counts = HashMap::new();
    for &&val in interior_values.iter() {
        if val != 0 {
            counts.insert(val, 0);
        }
    }
    let mut max_count = 0;
    for row in grid.iter() {
        for val in row {
            match counts.get_mut(val) {
                None => (),
                Some(n) => {
                    *n += 1;
                    max_count = std::cmp::max(max_count, *n);
                }
            }
        }
    }
    println!("{:?}", max_count);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("6.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let coords: Vec<_> = contents.lines().map(parse_coordinate).collect();
    let mut min_x = 999;
    let mut min_y = 999;
    let mut max_x = 0;
    let mut max_y = 0;
    for coord in coords.iter() {
        min_x = std::cmp::min(min_x, coord.x);
        min_y = std::cmp::min(min_y, coord.y);
        max_x = std::cmp::max(max_x, coord.x);
        max_y = std::cmp::max(max_y, coord.y);
    }
    let mut grid: Vec<Vec<isize>> = vec![];
    let mut num_valid = 0;
    for r in min_y..max_y + 1 {
        grid.push(vec![]);
        for c in min_x..max_x + 1 {
            let target = Coordinate { x: c, y: r };
            let total_dist = total_dist(&coords, &target);
            if total_dist < 10000 {
                num_valid += 1;
            }
        }
    }
    println!("{}", num_valid);
    Ok(())
}

#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

fn parse_coordinate(line: &str) -> Coordinate {
    let tokens: Vec<_> = line.split(", ").collect();
    Coordinate {
        x: parse_isize(tokens[0]),
        y: parse_isize(tokens[1]),
    }
}

fn parse_isize(n: &str) -> isize {
    n.parse::<isize>().unwrap()
}

fn closest_coordinate(coords: &Vec<Coordinate>, target: &Coordinate) -> Option<isize> {
    let mut closest = None;
    let mut closest_dist = 9999;
    for (idx, coord) in coords.iter().enumerate() {
        let dist = (coord.x - target.x).abs() + (coord.y - target.y).abs();
        if dist < closest_dist {
            closest_dist = dist;
            closest = Some((idx + 1) as isize);
        } else if dist == closest_dist {
            closest = None;
        }
    }
    closest
}

fn total_dist(coords: &Vec<Coordinate>, target: &Coordinate) -> usize {
    let mut total = 0;
    for coord in coords.iter() {
        total += (coord.x - target.x).abs() + (coord.y - target.y).abs();
    }
    total as usize
}

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
