use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use itertools::Itertools;

use regex::Regex;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("16.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut examples = vec![];
    let ops = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];
    for lines in &contents.lines().chunks(4) {
        let example_desc = lines.collect_vec();
        if !example_desc[0].contains("Before") {
            break;
        } else {
            examples.push(Example::new(example_desc[0], example_desc[1], example_desc[2]));
        }
    }
    let mut num_at_least_3 = 0;
    for example in examples.iter() {
        let mut num_matching_ops = 0;
        for op in ops.iter() {
            if let Some(new_regs) = op(&example.registers_before, example.insn.a, example.insn.b, example.insn.c) {
                if new_regs == example.registers_after {
                    num_matching_ops += 1;
                }
            }
        }
        if num_matching_ops >= 3 {
            num_at_least_3 += 1;
        }
    }
    println!("{:?}", num_at_least_3);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("16.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut examples = vec![];
    let mut program = vec![];
    let ops = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];
    let mut lines_parsed = 0;
    for lines in &contents.lines().chunks(4) {
        let example_desc = lines.collect_vec();
        if !example_desc[0].contains("Before") {
            break;
        } else {
            examples.push(Example::new(example_desc[0], example_desc[1], example_desc[2]));
            lines_parsed += 4;
        }
    }
    for line in contents.lines().skip(lines_parsed + 2) {
        program.push(Instruction::new(line));
    }
    let mut opcode_mapping = HashMap::new();
    for opcode in 0..=15 {
        let possible_funcs: HashSet<usize> = (0..=15).collect();
        opcode_mapping.insert(opcode, possible_funcs);
    }
    for example in examples.iter() {
        for (op_idx, op) in ops.iter().enumerate() {
            let mut valid = false;
            if let Some(new_regs) = op(&example.registers_before, example.insn.a, example.insn.b, example.insn.c) {
                if new_regs == example.registers_after {
                    valid = true;
                }
            }
            if !valid {
                opcode_mapping.get_mut(&example.insn.opcode).unwrap().remove(&op_idx);
            }
        }
    }
    let mut resolved_mapping = HashMap::new();
    let mut resolved_ops = HashSet::new();
    while resolved_mapping.len() != ops.len() {
        for (&opcode, possible_ops) in opcode_mapping.iter() {
            if possible_ops.len() == 1 {
                let op_idx = *possible_ops.iter().next().unwrap();
                resolved_mapping.insert(opcode, op_idx);
                resolved_ops.insert(op_idx);
            }
        }
        for (_, possible_ops) in opcode_mapping.iter_mut() {
            possible_ops.retain(|op_idx| !resolved_ops.contains(op_idx));
        }
    }
    //println!("{:?}", resolved_mapping);
    let mut regs = [0; 4];
    for insn in program.iter() {
        regs = ops[resolved_mapping[&insn.opcode]](&regs, insn.a, insn.b, insn.c).unwrap();
    }
    println!("{:?}", regs[0]);
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: isize,
    a: isize,
    b: isize,
    c: isize,

}

impl Instruction {
    fn new(insn: &str) -> Instruction {
        let tokens: Vec<_> = insn.split_whitespace().collect();
        Instruction {
            opcode: parse_isize(tokens[0]),
            a: parse_isize(tokens[1]),
            b: parse_isize(tokens[2]),
            c: parse_isize(tokens[3]),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Example {
    registers_before: [isize; 4],
    registers_after: [isize; 4],
    insn: Instruction,
}

impl Example {
    fn new(reg_before: &str, insn: &str, reg_after: &str) -> Example {
        Example {
            registers_before: Self::parse_reg(reg_before),
            registers_after: Self::parse_reg(reg_after),
            insn: Instruction::new(insn),
        }
    }

    fn parse_reg(reg: &str) -> [isize; 4] {
        let r = Regex::new(r"\[(.*)\]").unwrap();
        let cap = &r.captures(reg).unwrap()[1];
        let regex_match: Vec<_> = cap.split(", ").collect();
        [
            parse_isize(regex_match[0]),
            parse_isize(regex_match[1]),
            parse_isize(regex_match[2]),
            parse_isize(regex_match[3])
        ]
    }
}

fn parse_isize(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}

fn valid_reg(r: isize) -> bool {
    0 <= r && r <= 3
}

fn addr(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] + new_regs[b as usize];
    Some(new_regs)
}

fn addi(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] + b;
    Some(new_regs)
}

fn mulr(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] * new_regs[b as usize];
    Some(new_regs)
}

fn muli(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] * b;
    Some(new_regs)
}

fn banr(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] & new_regs[b as usize];
    Some(new_regs)
}

fn bani(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] & b;
    Some(new_regs)
}

fn borr(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] | new_regs[b as usize];
    Some(new_regs)
}

fn bori(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] | b;
    Some(new_regs)
}

fn setr(regs: &[isize; 4], a: isize, _b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize];
    Some(new_regs)
}

fn seti(regs: &[isize; 4], a: isize, _b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = a;
    Some(new_regs)
}

fn gtir(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if a > new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
}

fn gtri(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] > b { 1 } else { 0 };
    Some(new_regs)
}

fn gtrr(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] > new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
}

fn eqir(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if a == new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
}

fn eqri(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] == b { 1 } else { 0 };
    Some(new_regs)
}

fn eqrr(regs: &[isize; 4], a: isize, b: isize, c: isize) -> Option<[isize; 4]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] == new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
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
