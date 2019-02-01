use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("25.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut constellations: Vec<HashSet<Point>> = vec![];
    for line in contents.lines() {
        let tokens: Vec<_> = line.split(",").map(parse_isize).collect();
        let point = (tokens[0], tokens[1], tokens[2], tokens[3]);
        let mut intersections = vec![];
        let mut non_intersections = vec![];
        'outer: for constellation in constellations.into_iter() {
            for &member in constellation.iter() {
                if dist(member, point) <= 3 {
                    intersections.push(constellation);
                    continue 'outer;
                }
            }
            non_intersections.push(constellation);
        }
        constellations = non_intersections;
        let mut new_constellation = HashSet::new();
        new_constellation.insert(point);
        for intersection in intersections.iter() {
            new_constellation.extend(intersection.iter());
        }
        constellations.push(new_constellation);
    }
    println!("{}", constellations.len());
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn dist(a: Point, b: Point) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

type Point = (isize, isize, isize, isize);

fn parse_isize(s: &str) -> isize {
    s.parse::<isize>().unwrap()
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
