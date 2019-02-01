use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_set::HashSet;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("17.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut clay_locations = vec![];
    let mut min_x = std::usize::MAX / 2;
    let mut min_y = std::usize::MAX / 2;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in contents.lines() {
        let mut tokens = line.split(", ");
        let mut x = tokens.next().unwrap();
        let mut y = tokens.next().unwrap();
        if !x.contains("x=") {
            let tmp = x;
            x = y;
            y = tmp;
        }
        let (x_start, x_end) = parse_range(x);
        let (y_start, y_end) = parse_range(y);
        clay_locations.push((x_start, x_end, y_start, y_end));
        min_x = std::cmp::min(min_x, x_start);
        min_y = std::cmp::min(min_y, y_start);
        max_x = std::cmp::max(max_x, x_end);
        max_y = std::cmp::max(max_y, y_end);
    }
    println!("{}", min_x);
    println!("{}", min_y);
    println!("{}", max_x);
    println!("{}", max_y);
    min_x -= 1;
    max_x += 1;
    let mut grid: Vec<Vec<BlockType>> = vec![vec![BlockType::Sand; max_x - min_x + 1]; max_y - min_y + 1];
    for &(x_start, x_end, y_start, y_end) in clay_locations.iter() {
        for r in y_start..=y_end {
            for c in x_start..=x_end {
                grid[r - min_y][c - min_x] = BlockType::Clay;
            }
        }
    }
    let mut num_stillwater = 0;
    let mut frontier: HashSet<(usize, usize)> = HashSet::new();
    grid[0][500 - min_x] = BlockType::RunningWater { clay_left: false, clay_right: false };
    frontier.insert((500 - min_x, 0));
    let mut changed = true;
    let mut stillwater = HashSet::new();
    // Note: Can probably do this more efficiently by iterating down the grid one row at a time and
    // iteratively expanding still water/running water across the row, but local decision processes
    // are fun ¯\_(ツ)_/¯
    while changed {
        changed = false;
        println!("{}", num_stillwater);
        println!("{}", frontier.len());
        let running_water: Vec<(usize, usize)> = frontier.iter().cloned().collect();
        for &(x, y) in running_water.iter() {
            let mut has_clay_left = false;
            let mut has_clay_right = false;
            if let BlockType::RunningWater { clay_left, clay_right } = grid[y][x] {
                has_clay_left = clay_left;
                has_clay_right = clay_right;
            } else {
                panic!("Expected only running water in frontier");
            }
            let left = if x == 0 { None } else { Some(grid[y][x - 1]) };
            let right = if x == grid[0].len() - 1 { None } else { Some(grid[y][x + 1]) };
            let below = if y == grid.len() - 1 { None } else { Some(grid[y + 1][x]) };
            if below == Some(BlockType::Clay) || below == Some(BlockType::StillWater) {
                // Propagate clay_left and clay_right flags of adjacent running water
                if let Some(BlockType::Clay) = left {
                    has_clay_left = true;
                }
                if let Some(BlockType::Clay) = right {
                    has_clay_right = true;
                }
                // Spread to left and right if necessary
                if let Some(BlockType::RunningWater { clay_left: l_clay_left, clay_right: _l_clay_right }) = left {
                    has_clay_left = has_clay_left || l_clay_left;
                } else if left == Some(BlockType::Sand) {
                    changed = true;
                    grid[y][x - 1] = BlockType::RunningWater { clay_left: false, clay_right: false };
                    frontier.insert((x - 1, y));
                }
                if let Some(BlockType::RunningWater { clay_left: _r_clay_left, clay_right: r_clay_right }) = right {
                    has_clay_right = has_clay_right || r_clay_right;
                } else if right == Some(BlockType::Sand) {
                    changed = true;
                    grid[y][x + 1] = BlockType::RunningWater { clay_left: false, clay_right: false };
                    frontier.insert((x + 1, y));
                }
            // Move down if necessary
            } else if below == Some(BlockType::Sand) {
                changed = true;
                grid[y + 1][x] = BlockType::RunningWater { clay_left: false, clay_right: false };
                frontier.insert((x, y + 1));
            }
            // Handle converting running water to still water
            if left == Some(BlockType::StillWater) || right == Some(BlockType::StillWater) || (has_clay_left && has_clay_right) {
                changed = true;
                frontier.remove(&(x, y));
                grid[y][x] = BlockType::StillWater;
                num_stillwater += 1;
                stillwater.insert((x, y));
            } else {
                let new_block = BlockType::RunningWater { clay_left: has_clay_left, clay_right: has_clay_right };
                if grid[y][x] != new_block {
                    grid[y][x] = new_block;
                    changed = true;
                }
            }
        }
    }
    println!("a: {}", num_stillwater + frontier.len());
    println!("b: {}", num_stillwater);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    a()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BlockType {
    StillWater,
    RunningWater {
        clay_left: bool,
        clay_right: bool,
    },
    Clay,
    Sand,
}

fn parse_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn parse_range(s: &str) -> (usize, usize) {
    if s.contains("..") {
        let mut tokens = s[2..].split("..");
        let x = parse_usize(tokens.next().unwrap());
        let y = parse_usize(tokens.next().unwrap());
        (x, y)
    } else {
        let coord = parse_usize(&s[2..]);
        (coord, coord)
    }
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
