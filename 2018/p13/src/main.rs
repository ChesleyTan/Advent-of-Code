use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_set::HashSet;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("13.input")?);
    let mut contents = String::new();
    let mut grid = vec![];
    reader.read_to_string(&mut contents)?;
    let mut carts = vec![];
    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    let mut cart_id = 0;
    for row in grid.iter_mut() {
        let mut row_carts = vec![];
        for val in row.iter_mut() {
            match val {
                '^' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Up)));
                    *val = '|';
                },
                '<' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Left)));
                    *val = '-';
                },
                '>' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Right)));
                    *val = '-';
                }
                'v' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Down)));
                    *val = '|';
                },
                _ => {
                    row_carts.push(None);
                }
            }
        }
        carts.push(row_carts);
    }
    'outer: loop {
        let mut moved_this_turn = HashSet::new();
        for r in 0..carts.len() {
            for c in 0..carts[r].len() {
                if carts[r][c].is_none() {
                    continue;
                }
                let cart_id = carts[r][c].unwrap().id;
                if moved_this_turn.contains(&cart_id) {
                    continue;
                }
                let collision = do_move(r, c, &mut carts, &grid);
                match collision {
                    None => {
                        moved_this_turn.insert(cart_id);
                    },
                    Some(loc) => {
                        println!("{:?}", (loc.1, loc.0));
                        break 'outer;
                    }
                }
            }
        }
    }
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("13.input")?);
    let mut contents = String::new();
    let mut grid = vec![];
    reader.read_to_string(&mut contents)?;
    let mut carts = vec![];
    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    let mut cart_id = 0;
    for row in grid.iter_mut() {
        let mut row_carts = vec![];
        for val in row.iter_mut() {
            match val {
                '^' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Up)));
                    *val = '|';
                },
                '<' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Left)));
                    *val = '-';
                },
                '>' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Right)));
                    *val = '-';
                }
                'v' => {
                    cart_id += 1;
                    row_carts.push(Some(Cart::new(cart_id, Direction::Down)));
                    *val = '|';
                },
                _ => {
                    row_carts.push(None);
                }
            }
        }
        carts.push(row_carts);
    }
    let num_carts = cart_id;
    let mut num_carts_removed = 0;
    loop {
        let mut moved_this_turn = HashSet::new();
        for r in 0..carts.len() {
            for c in 0..carts[r].len() {
                if carts[r][c].is_none() {
                    continue;
                }
                let cart_id = carts[r][c].unwrap().id;
                if moved_this_turn.contains(&cart_id) {
                    continue;
                }
                let collision = do_move(r, c, &mut carts, &grid);
                match collision {
                    None => {
                        moved_this_turn.insert(cart_id);
                    },
                    Some(loc) => {
                        // Remove collided carts
                        carts[r][c] = None;
                        carts[loc.0][loc.1] = None;
                        num_carts_removed += 2;
                    }
                }
            }
        }
        if num_carts_removed == num_carts - 1 {
            for r in 0..carts.len() {
                for c in 0..carts[r].len() {
                    if carts[r][c].is_some() {
                        println!("{:?}", (c, r));
                    }
                }
            }
            break;
        }
    }
    Ok(())
}

type Point = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up, Down, Left, Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TurnDirection {
    Left, Straight, Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cart {
    id: usize,
    direction: Direction,
    next_turn: TurnDirection,
}

impl Cart {
    fn new(id: usize, direction: Direction) -> Cart {
        Cart { id, direction, next_turn: TurnDirection::Left }
    }
}

fn next_turn_dir(turn_dir: TurnDirection) -> TurnDirection {
    match turn_dir {
        TurnDirection::Left => TurnDirection::Straight,
        TurnDirection::Straight => TurnDirection::Right,
        TurnDirection::Right => TurnDirection::Left,
    }
}

fn do_turn(dir: Direction, turn_dir: TurnDirection) -> Direction {
    match (dir, turn_dir) {
        (Direction::Up, TurnDirection::Left) => Direction::Left,
        (Direction::Up, TurnDirection::Right) => Direction::Right,
        (Direction::Left, TurnDirection::Left) => Direction::Down,
        (Direction::Left, TurnDirection::Right) => Direction::Up,
        (Direction::Down, TurnDirection::Left) => Direction::Right,
        (Direction::Down, TurnDirection::Right) => Direction::Left,
        (Direction::Right, TurnDirection::Left) => Direction::Up,
        (Direction::Right, TurnDirection::Right) => Direction::Down,
        (d, TurnDirection::Straight) => d,
    }
}

fn next_tile(cart: &Cart, cart_row: usize, cart_col: usize) -> Point {
    let mut next_row = cart_row;
    let mut next_col = cart_col;
    match cart.direction {
        Direction::Left => next_col -= 1,
        Direction::Right => next_col += 1,
        Direction::Up => next_row -= 1,
        Direction::Down => next_row += 1,
    }
    (next_row, next_col)
}

fn do_move(cart_row: usize, cart_col: usize, carts: &mut Vec<Vec<Option<Cart>>>, grid: &Vec<Vec<char>>) -> Option<Point> {
    let mut cart = carts[cart_row][cart_col].unwrap();
    let (next_row, next_col) = next_tile(&cart, cart_row, cart_col);
    let next_tile = grid[next_row][next_col];
    // Check collision
    if let Some(_) = carts[next_row][next_col] {
        return Some((next_row, next_col));
    }
    carts[cart_row][cart_col] = None;
    match next_tile {
        '-' | '|' => (),
        '+' => {
            cart.direction = do_turn(cart.direction, cart.next_turn);
            cart.next_turn = next_turn_dir(cart.next_turn);
        }
        '/' => {
            if cart.direction == Direction::Left || cart.direction == Direction::Right {
                if cart_col < next_col {
                    cart.direction = Direction::Up;
                } else {
                    cart.direction = Direction::Down;
                }
            } else {
                if cart_row < next_row {
                    cart.direction = Direction::Left;
                } else {
                    cart.direction = Direction::Right;
                }
            }
        }
        '\\' => {
            if cart.direction == Direction::Left || cart.direction == Direction::Right {
                if cart_col < next_col {
                    cart.direction = Direction::Down;
                } else {
                    cart.direction = Direction::Up;
                }
            } else {
                if cart_row < next_row {
                    cart.direction = Direction::Right;
                } else {
                    cart.direction = Direction::Left;
                }
            }
        }
        _ => ()
    }
    carts[next_row][next_col] = Some(cart);
    None
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
