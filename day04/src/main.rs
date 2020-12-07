use std::collections::HashMap;
use std::io::{stdin, Read};
use regex::{Regex, RegexSet};

type Error = Box<dyn std::error::Error>;
type Result<T, E=Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<String>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    // split \n\n because that's how passports are split up
    Ok(buffer.split("\n\n").filter(|s| s.len() > 0).map(ToString::to_string).collect())
}

fn main() -> Result<()> {
    // Read lines from stdin
    let inputs = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    println!("Got {} passports", inputs.len());
    part1(&inputs);
    println!();
    part2(&inputs);

    Ok(())
}

fn part1(inputs: &Vec<String>) {
    const REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let pat = Regex::new(r"(\S+):(\S+)").unwrap();

    let mut valid = 0;
    for ident in inputs.iter() {
        let matches: HashMap<_, _> = pat.captures_iter(ident)
            .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
            .collect();
        let mut is_valid = true;
        for req in REQUIRED {
            if !matches.contains_key(req) {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            valid += 1;
        }
    }

    println!("Part 1");
    println!("======");
    println!("{} valid passports in the input set", valid);
}

fn part2(inputs: &Vec<String>) {
    let pat = RegexSet::new(&[
        r"(cid):(\S+)\b",
        r"(byr):(19[2-9][0-9]|200[0-2])\b",
        r"(iyr):(20(1[0-9]|20))\b",
        r"(eyr):(20(2[0-9]|30))\b",
        r"(hgt):((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)\b",
        r"(hcl):(\#[0-9a-f]{6})\b",
        r"(ecl):(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"(pid):([0-9]{9})\b",
    ]).unwrap();

    let mut valid = 0;
    for ident in inputs.iter() {
        let matches: Vec<_> = pat.matches(ident).into_iter().collect();
        if matches.len() == 8 {
            // If there are 8 matches, then this passport is certainly valid
            valid += 1;
        } else if matches.len() == 7 && !matches.contains(&0) {
            // If there are 7 matches, then make sure that the only one missing is the "cid"
            // member, which is item 0
            valid += 1;
        }
        // otherwise, it's not valid
    }

    println!("Part 2");
    println!("======");
    println!("{} valid passports in the input set", valid);
}
