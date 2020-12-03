use std::collections::HashMap;
use std::io::{stdin, Read};
use regex::Regex;

type Error = Box<dyn std::error::Error>;
type Result<T, E=Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<String>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    Ok(buffer.split('\n').filter(|s| s.len() > 0).map(ToString::to_string).collect())
}

fn main() -> Result<()> {
    // Read lines from stdin
    let lines = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    part1(&lines);
    println!();
    part2(&lines);

    Ok(())
}

fn letter_count(letters: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in letters.chars() {
        let count = counts.entry(c)
            .or_insert(0);
        *count += 1;
    }
    counts
}

fn part1(lines: &Vec<String>) {
    println!("Part 1");
    println!("======");

    // lo, hi, letter, password
    let pat = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut valid = 0;
    let mut invalid = 0;
    for line in lines {
        let caps = pat.captures(line)
            .ok_or_else(|| format!("invalid line: {}", line))
            .unwrap();
        let lo = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let hi = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = caps.get(4).unwrap().as_str();

        let counts = letter_count(pass);
        match counts.get(&letter).copied() {
            Some(count) if count >= lo && count <= hi => valid += 1,
            _ => invalid += 1,
        }
    }
    println!("{} valid passwords and {} invalid passwords", valid, invalid);
}

fn part2(lines: &Vec<String>) {
    println!("Part 2");
    println!("======");

    // first, second, letter, password
    let pat = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut valid = 0;
    let mut invalid = 0;
    for line in lines {
        let caps = pat.captures(line)
            .ok_or_else(|| format!("invalid line: {}", line))
            .unwrap();
        let p1 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let p2 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = caps.get(4).unwrap().as_str();

        // -1 because they use base-1 indexing
        let c1 = pass.chars().nth(p1 - 1).unwrap();
        let c2 = pass.chars().nth(p2 - 1).unwrap();
        if (c1 == letter) ^ (c2 == letter) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("{} valid passwords and {} invalid passwords", valid, invalid);
}
