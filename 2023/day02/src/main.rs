mod file_utils;
mod game;

use std::collections::HashMap;

use file_utils::read_lines;
use game::{Color, Game};

fn main() {
    let input_lines = match read_lines("./input.txt") {
        Ok(input_lines) => input_lines,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };
    let games = input_lines.into_iter()
        .filter_map(Result::ok)
        .map(|x| read_game(&x))
        .collect::<Result<Vec<_>, _>>();

    let games = match games {
        Ok(games) => games,
        Err(err) => {
            println!("Error parsing games data: {}", err);
            return;
        }
    };

    let (total_game_sum, total_gems_power) = calculate_results(games);
    print_results(total_game_sum, total_gems_power);
}

fn print_results(sum_games: u32, sum_gems: u32) {
    println!("Sum of possible games: {}", sum_games);
    println!("Sum of min gems power: {}", sum_gems);
}

fn calculate_results(games: Vec<Game>) -> (u32, u32) {
    games.iter().fold((0, 0), |(sum_games, sum_gems), game| {
        let winnable = game.enough_gems(14, 13, 12);
        let (blue, green, red) = game.max_color_counts();
        let sum_gems_updated = sum_gems + blue * green * red;

        if winnable {
            (sum_games + game.id, sum_gems_updated)
        } else {
            (sum_games, sum_gems_updated)
        }
    })
}

fn read_game(game: &str) -> Result<Game, String> {
    let mut game_parts = game.split(":");

    let game_id_str = game_parts.next().ok_or_else(|| "Game ID missing".to_string())?;

    let game_id = game_id_str
        .split_whitespace()
        .last()
        .ok_or_else(|| "Game ID missing".to_string())?
        .parse::<u32>()
        .map_err(|_| format!("Invalid game ID: {}", game_id_str))?;

    // Create a vector of Draw structs
    let draws_str = match game_parts.next() {
        Some(draws) => draws,
        None => return Err("Draws data missing or improperly formatted".to_string()),
    }.split(";");

    let draws = draws_str
        .map(read_bag_draw)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Game { id: game_id, draws })
}

fn read_bag_draw(bag_draw: &str) -> Result<HashMap<Color, u32>, String> {
    let mut draw = HashMap::new();

    for part in bag_draw.split(",").map(str::trim) {
        let mut parts = part.split_whitespace();
        let count = parts.next().ok_or_else(|| format!("Count missing in draw: {}", part))?.parse::<u32>()
            .map_err(|_| format!("Invalid count in draw: {}", part))?;

        let color_str = parts.next().ok_or_else(|| format!("Color missing in draw: {}", part))?;
        let color = match color_str {
            "blue" => Color::Blue,
            "green" => Color::Green,
            "red" => Color::Red,
            _ => return Err(format!("Invalid color in draw: {}", part)),
        };
        draw.insert(color, count);
    }
    Ok(draw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winnable_game() {
        let test_cases = vec![
            ("Game 5: 7 red, 7 green, 1 blue; 2 red, 1 green, 2 blue; 2 blue, 7 green; 7 red, 3 blue, 11 green", true),
        ];

        for (input, expected) in test_cases {
            let game = read_game(input).expect("Failed to parse game");
            let result = game.enough_gems(14, 13, 12);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn max_gems() {
        let test_cases = vec![
            ("Game 5: 7 red, 7 green, 1 blue; 2 red, 1 green, 2 blue; 2 blue, 7 green; 7 red, 3 blue, 11 green", (3, 11, 7)),
        ];
        for (input, expected) in test_cases {
            let game = read_game(input).expect("Failed to parse game");
            let (blue, green, red) = game.max_color_counts();
            let (expected_blue, expected_green, expected_red) = expected;
            assert_eq!(blue, expected_blue);
            assert_eq!(green, expected_green);
            assert_eq!(red, expected_red);
        }
    }

    #[test]
    fn min_gems() {
        let test_cases = vec![
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                (6, 2, 4),
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                (4, 3, 1),
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                (6, 13, 20),
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                (15, 3, 14),
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                (2, 3, 6),
            ),
        ];
        for (input, expected) in test_cases {
            let game = read_game(input).expect("Failed to parse game");
            let (blue, green, red) = game.max_color_counts();
            let (expected_blue, expected_green, expected_red) = expected;
            assert_eq!(blue, expected_blue, "Failed blue match on {}", input);
            assert_eq!(green, expected_green, "Failed green match on {}", input);
            assert_eq!(red, expected_red, "Failed red match on {}", input);
        }
    }
}
