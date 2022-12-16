use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Elf {
    food: Vec<i32>,
}

impl Elf {
    fn new() -> Self {
        Self { food: Vec::new() }
    }

    fn add_food(&mut self, food: i32) {
        self.food.push(food)
    }

    fn total_calories(&self) -> i32 {
        self.food.iter().sum()
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elves = vec![Elf::new()];
        for line in lines {
            if let Ok(n) = line {
                if n.is_empty() {
                    elves.push(Elf::new());
                    continue;
                }
                let elf = elves.last_mut().unwrap();
                elf.add_food(n.parse::<i32>().unwrap());
            }
        }
        elves.sort_by(|x, y| x.total_calories().partial_cmp(&y.total_calories()).unwrap());
        println!("Elf carrying most calories: {}", most_calories(&elves));
        let top_three = &elves[0..2].iter().map(|e| e.total_calories()).sum::<i32>();
        let bottom_three = &elves[elves.len() - 3..].iter().map(|e| e.total_calories()).sum::<i32>();
        println!("Total of top 3 elves calories:  {}", top_three);
        println!("Total of bottom 3 elves calories:  {}", bottom_three);
    }
}

fn most_calories(elves: &[Elf]) -> i32 {
    return elves
        .iter()
        .max_by(|x, y| x.total_calories().cmp(&y.total_calories()))
        .unwrap()
        .total_calories();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
