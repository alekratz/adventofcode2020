use std::io::{stdin, Read};

type Error = Box<dyn std::error::Error>;
type Result<T, E=Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<String>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    Ok(buffer.split('\n').map(ToString::to_string).collect())
}

fn main() -> Result<()> {
    // Read lines from stdin
    let lines = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    // Convert lines to integers
    let nums: Vec<i64> = lines.into_iter()
        .filter_map(|line| if line.is_empty() {
            None
        } else {
            Some(line.parse::<i64>())
        })
        .collect::<Result<Vec<_>, _>>()?;

    part1(&nums)?;
    println!();
    part2(&nums)?;

    Ok(())
}

fn part1(nums: &Vec<i64>) -> Result<()> {
    println!("Part 1");
    println!("======");
    // Brute-force search
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let a = nums[i];
            let b = nums[j];
            if a + b == 2020 {
                println!("{} + {} = {}", a, b, a + b);
                println!("{} * {} = {}", a, b, a * b);
                return Ok(());
            }
        }
    }
    Err("Could not find a pair that sums to 2020.".into())
}

fn part2(nums: &Vec<i64>) -> Result<()> {
    println!("Part 2");
    println!("======");

    // Brute-force search
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            for k in (j + 1)..nums.len() {
                let a = nums[i];
                let b = nums[j];
                let c = nums[k];
                if a + b + c == 2020 {
                    println!("{} + {} + {} = {}", a, b, c, a + b + c);
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    return Ok(());
                }
            }
        }
    }
    Err("Could not find a pair that sums to 2020.".into())
}
