advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let report = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut is_safe = true;
        let mut is_increasing = true;
        if report[0] > report[1] {
            is_increasing = false;
        }
        for p in report.windows(2) {
            match p[0].cmp(&p[1]) {
                std::cmp::Ordering::Less => {
                    if !is_increasing {
                        is_safe = false;
                        break;
                    }
                    let dif = p[1] - p[0];
                    if dif <= 3 && dif >= 1 {
                        continue;
                    }
                    is_safe = false;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    if is_increasing {
                        is_safe = false;
                        break;
                    }
                    let dif = p[0] - p[1];
                    if dif <= 3 && dif >= 1 {
                        continue;
                    }
                    is_safe = false;
                    break;
                }
                _ => {
                    is_safe = false;
                    break;
                }
            }
        }
        if is_safe {
            sum += 1;
        }
    }
    Some(sum)
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
