advent_of_code::solution!(4);
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut points = 0;
    for line in input.lines() {
        let line_split =
            line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" | ")
                .collect::<Vec<&str>>();

        let winning_numbers: HashSet<u32> =
            line_split[0]
                .trim()
                .split_terminator(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

        let ticket_numbers: Vec<u32> =
            line_split[1]
                .trim()
                .split_terminator(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

        let mut game_points: u32 = 0;
        for number in ticket_numbers {
            if winning_numbers.contains(&number) {
                if game_points == 0 {
                    game_points = 1;
                } else {
                    game_points *= 2;
                }
            }
        }
        points += game_points;
    }

    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn parse_lines(contents: &str) -> Vec<(HashSet<u32>, Vec<u32>)> {
        let mut lines: Vec<(HashSet<u32>, Vec<u32>)> = Vec::new();
        for line in contents.lines() {
            let line_split =
                line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(" | ")
                    .collect::<Vec<&str>>();

            let winning_numbers: HashSet<u32> =
                line_split[0]
                    .trim()
                    .split_terminator(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();

            let ticket_numbers: Vec<u32> =
                line_split[1]
                    .trim()
                    .split_terminator(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();

            lines.push((winning_numbers, ticket_numbers));
        }

        return lines;
    }
    let lines = &parse_lines(input);
    let mut card_map: HashMap<usize, u32> = HashMap::new();
    for (game, line) in lines.iter().enumerate() {
        let mut points: u32 = 0;
        let winning_numbers = &line.0;
        let ticket_numbers = &line.1;
        for number in ticket_numbers {
            if winning_numbers.contains(&number) {
                points += 1;
            }
        }

        let card_count =
            card_map
                .entry(game+1)
                .and_modify(|x| *x += 1)
                .or_insert(1)
                .clone();

        for i in game+2..game+2+points as usize {
            card_map
                .entry(i)
                .and_modify(|x| *x += card_count)
                .or_insert(card_count);
        }
    }

    let sum_cards = card_map
        .values()
        .fold(0, |acc, &v| acc + v);

    Some(sum_cards)
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
