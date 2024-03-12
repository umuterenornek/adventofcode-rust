advent_of_code::solution!(9);

fn find_next_num(line: &str, rev: bool) -> i64 {
    let mut non_zero_found = true;
    let mut history: Vec<i64>;
    if rev {
        history = line
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .rev()
            .collect();
    } else {
        history = line
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
    }

    let mut index_limit = history.len() - 1;

    while non_zero_found == true {
        index_limit -= 1;
        non_zero_found = false;
        let history_clone = history.clone();
        let history_clone_windows = history_clone.windows(2);

        for (i, w) in history_clone_windows.enumerate() {
            if i > index_limit {
                break;
            }
            history[i] = w[1] - w[0];
            if history[i] != 0 {
                non_zero_found = true;
            }
        }
        // println!("{:?}", history);
        // println!("------------------");
    }

    let next_num = history.iter().sum();
    // println!("Next num: {}", next_num);

    next_num
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut sum = 0;
    for line in input.lines() {
        let result = find_next_num(line, false);
        sum += result;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut sum = 0;
    for line in input.lines() {
        let result = find_next_num(line, true);
        sum += result;
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
