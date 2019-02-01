use std::collections::hash_map::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("4.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let events: Vec<Event> = contents.lines().map(parse_event).collect();
    let sleep_times = calc_sleep_times(events);
    let mut sleepiest_guard = 0;
    let mut sleepiest_guard_mins = 0;
    for guard in sleep_times.keys() {
        let total_sleep: usize = sleep_times.get(guard).unwrap().iter().sum();
        if total_sleep > sleepiest_guard_mins {
            sleepiest_guard = *guard;
            sleepiest_guard_mins = total_sleep;
        }
    }
    let max_minute = argmax(sleep_times.get(&sleepiest_guard).unwrap().iter().collect());
    println! {"{}", sleepiest_guard * max_minute};
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("4.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let events: Vec<Event> = contents.lines().map(parse_event).collect();
    let sleep_times = calc_sleep_times(events);
    let mut sleepiest_guard = 0;
    let mut sleepiest_guard_mins = 0;
    for guard in sleep_times.keys() {
        let &max_sleep_min: &usize = sleep_times.get(guard).unwrap().iter().max().unwrap();
        if max_sleep_min > sleepiest_guard_mins {
            sleepiest_guard = *guard;
            sleepiest_guard_mins = max_sleep_min;
        }
    }
    let max_minute = argmax(sleep_times.get(&sleepiest_guard).unwrap().iter().collect());
    println! {"{}", sleepiest_guard * max_minute};
    Ok(())
}

fn calc_sleep_times(mut events: Vec<Event>) -> HashMap<usize, [usize; 60]> {
    events.sort_by_key(|event| (event.year, event.month, event.day, event.hr, event.min));
    let mut sleep_times: HashMap<usize, [usize; 60]> = HashMap::new();
    let mut guard = 0;
    let mut sleep_start = 0;
    for event in events {
        if event.desc.contains("begins shift") {
            guard = parse_usize(event.desc
                    .split(|c| c == '#' || c == ' ')
                    .skip(2)
                    .next()
                    .unwrap()
            );
            if !sleep_times.contains_key(&guard) {
                sleep_times.insert(guard, [0; 60]);
            }
        } else if event.desc.contains("falls asleep") {
            sleep_start = event.min;
        } else if event.desc.contains("wakes up") {
            let sleep_end = event.min;
            let times = sleep_times.get_mut(&guard).unwrap();
            for time in sleep_start..sleep_end + 1 {
                times[time] += 1;
            }
        }
    }
    sleep_times
}

#[derive(Debug)]
struct Event<'a> {
    year: usize,
    month: usize,
    day: usize,
    hr: usize,
    min: usize,
    desc: &'a str,
}

fn parse_event(line: &str) -> Event {
    let tokens: Vec<_> = line
        .split(|c| c == '[' || c == ']' || c == '-' || c == ':' || c == ' ' || c == '#')
        .collect();
    let desc = line.split("] ").skip(1).next().unwrap();
    Event {
        year: parse_usize(tokens[1]),
        month: parse_usize(tokens[2]),
        day: parse_usize(tokens[3]),
        hr: parse_usize(tokens[4]),
        min: parse_usize(tokens[5]),
        desc,
    }
}

fn parse_usize(n: &str) -> usize {
    n.parse::<usize>().unwrap()
}

fn argmax(arr: Vec<&usize>) -> usize {
    let mut max_val = 0;
    let mut max_index = 0;
    for (index, &&val) in arr.iter().enumerate() {
        if val > max_val {
            max_index = index;
            max_val = val;
        }
    }
    max_index
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
