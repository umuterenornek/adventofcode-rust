advent_of_code::solution!(3);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    /* example contents:
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    */
    let mut part_numbers: Vec<u32> = Vec::new();
    let mut lines: Vec<String> = input
        .lines()
        .into_iter()
        .map(|line| format!(".{line}."))
        .collect();
    // add a line of dots to the top and bottom
    let top_bottom = ".".repeat(lines[0].len());
    lines.insert(0, top_bottom.clone());
    lines.push(top_bottom);

    for (i, line) in lines.iter().enumerate() {
        let mut is_parsing_number = false;
        let mut num = 0;
        let mut is_part_number = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if is_parsing_number {
                    num = num * 10 + c.to_digit(10).unwrap();
                } else {
                    num = c.to_digit(10).unwrap();
                    is_parsing_number = true;
                }
                for l in &lines[i-1..i+2] {
                    for k in l[j-1..j+2].chars() {
                        if k != '.' && !k.is_numeric() {
                            is_part_number = true;
                            break;
                        }
                    }
                    if is_part_number {
                        break;
                    }
                }
                continue;
            }
            if is_parsing_number && is_part_number {
                part_numbers.push(num);
            }
            is_parsing_number = false;
            is_part_number = false;
        }
    }
    Some(part_numbers.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    /* example contents:
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    */
    let mut gear_ratios: Vec<u32> = Vec::new();
    let mut lines: Vec<String> = input
        .lines()
        .into_iter()
        .map(|line| format!(".{line}."))
        .collect();
    // add a line of dots to the top and bottom
    let top_bottom = ".".repeat(lines[0].len());
    lines.insert(0, top_bottom.clone());
    lines.push(top_bottom);

    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let mut is_parsing_number = false;
        let mut num = 0;
        let mut is_part_number = false;
        let mut gear_index: (usize, usize) = (0, 0);
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if is_parsing_number {
                    num = num * 10 + c.to_digit(10).unwrap();
                } else {
                    num = c.to_digit(10).unwrap();
                    is_parsing_number = true;
                }
                for (n, l) in lines[i-1..i+2].iter().enumerate() {
                    for (m, k) in l[j-1..j+2].chars().enumerate() {
                        if k == '*' {
                            is_part_number = true;
                            // calculate index of * in lines
                            gear_index = (i-1+n, j-1+m);
                            break;
                        }
                    }
                    if is_part_number {
                        break;
                    }
                }
                continue;
            }
            if is_parsing_number && is_part_number {
                gear_map
                    .entry(gear_index)
                    .or_insert(Vec::new())
                    .push(num);
            }
            is_parsing_number = false;
            is_part_number = false;
        }
    }

    for v in gear_map.values() {
        if v.len() == 2 {
            gear_ratios.push(v[0] * v[1]);
        }
    }

    Some(gear_ratios.iter().sum::<u32>())
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
