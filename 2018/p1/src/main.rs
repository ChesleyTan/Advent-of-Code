use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_set::HashSet;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("1.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let total = contents.lines().fold(0, |acc, change| {
        acc + change.parse::<i32>().unwrap()
    });
    println!("{}", total);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("1.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut past_freqs = HashSet::new();
    let mut total = 0;
    'outer: loop {
        for line in contents.lines() {
            total += line.parse::<i32>().unwrap();
            if past_freqs.contains(&total) {
                break 'outer;
            } else {
                past_freqs.insert(total);
            }
        }
    }
    println!("{}", total);
    Ok(())
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
