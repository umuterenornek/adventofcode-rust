advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let row_count = input.lines().count();
    let mut last_empty_row: Vec<usize>;
    for (i, line) in input.lines().enumerate() {
        last_empty_row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                last_empty_row.push(j);
            }
        }
    }
    None
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
