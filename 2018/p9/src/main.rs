use std::collections::vec_deque::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("9.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let tokens: Vec<_> = contents.split(" ").collect();
    let num_players = parse_usize(tokens[0]);
    let num_marbles = parse_usize(tokens[6]);
    println!("{}", simulate(num_players, num_marbles));
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("9.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let tokens: Vec<_> = contents.split(" ").collect();
    let num_players = parse_usize(tokens[0]);
    let num_marbles = 100 * parse_usize(tokens[6]);
    println!("{}", simulate_deque(num_players, num_marbles));
    Ok(())
}

fn simulate_deque(num_players: usize, num_marbles: usize) -> usize {
    let mut scores = vec![0; num_players];
    // Keep current marble at tail of deque
    let mut marbles = VecDeque::new();
    marbles.push_back(0);
    marbles.push_back(1);
    let mut player = 2;
    for marble_num in 2..num_marbles {
        if marble_num % 23 == 0 {
            // rotate clockwise past next 6 counterclockwise marbles
            for _ in 0..6 {
                let marble = marbles.pop_front().unwrap();
                marbles.push_back(marble);
            }
            scores[player] += marble_num + marbles.pop_front().unwrap();
        } else {
            // rotate counterclockwise past current marble and marble 1 clockwise position away
            for _ in 0..2 {
                let marble = marbles.pop_back().unwrap();
                marbles.push_front(marble);
            }
            marbles.push_back(marble_num);
        }
        player = (player + 1) % num_players;
    }
    *scores.iter().max().unwrap()
}

fn simulate(num_players: usize, num_marbles: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut marbles = vec![0, 1];
    let mut current_marble = 1;
    let mut player = 2;
    for marble_num in 2..num_marbles {
        if marble_num % 23 == 0 {
            current_marble = counterclockwise(7, current_marble, marbles.len());
            let reward_marble = marbles.remove(current_marble);
            scores[player] += marble_num + reward_marble;
        } else {
            current_marble = clockwise(2, current_marble, marbles.len());
            marbles.insert(current_marble, marble_num);
        }
        player = (player + 1) % num_players;
    }
    *scores.iter().max().unwrap()
}

fn clockwise(units: usize, start: usize, circle_size: usize) -> usize {
    (start + units) % circle_size
}

fn counterclockwise(units: usize, start: usize, circle_size: usize) -> usize {
    let units = units as isize;
    let start = start as isize;
    let circle_size = circle_size as isize;
    let remainder = (start - units) % circle_size;
    if remainder < 0 {
        (remainder + circle_size) as usize
    } else {
        remainder as usize
    }
}

fn parse_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
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
