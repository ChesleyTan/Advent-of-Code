use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;

fn a() -> Result<(), Box<dyn Error>> {
    let mut r1: isize = 0;
    let mut r3: isize = 0;
    let mut r0 = 0;
    loop {
        r1 = r3 | 65536;
        r3 = 4921097;
        loop {
            r3 = (((r3 + (r1 & 255)) & 16777215) * 65899) & 16777215;
            if r1 < 256 { break; }
            r1 /= 256;
        }
        r0 = r3; // Capture first value of r3 used in loop guard
        if r3 == r0 { break; }
    }
    println!("{}", r0);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut r1: isize = 0;
    let mut r3: isize = 0;
    let mut history = HashSet::new();
    let mut prev = 0;
    loop {
        r1 = r3 | 65536;
        r3 = 4921097;
        loop {
            r3 = (((r3 + (r1 & 255)) & 16777215) * 65899) & 16777215;
            if r1 < 256 { break; }
            r1 /= 256;
        }
        // Find first duplicate value of r3
        if history.contains(&r3) {
            break;
        } else {
            history.insert(r3);
            prev = r3;
        }
        //if r3 == r0 { break; }
    }
    // Answer is the last value of r3 in the cycle
    println!("{}", prev);
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
