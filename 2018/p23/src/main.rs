use lazy_static::lazy_static;
use regex::Regex;
use std::collections::binary_heap::BinaryHeap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::Ordering;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("23.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut bots = vec![];
    for line in contents.lines() {
        bots.push(Nanobot::parse(line));
    }
    let mut largest = &bots[0];
    for bot in bots.iter() {
        if bot.r > largest.r {
            largest = bot;
        }
    }
    let mut in_radius = 0;
    for bot in bots.iter() {
        if dist(largest.pos, bot.pos) <= largest.r {
            in_radius += 1;
        }
    }
    println!("{}", in_radius);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("23.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut bots = vec![];
    for line in contents.lines() {
        bots.push(Nanobot::parse(line));
    }
    let mut partitions = BinaryHeap::new();
    for bot in bots.iter() {
        partitions.push(enclosing_partition(bot, &bots));
    }
    // Create cube partitions around each octahedron
    // Find the cube partition of smallest size with the most bots in range with the smallest
    // distance to the origin
    while let Some(part) = partitions.pop() {
        if part.size == 0 {
            println!("{:?}", part);
            println!("{}", part.min.0 + part.min.1 + part.min.2);
            break;
        } else if part.size <= 3 {
            partitions.push(Partition::new(part.min, part.min, &bots));
            partitions.push(Partition::new(part.max, part.max, &bots));
        }
        else {
            let xs = [part.min.0, (part.min.0 + part.max.0) / 2, part.max.0];
            let ys = [part.min.1, (part.min.1 + part.max.1) / 2, part.max.1];
            let zs = [part.min.2, (part.min.2 + part.max.2) / 2, part.max.2];
            for x in 0..=1 {
                for y in 0..=1 {
                    for z in 0..=1 {
                        let (x_min, x_max) = (xs[x], xs[x + 1]);
                        let (y_min, y_max) = (xs[y], xs[y + 1]);
                        let (z_min, z_max) = (xs[z], xs[z + 1]);
                        let subpart = Partition::new((x_min, y_min, z_min), (x_max, y_max, z_max), &bots);
                        partitions.push(subpart);
                    }
                }
            }
        }
    }
    Ok(())
}

fn dist(a: Coord, b: Coord) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

type Coord = (isize, isize, isize);

#[derive(Debug, Copy, Clone)]
struct Nanobot {
    pos: Coord,
    r: isize,
}

impl Nanobot {
    fn parse(desc: &str) -> Nanobot {
        lazy_static! {
            static ref NANOBOT_REGEX: Regex = Regex::new(r"pos=<(.*),(.*),(.*)>, r=(.*)").unwrap();
        }
        let cap = &NANOBOT_REGEX.captures(desc).unwrap();
        Nanobot {
            pos: (parse_isize(&cap[1]), parse_isize(&cap[2]), parse_isize(&cap[3])),
            r: parse_isize(&cap[4]),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Partition {
    min: Coord,
    max: Coord,
    num_bots_in_range: isize,
    dist_to_origin: isize,
    size: isize,
}

impl Partition {
    fn new(min: Coord, max: Coord, bots: &Vec<Nanobot>) -> Partition {
        let mut num_bots_in_range = 0;
        for bot in bots.iter() {
            let mut dist = 0;
            dist += dist_outside(min.0, max.0, bot.pos.0);
            dist += dist_outside(min.1, max.1, bot.pos.1);
            dist += dist_outside(min.2, max.2, bot.pos.2);
            if dist <= bot.r {
                num_bots_in_range += 1;
            }
        }
        let dist_to_origin = dist((
                (min.0 + max.0) / 2,
                (min.1 + max.1) / 2,
                (min.2 + max.2) / 2), (0, 0, 0));
        Partition {
            min,
            max,
            num_bots_in_range,
            dist_to_origin,
            size: max.0 - min.0 + max.1 - min.1 + max.2 - min.2,
        }
    }
}

impl Ord for Partition {
    fn cmp(&self, other: &Partition) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Partition {
    fn partial_cmp(&self, other: &Partition) -> Option<Ordering> {
        Some(self.num_bots_in_range.cmp(&other.num_bots_in_range)
             .then(other.dist_to_origin.cmp(&self.dist_to_origin))
             .then(other.size.cmp(&self.size)))
    }
}

fn dist_outside(min: isize, max: isize, pos: isize) -> isize {
    if pos < min {
        min - pos
    } else if pos > max {
        pos - max
    } else {
        0
    }
}

fn enclosing_partition(bot: &Nanobot, bots: &Vec<Nanobot>) -> Partition {
    Partition::new(
        (bot.pos.0 - bot.r, bot.pos.1 - bot.r, bot.pos.2 - bot.r),
        (bot.pos.0 + bot.r, bot.pos.1 + bot.r, bot.pos.2 + bot.r),
        bots
    )
}

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
