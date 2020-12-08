use std::collections::HashSet;
use std::io::{stdin, Read};
use std::iter::FromIterator;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<String>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    // split \n\n because that's how groups are split up
    Ok(buffer
        .split("\n\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().to_string())
        .collect())
}

fn main() -> Result<()> {
    // Read lines from stdin
    let groups = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    println!("Got {} groups", groups.len());
    part1(&groups);
    println!();
    part2(&groups);
    Ok(())
}

fn part1(groups: &Vec<String>) {
    let mut count = 0;
    for group in groups.iter() {
        let answers: HashSet<char> =
            HashSet::from_iter(group.chars().filter(|c| c.is_alphabetic()));
        count += answers.len();
    }
    println!("Part 1");
    println!("======");
    println!("Sum: {}", count);
}

fn part2(groups: &Vec<String>) {
    let mut count = 0;
    for group in groups.iter() {

        // get all answers that the group answered
        let mut answer_space: HashSet<char> =
            HashSet::from_iter(group.chars().filter(|c| c.is_alphabetic()));
        for answers in group.split('\n') {
            let answer_set = HashSet::from_iter(answers.chars());
            answer_space = answer_space.intersection(&answer_set).copied().collect();
        }
        count += answer_space.len();
    }

    println!("Part 2");
    println!("======");
    println!("Sum: {}", count);
}
