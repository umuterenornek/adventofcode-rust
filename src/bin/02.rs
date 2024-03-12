advent_of_code::solution!(2);
use std::collections::HashMap;

fn parse_games(contents: &str) -> Vec<HashMap<&str, u32>> {
    /* example contents:
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    */
    let mut games: Vec<HashMap<&str, u32>> = Vec::new();
    for line in contents.lines() {
        let mut game: HashMap<&str, u32> = HashMap::new();
        // split line into game number and game contents
        let sets: Vec<&str> = line.split(":").nth(1).unwrap().split(";").collect();
        for set in sets {
            // split set into color and number
            let color_and_number_group: Vec<&str> = set.trim().split(", ").collect();
            for color_and_number in color_and_number_group {
                let color_and_number_split: Vec<&str> = color_and_number.split(" ").collect();
                let color = color_and_number_split[1];
                let number = color_and_number_split[0].parse::<u32>().unwrap();
                if game.contains_key(color) && game.get(color).unwrap() > &number {
                    continue;
                }
                game.insert(color, number);
            }
        }
        games.push(game);
    }
    return games;
}

pub fn part_one(input: &str) -> Option<u32> {
    fn is_game_valid(game: &HashMap<&str, u32>, available_colors: &HashMap<&str, u32>) -> bool {
        for (color, number) in game {
            if available_colors.contains_key(color) && available_colors.get(color).unwrap() < &number {
                return false;
            }
        }
        return true;
    }
    let mut available_colors: HashMap<&str, u32> = HashMap::new();
    available_colors.insert("red", 12);
    available_colors.insert("green", 13);
    available_colors.insert("blue", 14);
    let games = parse_games(&input);

    let mut sum_of_ids = 0;
    for (i, game) in games.iter().enumerate() {
        if is_game_valid(game, &available_colors) {
            sum_of_ids += i+1;
        }
    }

    Some(sum_of_ids as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn calc_min_power_needed(game: &HashMap<&str, u32>) -> u32 {
        let mut min_power_needed = 0;
        for (_color, number) in game {
            if min_power_needed == 0 {
                min_power_needed = number.clone();
            } else {
                min_power_needed = min_power_needed * number.clone();
            }
        }
        return min_power_needed;
    }
    let games = parse_games(&input);

    let mut sum_of_power_needed = 0;
    for game in games {
        let min_power_needed = calc_min_power_needed(&game);
        sum_of_power_needed += min_power_needed;
    }

    Some(sum_of_power_needed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
