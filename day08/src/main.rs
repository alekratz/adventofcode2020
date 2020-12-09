use regex::Regex;
use std::io::{stdin, Read};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<String>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    Ok(buffer
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().to_string())
        .collect())
}

fn main() -> Result<()> {
    let lines = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    let code = lines.iter()
        .map(String::as_str)
        .map(From::from)
        .collect();
    part1(&code);
    println!();
    part2(&code);

    Ok(())
}

fn part1(code: &Vec<OpCode>) {
    let mut run = Run::new(code.clone());
    let acc = run.run_without_revisit();
    println!("Part 1");
    println!("======");
    println!("Accumulator value: {}", acc);
}

fn part2(code: &Vec<OpCode>) {
    println!("Part 2");
    println!("======");

    let mut run = Run::new(code.clone());
    let mut flip_ip = 0; // address that we're flipping the opcode of
    let acc = loop {
        // if this is an acc opcode, don't bother flipping it because it is not affected
        if let OpCode::Acc(_) = run.code[flip_ip] {
            flip_ip += 1;
            continue;
        }

        // try this IP
        run.code[flip_ip].flip();
        let acc = run.run_without_revisit();

        // if we reached the end, this is the correct IP
        if run.ip == run.code.len() {
            break acc;
        }

        // otherwise, unflip the last IP and try again
        run.code[flip_ip].flip();
        flip_ip += 1;
    };

    println!("Accumulator value: {}", acc);
}

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl OpCode {
    /// Flips this opcode for the purposes of determining a corrupted program.
    ///
    /// A single Jmp or Nop may have been corrupted in the supplied program. This function flips
    /// an instruction for the purposes of determining the issue.
    fn flip(&mut self) {
        match *self {
            OpCode::Jmp(amt) => { *self = OpCode::Nop(amt); }
            OpCode::Nop(amt) => { *self = OpCode::Jmp(amt); }
            OpCode::Acc(_) => { /* no-op */ }
        }
    }
}

impl From<&str> for OpCode {
    fn from(other: &str) -> Self {
        lazy_static::lazy_static! {
            static ref OP_RE: Regex = Regex::new(r"^(nop|acc|jmp) ([+-]\d+)$").unwrap();
        };
        let caps = OP_RE.captures(other).expect("invalid opcode");
        let op = caps.get(1).unwrap().as_str();
        let arg: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
        match op {
            "nop" => OpCode::Nop(arg),
            "acc" => OpCode::Acc(arg),
            "jmp" => OpCode::Jmp(arg),
            _ => unreachable!()
        }
    }
}

struct Run {
    hits: Vec<bool>,
    code: Vec<OpCode>,
    acc: i64,
    ip: usize,
}

impl Run {
    fn new(code: Vec<OpCode>) -> Self {
        Run {
            hits: vec!(false; code.len()),
            code,
            acc: 0,
            ip: 0,
        }
    }

    fn reset(&mut self) {
        self.hits = vec!(false; self.code.len());
        self.acc = 0;
        self.ip = 0;
    }

    /// Runs the code without revisiting an address.
    ///
    /// Returns the accumulator value before an instruction at an already-visited address is
    /// run.
    ///
    /// This is guaranteed to terminate.
    fn run_without_revisit(&mut self) -> i64 {
        self.reset();
        loop {
            // if we've reached the end of the program, exit
            if self.ip >= self.code.len() {
                break;
            }

            // if we've hit this address, exit the program
            if self.hits[self.ip] {
                break;
            }

            self.hits[self.ip] = true;
            let mut next_ip = self.ip + 1;

            match self.code[self.ip] {
                OpCode::Acc(amt) => { self.acc += amt; }
                OpCode::Jmp(amt) => { next_ip = (self.ip as i64 + amt) as usize; }
                OpCode::Nop(_) => { /* nop */ }
            }
            self.ip = next_ip;
        }
        self.acc
    }
}
