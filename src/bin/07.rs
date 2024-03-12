advent_of_code::solution!(7);
use std::{collections::HashMap, iter::zip};

fn parse_line(line: &str) -> (&str, u32) {
    let split_line: Vec<&str> = line.split_whitespace().filter(|s| !s.is_empty()).collect();

    let hand = split_line[0];
    let bid = split_line[1].parse::<u32>().unwrap();

    (hand, bid)
}

fn compare_hand_value(
    hand1: &str,
    hand2: &str,
    card_values: &HashMap<char, u32>,
) -> std::cmp::Ordering {
    for (a, b) in zip(hand1.chars(), hand2.chars()) {
        if a != b {
            return card_values
                .get(&a)
                .unwrap()
                .cmp(card_values.get(&b).unwrap());
        }
    }
    std::cmp::Ordering::Equal
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut card_values = HashMap::new();
    card_values.insert('A', 14);
    card_values.insert('K', 13);
    card_values.insert('Q', 12);
    card_values.insert('J', 11);
    card_values.insert('T', 10);
    card_values.insert('9', 9);
    card_values.insert('8', 8);
    card_values.insert('7', 7);
    card_values.insert('6', 6);
    card_values.insert('5', 5);
    card_values.insert('4', 4);
    card_values.insert('3', 3);
    card_values.insert('2', 2);
    let mut earnings = 0;
    let mut hand_value_mapping: HashMap<&str, (u32, u32)> = HashMap::new();
    for line in input.lines() {
        let mut hand_map: HashMap<char, u32> = HashMap::new();
        let (hand, bid) = parse_line(line);

        let mut hand_cards_value: u32 = 0;
        for c in hand.chars() {
            hand_cards_value += card_values.get(&c).unwrap();
            hand_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let hand_unique_count: Vec<u32> = hand_map.values().cloned().collect();

        let mut hand_type_value = 0;
        match hand_unique_count.len() {
            1 => {
                hand_type_value = 7;
            }
            2 => match hand_unique_count.iter().max().unwrap() {
                4 => {
                    hand_type_value = 6;
                }
                3 => {
                    hand_type_value = 5;
                }
                _ => {}
            },
            3 => match hand_unique_count.iter().max().unwrap() {
                3 => {
                    hand_type_value = 4;
                }
                2 => {
                    hand_type_value = 3;
                }
                _ => {}
            },
            4 => {
                hand_type_value = 2;
            }
            5 => {
                hand_type_value = 1;
            }
            _ => {}
        }

        hand_value_mapping.insert(hand, (hand_type_value, bid));
    }

    let mut sorted_hand_values: Vec<(&&str, &(u32, u32))> = hand_value_mapping.iter().collect();
    sorted_hand_values.sort_by(|a, b| {
        a.1 .0
            .cmp(&b.1 .0)
            .then_with(|| compare_hand_value(a.0, b.0, &card_values))
    });

    for (i, hand) in sorted_hand_values.iter().enumerate() {
        earnings += hand.1 .1 * (i as u32 + 1);
    }

    Some(earnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_values = HashMap::new();
    card_values.insert('A', 14);
    card_values.insert('K', 13);
    card_values.insert('Q', 12);
    card_values.insert('J', 11);
    card_values.insert('T', 10);
    card_values.insert('9', 9);
    card_values.insert('8', 8);
    card_values.insert('7', 7);
    card_values.insert('6', 6);
    card_values.insert('5', 5);
    card_values.insert('4', 4);
    card_values.insert('3', 3);
    card_values.insert('2', 2);
    let mut earnings = 0;
    let mut hand_value_mapping: HashMap<&str, (u32, u32)> = HashMap::new();
    for line in input.lines() {
        let mut hand_map: HashMap<char, u32> = HashMap::new();
        let (hand, bid) = parse_line(line);

        let mut hand_cards_value: u32 = 0;
        for c in hand.chars() {
            hand_cards_value += card_values.get(&c).unwrap();
            hand_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let hand_j_count = hand_map.get(&'J').unwrap_or(&0).clone();

        hand_map.remove(&'J');

        if hand_map.is_empty() {
            hand_value_mapping.insert(hand, (7, bid));
            continue;
        }

        let key_with_max_unique_count = hand_map.iter().max_by_key(|(_, v)| *v).unwrap().0;

        hand_map
            .entry(*key_with_max_unique_count)
            .and_modify(|v| *v += hand_j_count);

        let hand_unique_count: Vec<u32> = hand_map.values().cloned().collect();

        let mut hand_type_value = 0;
        match hand_unique_count.len() {
            1 => {
                hand_type_value = 7;
            }
            2 => match hand_unique_count.iter().max().unwrap() {
                4 => {
                    hand_type_value = 6;
                }
                3 => {
                    hand_type_value = 5;
                }
                _ => {}
            },
            3 => match hand_unique_count.iter().max().unwrap() {
                3 => {
                    hand_type_value = 4;
                }
                2 => {
                    hand_type_value = 3;
                }
                _ => {}
            },
            4 => {
                hand_type_value = 2;
            }
            5 => {
                hand_type_value = 1;
            }
            _ => {}
        }

        hand_value_mapping.insert(hand, (hand_type_value, bid));
    }

    let mut sorted_hand_values: Vec<(&&str, &(u32, u32))> = hand_value_mapping.iter().collect();
    sorted_hand_values.sort_by(|a, b| {
        a.1 .0
            .cmp(&b.1 .0)
            .then_with(|| compare_hand_value(a.0, b.0, &card_values))
    });

    for (i, hand) in sorted_hand_values.iter().enumerate() {
        earnings += hand.1 .1 * (i as u32 + 1);
    }

    Some(earnings)
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
