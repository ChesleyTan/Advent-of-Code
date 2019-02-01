use std::env;
use std::error::Error;

const INPUT: isize = 9445;

fn a() -> Result<(), Box<dyn Error>> {
    let mut grid = [[0; 300]; 300];
    for r in 0..300 {
        for c in 0..300 {
            grid[r][c] = value(c + 1, r + 1);
        }
    }
    let mut max_grid_value = std::isize::MIN;
    let mut max_grid_loc = (0, 0);
    for r in 0..300-2 {
        for c in 0..300-2 {
            let mut grid_value = 0;
            for row in r..r+3 {
                for col in c..c+3 {
                    grid_value += grid[row][col];
                }
            }
            if grid_value > max_grid_value {
                max_grid_value = grid_value;
                max_grid_loc = (c + 1, r + 1);
            }
        }
    }
    println!("{:?}", max_grid_loc);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut grid = [[0; 300]; 300];
    for r in 0..300 {
        for c in 0..300 {
            grid[r][c] = value(c + 1, r + 1);
        }
    }
    let mut max_grid_value = std::isize::MIN;
    let mut max_grid_loc = (0, 0);
    let mut max_grid_size = 0;
    for size in 1..300+1 {
        let mut sum_grid_row = [[0; 300]; 300];
        let mut sum_grid_col = [[0; 300]; 300];
        for r in 0..300 {
            let mut row_value = 0;
            for col in 0..size {
                row_value += grid[r][col];
            }
            sum_grid_row[r][0] = row_value;
            for c in 1..300-size+1 {
                row_value -= grid[r][c-1];
                row_value += grid[r][c+size-1];
                sum_grid_row[r][c] = row_value;
            }
        }
        for c in 0..300 {
            let mut col_value = 0;
            for row in 0..size {
                col_value += grid[row][c];
            }
            sum_grid_col[0][c] = col_value;
            for r in 1..300-size+1 {
                col_value -= grid[r-1][c];
                col_value += grid[r+size-1][c];
                sum_grid_col[r][c] = col_value;
            }
        }
        let mut grid_value = 0;
        for r in 0..size {
            for c in 0..size {
                grid_value += grid[r][c];
            }
        }
        for r in 0..300-size+1 {
            if r > 0 {
                grid_value -= sum_grid_row[r-1][0];
                grid_value += sum_grid_row[r+size-1][0];
            }
            let row_start_value = grid_value;
            for c in 0..300-size+1 {
                if c > 0 {
                    grid_value -= sum_grid_col[r][c-1];
                    grid_value += sum_grid_col[r][c+size-1];
                }
                if grid_value > max_grid_value {
                    max_grid_value = grid_value;
                    max_grid_loc = (c + 1, r + 1);
                    max_grid_size = size;
                }
            }
            grid_value = row_start_value;
        }
    }
    println!("{:?}", max_grid_loc);
    println!("{}", max_grid_size);
    Ok(())
}

fn value(x: usize, y: usize) -> isize {
    let x = x as isize;
    let y = y as isize;
    (((x + 10) * ((x + 10) * y + INPUT) / 100) % 10) - 5
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
