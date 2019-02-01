use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;

type Coord = (isize, isize);
const DEPTH: isize = 11394;
const TARGET: Coord = (7, 701);
//const DEPTH: isize = 510;
//const TARGET: Coord = (10, 10);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tool {
    Gear,
    Torch,
    Neither,
}

#[derive(Debug, Clone)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Configuration {
    tool: Tool,
    coord: Coord,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    config: Configuration,
    time: isize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        let other_cost = other.time
            + (TARGET.0 - other.config.coord.0).abs()
            + (TARGET.1 - other.config.coord.1).abs();
        let cost = self.time
            + (TARGET.0 - self.config.coord.0).abs()
            + (TARGET.1 - self.config.coord.1).abs();
        Some(other_cost.cmp(&cost))
    }
}

fn a() -> Result<(), Box<dyn Error>> {
    let mut total_risk = 0;
    let mut erosion_memo = HashMap::new();
    for y in 0..=TARGET.1 {
        for x in 0..=TARGET.0 {
            total_risk += erosion_level(&mut erosion_memo, (x, y)) % 3;
        }
    }
    println!("{}", total_risk);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    println!("{}", rescue());
    Ok(())
}

fn erosion_level(erosion_memo: &mut HashMap<Coord, isize>, coord: Coord) -> isize {
    if erosion_memo.contains_key(&coord) {
        erosion_memo[&coord]
    } else {
        let level = (geologic_index(erosion_memo, coord) + DEPTH) % 20183;
        erosion_memo.insert(coord, level);
        level
    }
}

fn geologic_index(erosion_memo: &mut HashMap<Coord, isize>, coord: Coord) -> isize {
    match (coord.0, coord.1) {
        (0, 0) => 0,
        TARGET => 0,
        (x, 0) => x * 16807,
        (0, y) => y * 48271,
        (x, y) => erosion_level(erosion_memo, (x - 1, y)) * erosion_level(erosion_memo, (x, y - 1)),
    }
}

fn region_type(n: isize) -> RegionType {
    match n % 3 {
        0 => RegionType::Rocky,
        1 => RegionType::Wet,
        2 => RegionType::Narrow,
        _ => panic!("Invalid region type {}", n),
    }
}

fn valid_tools(region_type: RegionType) -> HashSet<Tool> {
    let mut tools = HashSet::new();
    match region_type {
        RegionType::Rocky => {
            tools.insert(Tool::Torch);
            tools.insert(Tool::Gear);
        }
        RegionType::Wet => {
            tools.insert(Tool::Gear);
            tools.insert(Tool::Neither);
        }
        RegionType::Narrow => {
            tools.insert(Tool::Torch);
            tools.insert(Tool::Neither);
        }
    }
    tools
}

fn valid_moves(coord: Coord) -> Vec<Coord> {
    match (coord.0, coord.1) {
        (0, 0) => vec![(0, 1), (1, 0)],
        (x, 0) => vec![(x - 1, 0), (x + 1, 0), (x, 1)],
        (0, y) => vec![(0, y - 1), (0, y + 1), (1, y)],
        (x, y) => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
    }
}

fn print_frontier(frontier: &BinaryHeap<State>) {
    if frontier.is_empty() {
        return;
    }
    let mut max_x = std::isize::MIN;
    let mut max_y = std::isize::MIN;
    for state in frontier.iter() {
        max_x = std::cmp::max(max_x, state.config.coord.0);
        max_y = std::cmp::max(max_y, state.config.coord.1);
    }
    let max_x = max_x as usize;
    let max_y = max_y as usize;
    let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];
    for state in frontier.iter() {
        grid[state.config.coord.1 as usize][state.config.coord.0 as usize] = 'X';
    }
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

fn rescue() -> isize {
    let mut erosion_memo: HashMap<Coord, isize> = HashMap::new();
    let mut fastest_times: HashMap<Configuration, isize> = HashMap::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        config: Configuration {
            tool: Tool::Torch,
            coord: (0, 0),
        },
        time: 0,
    });
    let goal = Configuration {
        tool: Tool::Torch,
        coord: TARGET,
    };
    while let Some(state) = frontier.pop() {
        //println!();
        //print_frontier(&frontier);
        // Exit early if this route cannot be the most efficient solution
        let fastest_to_current_state =
            *fastest_times.get(&state.config).unwrap_or(&std::isize::MAX);
        if state.time > fastest_to_current_state {
            continue;
        }
        if state.config == goal {
            println!("Found target in {}", state.time);
            return state.time;
        }
        let current_valid_tools = valid_tools(region_type(erosion_level(
            &mut erosion_memo,
            state.config.coord,
        )));
        for &tool in current_valid_tools.iter() {
            if tool == state.config.tool {
                continue;
            }
            let next_state = State {
                config: Configuration {
                    tool,
                    coord: state.config.coord,
                },
                time: state.time + 7,
            };
            if *fastest_times
                .get(&next_state.config)
                .unwrap_or(&std::isize::MAX)
                > next_state.time
            {
                fastest_times.insert(next_state.config, next_state.time);
                frontier.push(next_state);
            }
        }
        for &valid_move in valid_moves(state.config.coord).iter() {
            let valid_tools =
                valid_tools(region_type(erosion_level(&mut erosion_memo, valid_move)));
            if !valid_tools.contains(&state.config.tool) {
                continue;
            }
            let next_state = State {
                config: Configuration {
                    tool: state.config.tool,
                    coord: valid_move,
                },
                time: state.time + 1,
            };
            if *fastest_times
                .get(&next_state.config)
                .unwrap_or(&std::isize::MAX)
                > next_state.time
            {
                fastest_times.insert(next_state.config, next_state.time);
                frontier.push(next_state);
            }
        }
    }
    fastest_times[&goal]
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
