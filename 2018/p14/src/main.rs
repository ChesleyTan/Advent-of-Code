use std::collections::vec_deque::VecDeque;
use std::env;
use std::error::Error;

const INPUT: usize = 330121;

fn a() -> Result<(), Box<dyn Error>> {
    let mut recipes = VecDeque::new();
    recipes.push_back(3);
    recipes.push_back(7);
    let mut idx1 = 0;
    let mut idx2 = 1;
    while recipes.len() < INPUT + 10 {
        let new_recipes = next_recipes(recipes[idx1], recipes[idx2]);
        for recipe in new_recipes {
            recipes.push_back(recipe);
        }
        idx1 = (idx1 + 1 + recipes[idx1]) % recipes.len();
        idx2 = (idx2 + 1 + recipes[idx2]) % recipes.len();
    }
    if recipes.len() > INPUT + 10 {
        recipes.pop_back();
    }
    while recipes.len() > 10 {
        recipes.pop_front();
    }
    println!("{:?}", recipes);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let target = vec![3, 3, 0, 1, 2, 1];
    let mut last_n = VecDeque::new();
    let mut recipes = VecDeque::new();
    recipes.push_back(3);
    recipes.push_back(7);
    let mut idx1 = 0;
    let mut idx2 = 1;
    'outer: loop {
        let new_recipes = next_recipes(recipes[idx1], recipes[idx2]);
        for recipe in new_recipes {
            recipes.push_back(recipe);
            last_n.push_back(recipe);
            while last_n.len() > target.len() {
                last_n.pop_front();
            }
            if last_n == target {
                println!("{}", recipes.len() - target.len());
                break 'outer;
            }
        }
        idx1 = (idx1 + 1 + recipes[idx1]) % recipes.len();
        idx2 = (idx2 + 1 + recipes[idx2]) % recipes.len();
    }
    Ok(())
}

fn next_recipes(a: usize, b: usize) -> Vec<usize> {
    let result = a + b;
    if result >= 10 {
        vec![result / 10, result % 10]
    } else {
        vec![result]
    }
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
