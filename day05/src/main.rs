use std::io::{stdin, Read};

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

fn find_row(text: &str, lo: usize, hi: usize) -> usize {
    let avg = (lo + hi) / 2;
    match text.chars().next() {
        None => avg,
        Some('F') => find_row(&text[1..], lo, avg),
        Some('B') => find_row(&text[1..], avg, hi),
        _ => unreachable!(),
    }
}

fn find_col(text: &str, lo: usize, hi: usize) -> usize {
    let avg = (lo + hi) / 2;
    match text.chars().next() {
        None => avg,
        Some('L') => find_col(&text[1..], lo, avg),
        Some('R') => find_col(&text[1..], avg, hi),
        _ => unreachable!(),
    }
}

fn part1(lines: &Vec<String>) {
    let mut max = 0;
    for id in lines.iter() {
        let row = find_row(&id[0..7], 0, 128);
        let col = find_col(&id[7..], 0, 8);
        let seat = row * 8 + col;
        if seat > max {
            max = seat;
        }
    }

    println!("Part 1");
    println!("======");
    println!("Max seat ID: {}", max);
}

fn part2(lines: &Vec<String>) {
    let mut occupied = vec!(vec!(false; 8); 128);
    let mut max = 0;
    let mut min = usize::MAX;
    for id in lines.iter() {
        let row = find_row(&id[0..7], 0, 128);
        let col = find_col(&id[7..], 0, 8);
        occupied[row][col] = true;
        let seat = row * 8 + col;
        if seat > max {
            max = seat;
        }
        if seat < min {
            min = seat;
        }
    }


    println!("Part 2");
    println!("======");
    let max_row = max / 8;
    let min_row = min / 8;
    for row in min_row + 1 .. max_row + 1 {
        for col in 0..8 {
            if !occupied[row][col] {
                let id = row * 8 + col;
                println!("Seat ID: {}", id);
            }
        }
    }
}

// some sanity checks ...

#[test]
fn test_find_row() {
    // make sure this works how I actually think it should lol
    assert_eq!(find_row("FBFBBFF", 0, 128), 44);
    assert_eq!(find_row("BFFFBBF", 0, 128), 70);
    assert_eq!(find_row("FFFBBBF", 0, 128), 14);
    assert_eq!(find_row("BBFFBBF", 0, 128), 102);
}

#[test]
fn test_find_col() {
    assert_eq!(find_col("RLR", 0, 8), 5);
    assert_eq!(find_col("RRR", 0, 8), 7);
    assert_eq!(find_col("RLL", 0, 8), 4);
}
