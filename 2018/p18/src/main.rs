use std::collections::hash_map::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("18.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut grid = vec![];
    for line in contents.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    for _ in 0..10 {
        grid = simulate(&grid);
    }
    let mut num_wooded = 0;
    let mut num_lumberyard = 0;
    for row in grid.iter() {
        for &val in row.iter() {
            if val == '#' {
                num_lumberyard += 1;
            } else if val == '|' {
                num_wooded += 1;
            }
            print!("{}", val);
        }
        println!();
    }
    println!("{}", num_lumberyard * num_wooded);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("18.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut grid = vec![];
    for line in contents.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    let mut history = HashMap::new();
    history.insert(compress_grid(&grid), 0);
    let mut timestep = 1;
    while timestep <= 1_000_000_000 {
        println!("Step {}", timestep);
        grid = simulate(&grid);
        let compressed = compress_grid(&grid);
        if history.contains_key(&compressed) {
            let cycle_len = timestep - history[&compressed];
            let remaining = 1_000_000_000 - timestep;
            history.insert(compressed, timestep);
            timestep = 1_000_000_000 - (remaining % cycle_len) + 1;
        } else {
            history.insert(compressed, timestep);
            timestep += 1;
        }
    }
    let mut num_wooded = 0;
    let mut num_lumberyard = 0;
    for row in grid.iter() {
        for &val in row.iter() {
            if val == '#' {
                num_lumberyard += 1;
            } else if val == '|' {
                num_wooded += 1;
            }
            print!("{}", val);
        }
        println!();
    }
    println!("{}", num_lumberyard * num_wooded);
    Ok(())
}

fn compress_grid(grid: &Vec<Vec<char>>) -> String {
    let mut s = String::new();
    for row in grid.iter() {
        for &val in row.iter() {
            s.push(val);
        }
    }
    s
}

fn simulate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![];
    for (r, row) in grid.iter().enumerate() {
        let mut new_row = vec![];
        for (c, &val) in row.iter().enumerate() {
            let neighbors = neighboring_acres(&grid, r, c);
            match val {
                '.' => {
                    if neighbors.iter().cloned().filter(|&n| n == '|').count() >= 3 {
                        new_row.push('|');
                    } else {
                        new_row.push('.');
                    }
                }
                '|' => {
                    if neighbors.iter().cloned().filter(|&n| n == '#').count() >= 3 {
                        new_row.push('#');
                    } else {
                        new_row.push('|');
                    }
                }
                '#' => {
                    if neighbors.iter().cloned().filter(|&n| n == '#').count() >= 1
                        && neighbors.iter().cloned().filter(|&n| n == '|').count() >= 1
                    {
                        new_row.push('#');
                    } else {
                        new_row.push('.');
                    }
                }
                _ => (),
            }
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn neighboring_acres(grid: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<char> {
    let mut neighbors = vec![];
    let top = r > 0;
    let bottom = r < grid.len() - 1;
    let left = c > 0;
    let right = c < grid[0].len() - 1;
    if top {
        neighbors.push(grid[r - 1][c]);
        if left {
            neighbors.push(grid[r - 1][c - 1]);
        }
        if right {
            neighbors.push(grid[r - 1][c + 1]);
        }
    }
    if left {
        neighbors.push(grid[r][c - 1]);
    }
    if right {
        neighbors.push(grid[r][c + 1]);
    }
    if bottom {
        neighbors.push(grid[r + 1][c]);
        if left {
            neighbors.push(grid[r + 1][c - 1]);
        }
        if right {
            neighbors.push(grid[r + 1][c + 1]);
        }
    }
    neighbors
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
