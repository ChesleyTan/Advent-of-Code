use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_set::HashSet;

const DIST_UNREACHABLE: usize = 999999;
const DIST_UNKNOWN: usize = 99999;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("15.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut grid = vec![];
    for line in contents.lines() {
        let mut row = vec![];
        for val in line.chars() {
            row.push(val);
        }
        grid.push(row);
    }
    let mut health_grid = vec![];
    let mut num_goblins = 0;
    let mut num_elves = 0;
    for row in grid.iter() {
        let mut health_row = vec![];
        for &val in row.iter() {
            if val == 'E' {
                num_elves += 1;
                health_row.push(200);
            } else if val == 'G' {
                num_goblins += 1;
                health_row.push(200);
            } else {
                health_row.push(0);
            }
        }
        health_grid.push(health_row);
    }
    let mut step = 0;
    while let Some(new_grid) = simulate(&grid, &mut health_grid, &mut num_elves, &mut num_goblins, 3) {
        grid = new_grid;
        step += 1;
        println!("=== Step {} ===\n", step);
        print_grid(&grid, "");
    }
    let mut total_health = 0;
    for row in health_grid.iter() {
        for &val in row.iter() {
            total_health += val;
        }
    }
    println!("{}", step * total_health);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("15.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut grid = vec![];
    for line in contents.lines() {
        let mut row = vec![];
        for val in line.chars() {
            row.push(val);
        }
        grid.push(row);
    }
    let mut health_grid = vec![];
    let mut num_goblins = 0;
    let mut num_elves = 0;
    for row in grid.iter() {
        let mut health_row = vec![];
        for &val in row.iter() {
            if val == 'E' {
                num_elves += 1;
                health_row.push(200);
            } else if val == 'G' {
                num_goblins += 1;
                health_row.push(200);
            } else {
                health_row.push(0);
            }
        }
        health_grid.push(health_row);
    }
    let mut step = 0;
    let orig_num_goblins = num_goblins;
    let orig_num_elves = num_elves;
    let orig_grid = grid.clone();
    let orig_health_grid = health_grid.clone();
    'outer: for elf_attack_power in 4.. {
        grid = orig_grid.clone();
        health_grid = orig_health_grid.clone();
        num_elves = orig_num_elves;
        num_goblins = orig_num_goblins;
        step = 0;
        while let Some(new_grid) = simulate(&grid, &mut health_grid, &mut num_elves, &mut num_goblins, elf_attack_power) {
            if num_elves != orig_num_elves {
                continue 'outer;
            }
            grid = new_grid;
            step += 1;
            println!("=== Step {} Atk {} ===\n", step, elf_attack_power);
            print_grid(&grid, "");
        }
        println!("{}", elf_attack_power);
        break;
    }
    let mut total_health = 0;
    for row in health_grid.iter() {
        for &val in row.iter() {
            total_health += val;
        }
    }
    println!("{}", step * total_health);
    Ok(())
}

