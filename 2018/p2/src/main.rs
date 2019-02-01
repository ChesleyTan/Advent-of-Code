use counter::Counter;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("2.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut num2s = 0;
    let mut num3s = 0;
    for line in contents.lines() {
        let char_counts: Counter<_> = line.chars().collect::<Counter<_>>();
        let mut has2 = false;
        let mut has3 = false;
        for &val in char_counts.values() {
            has2 = has2 || val == 2;
            has3 = has3 || val == 3;
        }
        if has2 {
            num2s += 1;
        }
        if has3 {
            num3s += 1;
        }
    }
    println!("{}", num2s * num3s);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("2.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.lines().collect();
    'outer: for (idx, line1) in lines.iter().enumerate() {
        for line2 in lines.iter().skip(idx + 1) {
            let overlap = check_similar(line1, line2);
            if overlap.is_some() {
                println!("{}", overlap.unwrap());
                break 'outer;
            }
        }
    }
    Ok(())
}

fn check_similar(line1: &str, line2: &str) -> Option<String> {
    let mut differing_char: Option<char> = None;
    let mut same_chars: Vec<char> = vec![];
    for (a, b) in line1.chars().zip(line2.chars()) {
        if a != b {
            if differing_char != None {
                return None;
            }
            differing_char = Some(a);
        } else {
            same_chars.push(a);
        }
    }
    Some(same_chars.iter().collect::<String>())
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
