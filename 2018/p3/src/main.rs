use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_set::HashSet;
use std::cell::RefCell;

const GRID_SIZE: usize = 1000;
const CONFLICT: usize = 99999;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("3.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut grid: [[usize; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    map_grid(&mut grid, contents.as_str(), |grid, r, c, _| {
        grid[r][c] += 1;
    });
    let mut num_overlaps = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if grid[r][c] > 1 {
                num_overlaps += 1;
            }
        }
    }
    println!("{}", num_overlaps);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("3.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut grid: [[usize; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    map_grid(&mut grid, contents.as_str(), |grid, r, c, claim| {
        if grid[r][c] != 0 {
            grid[r][c] = CONFLICT;
        } else {
            grid[r][c] = claim;
        }
    });
    let claims = RefCell::new(HashSet::new());
    let invalid_claims = RefCell::new(HashSet::new());
    map_grid(&mut grid, contents.as_str(), |grid, r, c, claim| {
        claims.borrow_mut().insert(claim);
        if grid[r][c] != claim {
            invalid_claims.borrow_mut().insert(claim);
        }
    });
    println!("{:?}", claims.borrow().difference(&(*invalid_claims.borrow())));
    Ok(())
}

fn map_grid<F>(grid: &mut [[usize; GRID_SIZE]; GRID_SIZE], claims: &str, map_fn: F)
    where F: Fn(&mut [[usize; GRID_SIZE]; GRID_SIZE], usize, usize, usize) -> ()
{
    for line in claims.lines() {
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        let coords: Vec<usize> = tokens[2][..tokens[2].len() - 1]
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let dims: Vec<usize> = tokens[3]
            .split("x")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let claim: usize = tokens[0][1..].parse().unwrap();
        let (x, y) = (coords[0], coords[1]);
        let (w, h) = (dims[0], dims[1]);
        for r in 0..h {
            for c in 0..w {
                map_fn(grid, y + r, x + c, claim);
            }
        }
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