fn simulate(grid: &Vec<Vec<char>>, health_grid: &mut Vec<Vec<isize>>, num_elves: &mut usize, num_goblins: &mut usize, elf_attack_power: isize) -> Option<Vec<Vec<char>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = grid.clone();
    let mut dead_locs = HashSet::new();
    for r in 0..height {
        for c in 0..width {
            let val = grid[r][c];
            let mut current = (r, c);
            // Check unit is not already dead
            if (val == 'E' || val == 'G') && !dead_locs.contains(&current) {
                if *num_elves == 0 || *num_goblins == 0 {
                    return None;
                }
                let enemy = if val == 'E' { 'G' } else { 'E' };
                // Calculate attack squares
                let attack_squares = attack_squares(&new_grid, enemy);
                let mut is_in_range = false;
                for &attack_square in attack_squares.iter() {
                    if attack_square == current {
                        is_in_range = true;
                        break;
                    }
                }
                if !is_in_range {
                    // Calculate square to move to
                    let dists_grid = dists_to(&new_grid, current);
                    let mut closest_attack_square = None;
                    let mut closest_attack_square_dist = DIST_UNKNOWN;
                    for &attack_square in attack_squares.iter() {
                        if new_grid[attack_square.0][attack_square.1] != '.' {
                            continue;
                        }
                        if dists_grid[attack_square.0][attack_square.1] < closest_attack_square_dist {
                            closest_attack_square_dist = dists_grid[attack_square.0][attack_square.1];
                            closest_attack_square = Some(attack_square);
                        }
                    }
                    // Move towards square
                    if closest_attack_square.is_some() {
                        let dists_grid = dists_to(&new_grid, closest_attack_square.unwrap());
                        let mut optimal_move = None;
                        let mut optimal_move_dist = DIST_UNKNOWN;
                        for adj in adj_locs(current, width, height) {
                            if new_grid[adj.0][adj.1] != '.' {
                                continue;
                            }
                            if dists_grid[adj.0][adj.1] < optimal_move_dist {
                                optimal_move_dist = dists_grid[adj.0][adj.1];
                                optimal_move = Some(adj);
                            }
                        }
                        if optimal_move.is_some() {
                            let (new_r, new_c) = optimal_move.unwrap();
                            new_grid[r][c] = '.';
                            new_grid[new_r][new_c] = val;
                            health_grid[new_r][new_c] = health_grid[r][c];
                            health_grid[r][c] = 0;
                            current = (new_r, new_c);
                        }
                    }
                }
                // Attack
                let mut optimal_target = None;
                let mut optimal_target_hp = std::isize::MAX;
                for adj in adj_locs(current, width, height) {
                    if new_grid[adj.0][adj.1] != enemy {
                        continue;
                    }
                    if health_grid[adj.0][adj.1] < optimal_target_hp {
                        optimal_target_hp = health_grid[adj.0][adj.1];
                        optimal_target = Some(adj);
                    }
                }
                if optimal_target.is_some() {
                    let optimal_target = optimal_target.unwrap();
                    if enemy == 'G' {
                        health_grid[optimal_target.0][optimal_target.1] -= elf_attack_power;
                    } else {
                        health_grid[optimal_target.0][optimal_target.1] -= 3;
                    }
                    if health_grid[optimal_target.0][optimal_target.1] <= 0 {
                        health_grid[optimal_target.0][optimal_target.1] = 0;
                        new_grid[optimal_target.0][optimal_target.1] = '.';
                        dead_locs.insert(optimal_target);
                        if enemy == 'E' {
                            *num_elves -= 1;
                        } else if enemy == 'G' {
                            *num_goblins -= 1;
                        }
                    }
                }
            }
        }
    }
    Some(new_grid)
}

fn print_grid_debug<T: std::fmt::Debug>(grid: &Vec<Vec<T>>, separator: &str) {
    for row in grid.iter() {
        for val in row.iter() {
            print!("{:?}{}", val, separator);
        }
        println!();
    }
}

fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>, separator: &str) {
    for row in grid.iter() {
        for val in row.iter() {
            print!("{}{}", val, separator);
        }
        println!();
    }
}

type Point = (usize, usize);

fn adj_locs(target: Point, width: usize, height: usize) -> Vec<Point> {
    let mut adj = vec![];
    let (r, c) = target;
    if r >= 1 {
        adj.push((r - 1, c));
    }
    if c >= 1 {
        adj.push((r, c - 1));
    }
    if c + 1 < width {
        adj.push((r, c + 1));
    }
    if r + 1 < height {
        adj.push((r + 1, c));
    }
    adj
}

fn dists_to(grid: &Vec<Vec<char>>, target: Point) -> Vec<Vec<usize>> {
    let mut dist_grid = vec![];
    for row in grid.iter() {
        let mut row_dists = vec![];
        for &val in row.iter() {
            match val {
                '#' | 'E' | 'G' => row_dists.push(DIST_UNREACHABLE),
                _ => row_dists.push(DIST_UNKNOWN),
            }
        }
        dist_grid.push(row_dists);
    }
    dist_grid[target.0][target.1] = 0;
    let height = grid.len();
    let width = grid[0].len();
    for _ in 0..height {
        for r in 0..height {
            for c in 0..width {
                let current_dist = dist_grid[r][c];
                let adj_locs = adj_locs((r, c), width, height);
                for &adj in adj_locs.iter() {
                    if dist_grid[adj.0][adj.1] == DIST_UNREACHABLE {
                        continue;
                    }
                    dist_grid[adj.0][adj.1] = std::cmp::min(dist_grid[adj.0][adj.1], 1 + current_dist);
                }
            }
        }
    }
    dist_grid
}

fn attack_squares(grid: &Vec<Vec<char>>, target: char) -> Vec<Point> {
    let mut targets = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let val = grid[r][c];
            if val != target {
                continue;
            }
            let adj_locs = adj_locs((r, c), grid[0].len(), grid.len());
            for &adj in adj_locs.iter() {
                if grid[adj.0][adj.1] == '#' {
                    continue;
                }
                targets.push(adj);
            }
        }
    }
    targets
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
