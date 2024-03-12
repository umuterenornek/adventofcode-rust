advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    fn get_first_substr(digit_map: &HashMap<&str, u32>, line: &str) -> (Option<usize>, u32) {
        let mut first_substr_index = None;
        let mut digit: u32 = 0;
        for (key, value) in digit_map.iter() {
            let index = line.find(key);
            if index.is_some() {
                if first_substr_index.is_some() {
                    if index < first_substr_index {
                        first_substr_index = index;
                        digit = value.clone();
                    }
                } else {
                    first_substr_index = index;
                    digit = value.clone();
                }
            }
        }

        return (first_substr_index, digit);
    }
    fn get_first_digit(digit_map: &HashMap<&str, u32>, line: &str) -> u32 {
        let mut first: u32 = 0;
        let first_substr = get_first_substr(&digit_map, line);
        let first_substr_index = first_substr.0;
        let first_substr_digit = first_substr.1;
        let first_digit_index = line
            .chars()
            .position(|c| c.is_numeric());

        if first_digit_index.is_some() {
            first = line
                .chars()
                .nth(first_digit_index.unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap();
        }

        if first_substr_index.is_some() {
            if first_digit_index.is_some() && first_substr_index.unwrap() > first_digit_index.unwrap(){
                return first;
            } else {
                first = first_substr_digit;
            }
        }

        return first;
    }
    fn find_last_substr(digit_map: &HashMap<&str, u32>, line: &str) -> (Option<usize>, u32) {
        let mut last_substr_index = None;
        let mut digit: u32 = 0;
        for (key, value) in digit_map.iter() {
            let index = line.rfind(key);
            if index.is_some() {
                if last_substr_index.is_some() {
                    if index > last_substr_index {
                        last_substr_index = index;
                        digit = value.clone();
                    }
                } else {
                    last_substr_index = index;
                    digit = value.clone();
                }
            }
        }

        return (last_substr_index, digit);
    }
    fn get_last_digit(digit_map: &HashMap<&str, u32>, line: &str) -> u32 {
        let mut last: u32 = 0;
        let last_substr = find_last_substr(&digit_map, line);
        let last_substr_index = last_substr.0;
        let last_substr_digit = last_substr.1;
        let last_digit_index = line
            .chars()
            .rev()
            .position(|c| c.is_numeric())
            .map(|i| line.len() - i - 1);

        if last_digit_index.is_some() {
            last = line
                .chars()
                .nth(last_digit_index.unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap();
        }

        if last_substr_index.is_some() {
            if last_digit_index.is_some() && last_substr_index.unwrap() < last_digit_index.unwrap(){
                return last;
            } else {
                last = last_substr_digit;
            }
        }

        return last;
    }
    let mut sum = 0;
    let mut digit_mapping: HashMap<&str, u32> = HashMap::new();
    digit_mapping.insert("one", 1);
    digit_mapping.insert("two", 2);
    digit_mapping.insert("three", 3);
    digit_mapping.insert("four", 4);
    digit_mapping.insert("five", 5);
    digit_mapping.insert("six", 6);
    digit_mapping.insert("seven", 7);
    digit_mapping.insert("eight", 8);
    digit_mapping.insert("nine", 9);

    for line in input.lines() {
        let first = get_first_digit(&digit_mapping, line);
        if first == 0 {
            // throw error
            println!("Error, first index not found in line: {}", line)
        }
        let last = get_last_digit(&digit_mapping, line);
        if last == 0 {
            // throw error
            println!("Error, last index not found in line: {}", line)
        }

        // Create number from first and last digit
        let cal_value = first * 10 + last;
        sum += cal_value;
    }

    Some(sum)
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
