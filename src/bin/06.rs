advent_of_code::solution!(6);

fn calculate_possibility_count(time: &i64, distance: &i64) -> i64 {
    // Equation to satisfy: distance > (time - button_time) * button_time
    // -button_time^2 + time * button_time - distance > 0
    // determine the roots of the equation
    // (-b +- sqrt(b^2 - 4ac)) / 2a

    let a = -1;
    let b = *time;
    let c = -*distance;
    let discriminant = b * b - 4 * a * c;
    let discr_root = (discriminant as f64).sqrt();

    let mut root1 = (-b as f64 + discr_root) / (2 * a) as f64;
    let mut root2 = (-b as f64 - discr_root) / (2 * a) as f64;

    if root1 > root2 {
        let temp = root1;
        root1 = root2;
        root2 = temp;
    }

    root1 = root1.ceil();
    root2 = root2.floor();

    let mut root_min = root1 as i64;
    let mut root_max = root2 as i64;

    // println!("Time: {}", time);
    // println!("Distance: {}", distance);
    // println!("Roots: {}, {}", root1, root2);
    if -root_min * root_min + time * root_min - distance == 0 {
        root_min += 1;
    }
    if -root_max * root_max + time * root_max - distance == 0 {
        root_max -= 1;
    }

    if discriminant < 0 {
        0
    } else if discriminant == 0 {
        1
    } else {
        if root_min > 0 && root_max > 0 {
            root_max - root_min + 1
        } else if root_min > 0 || root_max > 0 {
            1
        } else {
            0
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut solution = 1;
    let times =
        lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .filter(|x| !x.is_empty())
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
    let distances =
        lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .filter(|x| !x.is_empty())
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
    for (time, distance) in times.iter().zip(distances.iter()) {
        // println!("-----------------");
        let possibility_count = calculate_possibility_count(time, distance);
        // println!("Possibility count: {}", possibility_count);
        solution *= possibility_count;
    }

    Some(solution as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut solution = 1;
    let times =
        lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .filter(|x| !x.is_empty())
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
    let distances =
        lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .filter(|x| !x.is_empty())
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
    let time =
        times
        .concat()
        .parse::<i64>()
        .unwrap();
    let distance =
        distances
        .concat()
        .parse::<i64>()
        .unwrap();
    // println!("-----------------");
    let possibility_count = calculate_possibility_count(&time, &distance);
    // println!("Possibility count: {}", possibility_count);
    solution *= possibility_count;

    Some(solution as u32)
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
