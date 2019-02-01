use std::cmp;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("5.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut polymers: Vec<char> = contents.lines().next().unwrap().chars().collect();
    let mut prev_len = 0;
    while polymers.len() != prev_len {
        prev_len = polymers.len();
        polymers = reduce(polymers);
    }
    println!("{}", polymers.len());
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("5.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let polymers: Vec<char> = contents.lines().next().unwrap().chars().collect();
    let unique_polymers: HashSet<char> = contents
        .lines()
        .next()
        .unwrap()
        .to_ascii_uppercase()
        .chars()
        .collect();
    let mut min_len_result = polymers.len();
    for polymer in unique_polymers {
        let mut other_polymers: Vec<char> = polymers
            .iter()
            .filter(|&c| c.to_ascii_uppercase() != polymer)
            .map(|&c| c)
            .collect();
        let mut prev_len = 0;
        while other_polymers.len() != prev_len {
            prev_len = other_polymers.len();
            other_polymers = reduce(other_polymers);
        }
        min_len_result = cmp::min(min_len_result, other_polymers.len());
    }
    println!("{}", min_len_result);
    Ok(())
}

fn reduce(polymers: Vec<char>) -> Vec<char> {
    let mut new_polymers = vec![];
    for c in polymers {
        if new_polymers.len() == 0 {
            new_polymers.push(c);
        } else {
            let &last_polymer: &char = new_polymers.last().unwrap();
            if last_polymer != c && last_polymer.to_ascii_uppercase() == c.to_ascii_uppercase() {
                new_polymers.pop();
                continue;
            } else {
                new_polymers.push(c);
            }
        }
    }
    new_polymers
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
