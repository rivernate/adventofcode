mod file_utils;
mod part;

use file_utils::read_lines;
use part::Part;

fn main() {
    if let Ok(input_lines) = read_lines("./input.txt") {
        let grid = lines_to_grid(input_lines.filter_map(Result::ok).collect());
        find_parts(grid);
    } else {
        println!("Error reading input file");
    }
}

fn find_parts(grid: Vec<Vec<char>>) {
    grid.iter().enumerate().for_each(|(y, line)| {
        let part: Option<Part> = None;
        line.iter()
            .enumerate()
            .for_each(|(x, c)| match c.to_digit(10) {
                Some(d) => {
                    if let Some(p) = part {
                    } else {
                        let p = Part {
                            number: d,
                            start_coord: (x as i32, y as i32),
                            end_coord: (x as i32, y as i32),
                        };
                    };
                }
                _ => (),
            });
    });
}

fn lines_to_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    lines.iter().for_each(|line| {
        grid.push(line.chars().collect::<Vec<char>>());
    });
    grid
}
