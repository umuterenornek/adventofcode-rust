advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    fn calc_recursive(
        condition: String,
        damaged_sizes: &Vec<u32>,
        last_char_was_dot: bool,
        is_new_damaged_size_index: bool,
        solution: &str,
    ) -> u32 {
        // println!("{} {:?} {} {}", condition, damaged_sizes, last_char_was_dot, is_new_damaged_size_index);
        let mut damaged_sizes_reduced = damaged_sizes.clone();
        let mut reduced_is_new_damaged_size_index = false;
        if !damaged_sizes_reduced.is_empty() {
            if damaged_sizes.len() == 1 && damaged_sizes[0] > condition.len() as u32 {
                // println!("-----------------");
                return 0;
            }
            damaged_sizes_reduced[0] -= 1;
            if damaged_sizes_reduced[0] == 0 {
                damaged_sizes_reduced.remove(0);
                reduced_is_new_damaged_size_index = true;
            }
        }
        for c in condition.chars() {
            match c {
                '#' => {
                    if damaged_sizes.is_empty() {
                        // println!("-----------------");
                        return 0;
                    }
                    if !last_char_was_dot && is_new_damaged_size_index {
                        // println!("-----------------");
                        return 0;
                    }
                    return calc_recursive(
                        format!("{}", &condition[1..condition.len()]),
                        &damaged_sizes_reduced,
                        false,
                        reduced_is_new_damaged_size_index,
                        &format!("{}#", solution),
                    );
                }
                '?' => {
                    if damaged_sizes.is_empty() {
                        return calc_recursive(
                            format!("{}", &condition[1..condition.len()]),
                            damaged_sizes,
                            true,
                            false,
                            &format!("{}.", solution),
                        );
                    }
                    if last_char_was_dot && is_new_damaged_size_index {
                        return calc_recursive(
                            format!("{}", &condition[1..condition.len()]),
                            &damaged_sizes_reduced,
                            false,
                            reduced_is_new_damaged_size_index,
                            &format!("{}#", solution),
                        ) + calc_recursive(
                            format!("{}", &condition[1..condition.len()]),
                            &damaged_sizes,
                            true,
                            is_new_damaged_size_index,
                            &format!("{}.", solution),
                        );
                    }
                    if !last_char_was_dot && !is_new_damaged_size_index {
                        return calc_recursive(
                            format!("{}", &condition[1..condition.len()]),
                            &damaged_sizes_reduced,
                            false,
                            reduced_is_new_damaged_size_index,
                            &format!("{}#", solution),
                        );
                    }
                    return calc_recursive(
                        format!("{}", &condition[1..condition.len()]),
                        &damaged_sizes,
                        true,
                        is_new_damaged_size_index,
                        &format!("{}.", solution),
                    );
                }
                _ => {
                    if !is_new_damaged_size_index {
                        // println!("-----------------");
                        return 0;
                    }
                    return calc_recursive(
                        format!("{}", &condition[1..condition.len()]),
                        &damaged_sizes,
                        true,
                        true,
                        &format!("{}.", solution),
                    );
                }
            }
        }
        if damaged_sizes.len() == 0 {
            println!("Found match: {}", solution);
            return 1;
        }
        // println!("-----------------");
        0
    }

    fn calc_line(line: &str) -> u32 {
        let line = line.trim();
        let mut line = line.split_whitespace();
        let condition = line.next().unwrap();
        let damaged_sizes = line
            .next()
            .unwrap()
            .split(",")
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        println!("------------ {} {:?}", condition, damaged_sizes);
        calc_recursive(condition.to_string(), &damaged_sizes, true, true, "")
    }

    let mut result = 0;
    for line in input.lines() {
        let line_result = calc_line(line);
        result += line_result;
        println!("{}: {}", line, line_result);
        println!("-----------------");
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
