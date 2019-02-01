use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("10.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut points: Vec<Point> = contents.lines().map(Point::new).collect();
    for step in 1..100000 {
        simulate(&mut points);
        print_points(&points, step);
    }
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("10.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut points: Vec<Point> = contents.lines().map(Point::new).collect();
    for step in 1..100000 {
        simulate(&mut points);
        print_points(&points, step);
    }
    Ok(())
}

fn simulate(points: &mut Vec<Point>) {
    for point in points.iter_mut() {
        point.position.0 += point.velocity.0;
        point.position.1 += point.velocity.1;
    }
}

fn print_points(points: &Vec<Point>, timestep: usize) {
    let mut min_x = std::isize::MAX / 2;
    let mut max_x = std::isize::MIN / 2;
    let mut min_y = std::isize::MAX / 2;
    let mut max_y = std::isize::MIN / 2;
    for point in points.iter() {
        min_x = std::cmp::min(min_x, point.position.0);
        max_x = std::cmp::max(max_x, point.position.0);
        min_y = std::cmp::min(min_y, point.position.1);
        max_y = std::cmp::max(max_y, point.position.1);
    }
    if max_x - min_x < 100 && max_y - min_y < 15 {
        let mut grid = [[0; 100]; 15];
        for point in points.iter() {
            grid[(point.position.1 - min_y) as usize][(point.position.0 - min_x) as usize] = 1;
        }
        println!("Step {}", timestep);
        for row in grid.iter() {
            for &col in row.iter() {
                print!("{}", if col == 0 { '.' } else { '#' });
            }
            println!();
        }
    }
}

struct Point {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Point {
    fn new(desc: &str) -> Point {
        let desc = desc.replace("< ", "<");
        let tokens: Vec<_> = desc.split_whitespace().collect();
        let x = parse_isize(tokens[0].split(|c| c == '<' || c == ',').skip(1).next().unwrap());
        let y = parse_isize(tokens[1].split(">").next().unwrap());
        let x_vel = parse_isize(tokens[2].split(|c| c == '<' || c == ',').skip(1).next().unwrap());
        let y_vel = parse_isize(tokens[3].split(">").next().unwrap());
        Point { position: (x, y), velocity: (x_vel, y_vel) }
    }
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
