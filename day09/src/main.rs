use std::io::{stdin, Read};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<usize>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    Ok(buffer
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().parse().unwrap())
        .collect())
}

const WINDOW_SIZE: usize = 25;

fn main() -> Result<()> {
    let nums = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    let invalid = part1(&nums);
    println!();
    part2(&nums, invalid);

    Ok(())
}

fn is_valid(num: usize, window: &[usize]) -> bool {
    for i in 0..window.len() {
        if window[i] > num {
            continue;
        }
        for j in i..window.len() {
            if window[j] > num {
                continue;
            }
            if window[i] + window[j] == num {
                return true;
            }
        }
    }
    false
}

fn part1(nums: &Vec<usize>) -> usize {
    println!("Part 1");
    println!("======");
    for (num, window) in nums.as_slice().windows(WINDOW_SIZE + 1).map(|w| (w[WINDOW_SIZE], &w[0..WINDOW_SIZE])) {
        // check all possible sums for a number
        if !is_valid(num, window) {
            println!("{} is invalid", num);
            return num;
        }
    }
    unreachable!()
}

fn part2(nums: &Vec<usize>, invalid: usize) {
    println!("Part 2");
    println!("======");
    for len in 2..nums.len() {
        // try every window of length nums.len() .. 1
        for window in nums.as_slice().windows(len) {
            let sum = window.iter().sum::<usize>();
            if sum == invalid {
                println!("{:?} = {}", window, invalid);
                let lo = *window.iter().min().unwrap();
                let hi = *window.iter().max().unwrap();
                println!("{} + {} = {}", lo, hi, lo + hi);
            }
        }
    }
}
