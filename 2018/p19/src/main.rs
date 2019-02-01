use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const NUM_REGISTERS: usize = 6;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("19.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut program = vec![];
    let ip_reg = parse_usize(&contents.lines().next().unwrap()[4..]);
    for line in contents.lines().skip(1){
        program.push(Instruction::new(line));
    }
    let mut regs = [0 as isize; NUM_REGISTERS];
    let mut ip: isize = 0;
    while 0 <= ip && (ip as usize) < program.len() {
        let insn = &program[ip as usize];
        regs[ip_reg] = ip;
        match insn.opcode.as_ref() {
            "addr" => regs = addr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "addi" => regs = addi(&regs, insn.a, insn.b, insn.c).unwrap(),
            "mulr" => regs = mulr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "muli" => regs = muli(&regs, insn.a, insn.b, insn.c).unwrap(),
            "banr" => regs = banr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "bani" => regs = bani(&regs, insn.a, insn.b, insn.c).unwrap(),
            "borr" => regs = borr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "bori" => regs = bori(&regs, insn.a, insn.b, insn.c).unwrap(),
            "setr" => regs = setr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "seti" => regs = seti(&regs, insn.a, insn.b, insn.c).unwrap(),
            "gtir" => regs = gtir(&regs, insn.a, insn.b, insn.c).unwrap(),
            "gtri" => regs = gtri(&regs, insn.a, insn.b, insn.c).unwrap(),
            "gtrr" => regs = gtrr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "eqir" => regs = eqir(&regs, insn.a, insn.b, insn.c).unwrap(),
            "eqri" => regs = eqri(&regs, insn.a, insn.b, insn.c).unwrap(),
            "eqrr" => regs = eqrr(&regs, insn.a, insn.b, insn.c).unwrap(),
            _ => ()
        }
        ip = regs[ip_reg] + 1;
    }
    println!("{}", regs[0]);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("19.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut program = vec![];
    let ip_reg = parse_usize(&contents.lines().next().unwrap()[4..]);
    for line in contents.lines().skip(1){
        program.push(Instruction::new(line));
    }
    let mut regs = [0 as isize; NUM_REGISTERS];
    regs[0] = 1;
    let mut ip: isize = 0;
    // Iterate to compute initialization of r2
    for _ in 0..1000 {
        let insn = &program[ip as usize];
        regs[ip_reg] = ip;
        match insn.opcode.as_ref() {
            "addr" => regs = addr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "addi" => regs = addi(&regs, insn.a, insn.b, insn.c).unwrap(),
            "mulr" => regs = mulr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "muli" => regs = muli(&regs, insn.a, insn.b, insn.c).unwrap(),
            "banr" => regs = banr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "bani" => regs = bani(&regs, insn.a, insn.b, insn.c).unwrap(),
            "borr" => regs = borr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "bori" => regs = bori(&regs, insn.a, insn.b, insn.c).unwrap(),
            "setr" => regs = setr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "seti" => regs = seti(&regs, insn.a, insn.b, insn.c).unwrap(),
            "gtir" => regs = gtir(&regs, insn.a, insn.b, insn.c).unwrap(),
            "gtri" => regs = gtri(&regs, insn.a, insn.b, insn.c).unwrap(),
            "gtrr" => regs = gtrr(&regs, insn.a, insn.b, insn.c).unwrap(),
            "eqir" => regs = eqir(&regs, insn.a, insn.b, insn.c).unwrap(),
            "eqri" => regs = eqri(&regs, insn.a, insn.b, insn.c).unwrap(),
            "eqrr" => regs = eqrr(&regs, insn.a, insn.b, insn.c).unwrap(),
            _ => ()
        }
        ip = regs[ip_reg] + 1;
    }
    // Compute sum of factors of r2
    let r2 = regs[2];
    let mut r0 = 0;
    for factor in 1..=r2 {
        if r2 % factor == 0 {
            r0 += factor;
        }
    }
    println!("{}", r0);
    Ok(())
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: String,
    a: isize,
    b: isize,
    c: isize,

}

impl Instruction {
    fn new(insn: &str) -> Instruction {
        let tokens: Vec<_> = insn.split_whitespace().collect();
        Instruction {
            opcode: String::from(tokens[0]),
            a: parse_isize(tokens[1]),
            b: parse_isize(tokens[2]),
            c: parse_isize(tokens[3]),
        }
    }
}

fn parse_isize(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}

fn parse_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn valid_reg(r: isize) -> bool {
    0 <= r && r <= (NUM_REGISTERS as isize) - 1
}

fn addr(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] + new_regs[b as usize];
    Some(new_regs)
}

fn addi(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] + b;
    Some(new_regs)
}

fn mulr(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] * new_regs[b as usize];
    Some(new_regs)
}

fn muli(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] * b;
    Some(new_regs)
}

fn banr(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] & new_regs[b as usize];
    Some(new_regs)
}

fn bani(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] & b;
    Some(new_regs)
}

fn borr(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] | new_regs[b as usize];
    Some(new_regs)
}

fn bori(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize] | b;
    Some(new_regs)
}

fn setr(regs: &[isize; NUM_REGISTERS], a: isize, _b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = new_regs[a as usize];
    Some(new_regs)
}

fn seti(regs: &[isize; NUM_REGISTERS], a: isize, _b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = a;
    Some(new_regs)
}

fn gtir(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if a > new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
}

fn gtri(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] > b { 1 } else { 0 };
    Some(new_regs)
}

fn gtrr(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] > new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
}

fn eqir(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(b) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if a == new_regs[b as usize] { 1 } else { 0 };
    Some(new_regs)
}

fn eqri(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
    if !valid_reg(a) || !valid_reg(c) {
        return None;
    }
    let mut new_regs = regs.clone();
    new_regs[c as usize] = if new_regs[a as usize] == b { 1 } else { 0 };
    Some(new_regs)
}

fn eqrr(regs: &[isize; NUM_REGISTERS], a: isize, b: isize, c: isize) -> Option<[isize; NUM_REGISTERS]> {
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
