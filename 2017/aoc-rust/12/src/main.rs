#![feature(range_contains)]

extern crate regex;
use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Op {
    CPY(String, String),
    INC(String),
    DEC(String),
    JNZ(String, i32),
    UNKNOWN
}

fn parse_int(str: String) -> i32 {
    if str.chars().nth(0).unwrap() == '-' {
        println!("{:?}", str.chars());
        str[1..str.len()].parse::<i32>().unwrap()*-1
    } else {
        str.parse::<i32>().unwrap()
    }
}

fn reg_or_val(registers: &HashMap<String, i32>, str: String) -> i32 {
    let val = registers.get(&str);

    match val {
        Some(v) => *v,
        None if !("a".."z").contains(&str) => parse_int(str),
        None => 0
    }
}

fn main() {
    use std::io;
    use regex::Regex;
    use std::io::prelude::*;

    let stdin = io::stdin();

    let re = Regex::new(r"([a-z-]{3}) ([a-z0-9]+) ?([-a-z0-9]+)?").unwrap();
    let mut registers: HashMap<String, i32> = HashMap::new();

    // Part 2
    registers.insert("c".to_string(), 1);

    let mut ops: Vec<Op> = Vec::new();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        println!("{:?}", input);
        let caps = re.captures(input.as_str()).unwrap();

        println!("{:?}", caps);
        println!("{:?}", caps.at(3));

        let op = match caps.at(1).unwrap() {
            "cpy"=> Op::CPY(caps.at(2).unwrap().to_string(),
                            caps.at(3).unwrap().to_string()),
            "inc" => Op::INC(caps.at(2).unwrap().to_string()),
            "dec" => Op::DEC(caps.at(2).unwrap().to_string()),
            "jnz" => Op::JNZ(caps.at(2).unwrap().to_string(),
                             parse_int(caps.at(3).unwrap().to_string())),
             _ => Op::UNKNOWN
        };

        ops.push(op);
    }

    println!("{:?}", ops);

    let mut ic = 0isize;
    while ic < ops.len() as isize {
        let op = ops[ic as usize].clone();
        ic += 1;

        // println!("ic: {:?}", ic);
        match op {
            Op::CPY(value, register) => {
                let value2 = reg_or_val(&registers, value);
                registers.insert(register, value2).unwrap_or(0);
            },
            Op::INC(register) => {
                let count = registers.entry(register).or_insert(0);
                *count += 1;
            },
            Op::DEC(register) => {
                let count = registers.entry(register).or_insert(0);
                *count -= 1;
            },
            Op::JNZ(register, skip) => {
                if reg_or_val(&registers, register) != 0i32 {
                    //println!("before jnz {:?}, ic: {:?}, r: {:?}", skip, ic, registers);
                    ic += skip as isize - 1;
                    //println!("after jnz {:?}, ic: {:?}, r: {:?}", skip, ic, registers);
                }
            },
            Op::UNKNOWN => {}
        };

        //std::thread::sleep_ms(100);
    }

    println!("{:?}", registers);
}
