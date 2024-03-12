advent_of_code::solution!(8);
use num::integer::lcm;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut directions = Vec::new();
    let mut desert_map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            for c in line.chars() {
                directions.push(c);
            }
        } else if !line.is_empty() {
            let line = line
                .split(" = ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();
            let key = line[0];
            let values_unparsed = line[1].split(", ").collect::<Vec<&str>>();
            let values = (
                &values_unparsed[0][1..],
                &values_unparsed[1][..values_unparsed[1].len() - 1],
            );
            desert_map.insert(key, values);
        }
    }
    let mut step_count = 0;
    let mut current_node = "AAA";
    let mut direction_index = 0;
    let mut direction = directions[direction_index];
    // println!("{}", directions.len());
    while current_node != "ZZZ" {
        step_count += 1;
        let (left_node, right_node) = *desert_map.get(current_node).unwrap();

        if direction == 'L' {
            current_node = left_node;
        } else {
            current_node = right_node;
        }

        direction_index = (direction_index + 1) % directions.len();
        direction = directions[direction_index];
    }

    Some(step_count)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut directions = Vec::new();
    let mut desert_map = HashMap::new();
    let mut start_nodes = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            for c in line.chars() {
                directions.push(c);
            }
        } else if !line.is_empty() {
            let line = line
                .split(" = ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();
            let key = line[0];
            if key.ends_with("A") {
                start_nodes.push(key);
            }
            let values_unparsed = line[1].split(", ").collect::<Vec<&str>>();
            let values = (
                &values_unparsed[0][1..],
                &values_unparsed[1][..values_unparsed[1].len() - 1],
            );
            desert_map.insert(key, values);
        }
    }
    let mut step_count = 0;
    let mut visited_node_maps: Vec<HashMap<&str, (u32, usize)>> = Vec::new();
    for i in 0..start_nodes.len() {
        visited_node_maps.push(HashMap::new());
        visited_node_maps
            .last_mut()
            .unwrap()
            .insert(start_nodes[i], (0, 0));
    }
    let mut end_node_steps_tmp: Vec<Vec<u32>> = Vec::new();
    let mut end_node_steps: Vec<i128> = Vec::new();
    for _ in 0..start_nodes.len() {
        end_node_steps_tmp.push(Vec::new());
    }
    let mut current_nodes = start_nodes.clone();
    let mut direction_index = 0;
    let mut direction = directions[direction_index];
    let mut non_end_node_found = true;
    while non_end_node_found && current_nodes.len() > 0 {
        step_count += 1;
        non_end_node_found = false;
        let mut i = 0;
        while i < current_nodes.len() {
            let (left_node, right_node) = *desert_map.get(current_nodes[i]).unwrap();

            if direction == 'L' {
                current_nodes[i] = left_node;
            } else {
                current_nodes[i] = right_node;
            }

            if !current_nodes[i].ends_with("Z") {
                non_end_node_found = true;
            } else {
                end_node_steps.push(step_count);
                current_nodes.remove(i);
                continue;
            }

            i += 1;
        }

        direction_index = (direction_index + 1) % directions.len();
        direction = directions[direction_index];
    }
    // println!("{:?}", end_node_steps);

    Some(end_node_steps.iter().fold(1, |acc, a| lcm(acc, *a)))
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
