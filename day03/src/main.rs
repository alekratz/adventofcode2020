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

    let grid: Vec<Vec<char>> = lines.into_iter()
        .map(|s| s.chars().collect())
        .collect();

    part1(&grid);
    println!();
    part2(&grid);

    Ok(())
}

fn count_hits(grid: &Vec<Vec<char>>, slope_x: usize, slope_y: usize) -> usize {
    let mut y = 0;
    let mut x = 0;
    let height = grid.len();
    let width = grid[0].len();
    let mut hits = 0;
    while y < height {
        if grid[y][x] == '#' {
            hits += 1;
        }
        y += slope_y;
        x = (x + slope_x) % width;
    }
    hits
}

fn part1(grid: &Vec<Vec<char>>) {
    println!("Part 1");
    println!("======");
    let hits = count_hits(grid, 3, 1);
    println!("Hit {} trees", hits);
}

fn part2(grid: &Vec<Vec<char>>) {
    println!("Part 2");
    println!("======");
    let hits_11 = count_hits(grid, 1, 1);
    let hits_31 = count_hits(grid, 3, 1);
    let hits_51 = count_hits(grid, 5, 1);
    let hits_71 = count_hits(grid, 7, 1);
    let hits_12 = count_hits(grid, 1, 2);

    println!("Right 1, down 1: {}", hits_11);
    println!("Right 3, down 1: {}", hits_31);
    println!("Right 5, down 1: {}", hits_51);
    println!("Right 7, down 1: {}", hits_71);
    println!("Right 1, down 2: {}", hits_12);
    println!("Product: {}", hits_11 * hits_31 * hits_51 * hits_71 * hits_12);
}
