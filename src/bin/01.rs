advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_l: Vec<u32> = Vec::new();
    let mut list_r: Vec<u32> = Vec::new();
    for line in input.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        list_l.push(split_line[0].parse::<u32>().unwrap());
        list_r.push(split_line[1].parse::<u32>().unwrap());
    }
    list_l.sort();
    list_r.sort();
    let mut sum = 0;
    for (i, num) in list_l.iter().enumerate() {
        sum += num.abs_diff(list_r[i]);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_l: Vec<u32> = Vec::new();
    let mut list_r: Vec<u32> = Vec::new();
    for line in input.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        list_l.push(split_line[0].parse::<u32>().unwrap());
        list_r.push(split_line[1].parse::<u32>().unwrap());
    }
    list_l.sort();
    list_r.sort();
    let mut sum = 0;
    for num in list_l.iter() {
        let list_r_iter = list_r.iter();
        sum += list_r_iter.filter(|&x| x == num).count() as u32 * num;
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
