use std::cmp;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Color {
    Blue,
    Green,
    Red,
}

pub struct Game {
    pub id: u32,
    pub draws: Vec<HashMap<Color, u32>>,
}

impl Game {
    pub fn enough_gems(&self, blue: u32, green: u32, red: u32) -> bool {
        let (max_blue, max_green, max_red) = self.max_color_counts();
        blue >= max_blue && green >= max_green && red >= max_red
    }

    pub fn max_color_counts(&self) -> (u32, u32, u32) {
        let mut max_colors = HashMap::<Color, u32>::new();
        max_colors.insert(Color::Blue, 0);
        max_colors.insert(Color::Green, 0);
        max_colors.insert(Color::Red, 0);

        self.draws.iter().for_each(|draw| {
            for (color, count) in draw {
                *max_colors.entry(color.clone()).or_insert(0) =
                    cmp::max(*max_colors.get(color).unwrap(), *count);
            }
        });
        (
            *max_colors.get(&Color::Blue).unwrap(),
            *max_colors.get(&Color::Green).unwrap(),
            *max_colors.get(&Color::Red).unwrap(),
        )
    }
}
