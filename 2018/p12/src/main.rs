use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_map::HashMap;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("12.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut state: HashMap<isize, usize> = HashMap::new();
    let mut rules: HashMap<usize, usize> = HashMap::new();
    let initial_state = contents.lines().next().unwrap().split(": ").nth(1).unwrap();
    for (idx, c) in initial_state.chars().enumerate() {
        state.insert(idx as isize, if c == '#' { 1 } else { 0 });
    }
    for line in contents.lines().skip(2) {
        let mut local_state = 0;
        for c in line[0..5].chars() {
            local_state *= 10;
            local_state += if c == '#' { 1 } else { 0 };
        }
        rules.insert(local_state, if line.chars().nth(9).unwrap() == '#' { 1 } else { 0 });
    }
    for _ in 0..20 {
        simulate(&mut state, &rules);
        let min_idx = *state.keys().min().unwrap();
        let max_idx = *state.keys().max().unwrap();
        for idx in min_idx..max_idx {
            print!("{}", state.get(&idx).unwrap());
        }
        println!();
    }
    let mut total = 0;
    for (&idx, &val) in state.iter() {
        if val != 0 {
            total += idx;
        }
    }
    println!("{}", total);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("12.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut state: HashMap<isize, usize> = HashMap::new();
    let mut rules: HashMap<usize, usize> = HashMap::new();
    let initial_state = contents.lines().next().unwrap().split(": ").nth(1).unwrap();
    for (idx, c) in initial_state.chars().enumerate() {
        state.insert(idx as isize, if c == '#' { 1 } else { 0 });
    }
    for line in contents.lines().skip(2) {
        let mut local_state = 0;
        for c in line[0..5].chars() {
            local_state *= 10;
            local_state += if c == '#' { 1 } else { 0 };
        }
        rules.insert(local_state, if line.chars().nth(9).unwrap() == '#' { 1 } else { 0 });
    }
    let mut state_str = String::new();
    let mut num_steps = 0;
    // Reaches a fixed point
    loop {
        simulate(&mut state, &rules);
        num_steps += 1;
        let mut new_state_str = String::new();
        let min_idx = *state.keys().min().unwrap();
        let max_idx = *state.keys().max().unwrap();
        for idx in min_idx..max_idx {
            new_state_str.push_str(&format!("{}", state.get(&idx).unwrap()));
        }
        println!("{}", new_state_str);
        if new_state_str == state_str {
            break;
        }
        state_str = new_state_str;
    }
    let mut total_prev = 0;
    for (&idx, &val) in state.iter() {
        if val != 0 {
            total_prev += idx;
        }
    }
    simulate(&mut state, &rules);
    let mut total = 0;
    for (&idx, &val) in state.iter() {
        if val != 0 {
            total += idx;
        }
    }
    println!("{}", total_prev + (total - total_prev) * (50_000_000_000 - num_steps));
    Ok(())
}

fn simulate(state: &mut HashMap<isize, usize>, rules: &HashMap<usize, usize>) {
    let prev_state = HashMap::clone(state);
    let min_idx = *state.keys().min().unwrap();
    let max_idx = *state.keys().max().unwrap();
    let mut local_state = 0;
    for idx in min_idx..max_idx+5 {
        local_state *= 10;
        local_state += prev_state.get(&idx).unwrap_or(&0);
        local_state %= 100000;
        state.insert(idx - 2, *rules.get(&local_state).unwrap_or(&0));
    }
    // Remove contiguous blocks of zeros on the sides
    let mut min_idx = *state.keys().min().unwrap();
    let mut max_idx = *state.keys().max().unwrap();
    while let Some(0) = state.get(&min_idx) {
        state.remove(&min_idx);
        min_idx += 1;
    }
    while let Some(0) = state.get(&max_idx) {
        state.remove(&max_idx);
        max_idx -= 1;
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
