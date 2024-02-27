advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    fn transpose(block: &Vec<String>) -> Vec<String> {
        let mut transposed: Vec<String> = Vec::new();
        for i in 0..block[0].len() {
            let mut line = String::new();
            for j in 0..block.len() {
                line.push(block[j].chars().nth(i).unwrap());
            }
            transposed.push(line);
        }
        transposed
    }

    fn solve_for_block(block: Vec<String>) -> Option<u32> {
        for i in 1..block.len() {
            if i * 2 > block.len() {
                break;
            }
            let left = block[..i].to_vec();
            let mut right = block[i..i * 2].to_vec();
            right.reverse();
            // println!("{:?} {:?}", left, right);
            if left == right {
                // println!("Found a mirror at index {}", i);
                return Some(i as u32);
            }
        }
        let rev_block: Vec<String> = block.into_iter().rev().collect();
        for i in 1..rev_block.len() {
            if i * 2 > rev_block.len() {
                break;
            }
            let left = rev_block[..i].to_vec();
            let mut right = rev_block[i..i * 2].to_vec();
            right.reverse();
            // println!("{:?} {:?}", left, right);
            if left == right {
                // println!("Found a mirror at index {}", (rev_block.len() - i));
                return Some((rev_block.len() - i) as u32);
            }
        }

        None
    }
    let mut block: Vec<String> = Vec::new();
    let mut lines = input.lines();
    let mut solution: u32 = 0;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            // println!("Block: {:?}", block);
            if let Some(s) = solve_for_block(block.clone()) {
                solution += s * 100;
                block.clear();
                continue;
            }
            if let Some(s) = solve_for_block(transpose(&block)) {
                solution += s;
                block.clear();
                continue;
            }
        }
        block.push(line.to_string());
    }
    if !block.is_empty() {
        // println!("Block: {:?}", block);
        if let Some(s) = solve_for_block(block) {
            solution += s;
        }
    }

    Some(solution)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
