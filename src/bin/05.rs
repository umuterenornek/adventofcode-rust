advent_of_code::solution!(5);

use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct RangeSet {
    ranges: Vec<Range<i64>>,
}

impl RangeSet {
    pub fn new(ranges: Vec<Range<i64>>) -> Self {
        RangeSet { ranges }
    }

    // union is inplace for performance reasons
    pub fn union(&mut self, other: Self) {
        match (self.ranges.is_empty(), other.ranges.is_empty()) {
            (true, true) => self.ranges.clear(),
            (true, false) => self.ranges = other.ranges,
            (false, true) => (),
            _ => {
                let mut new_ranges = self.ranges.clone();
                new_ranges.extend(other.ranges);
                new_ranges.sort_by(|a, b| a.start.cmp(&b.start));

                let mut result = Vec::new();
                let mut current = new_ranges[0].clone();

                for range in new_ranges[1..].iter() {
                    if current.end < range.start {
                        result.push(current);
                        current = range.clone();
                    } else {
                        current.end = current.end.max(range.end);
                    }
                }
                result.push(current);

                self.ranges = result;
            }
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for range1 in self.ranges.iter() {
            for range2 in other.ranges.iter() {
                if range1.start < range2.end && range1.end > range2.start {
                    let start = range1.start.max(range2.start);
                    let end = range1.end.min(range2.end);
                    result.push(start..end);
                }
            }
        }
        RangeSet { ranges: result }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = self.ranges.clone();
        for other_range in other.ranges.iter() {
            let mut new_result = Vec::new();
            for range in result.iter() {
                if range.start >= other_range.end || range.end <= other_range.start {
                    new_result.push(range.clone());
                } else {
                    if range.start < other_range.start {
                        new_result.push(range.start..other_range.start);
                    }
                    if range.end > other_range.end {
                        new_result.push(other_range.end..range.end);
                    }
                }
            }
            result = new_result;
        }
        RangeSet { ranges: result }
    }

    pub fn get_first(&self) -> Option<i64> {
        self.ranges.first().map(|r| r.start)
    }

    pub fn iter(&self) -> std::vec::IntoIter<Range<i64>> {
        self.ranges.clone().into_iter()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    fn map_resources(contents: &str) -> (Vec<u64>, Vec<HashMap<u64, (u64, u64)>>){
        let mut resource_mappings: Vec<HashMap<u64, (u64, u64)>> = Vec::new();
        let mut seeds: Vec<u64> = Vec::new();
        for (i, line) in contents.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            if i == 0 {
                for seed in line.split(": ").filter(|s| !s.is_empty()).nth(1).unwrap().split(" ").filter(|s| !s.is_empty()) {
                    seeds.push(seed.parse::<u64>().unwrap());
                }
            } else if line.chars().next().unwrap().is_numeric() {
                let mapping_line =
                    line
                        .split(" ")
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                let destination_start = mapping_line[0];
                let source_start = mapping_line[1];
                let range = mapping_line[2];
                resource_mappings
                    .iter_mut()
                    .last()
                    .unwrap()
                    .insert(source_start, (destination_start, range));
            } else {
                resource_mappings.push(HashMap::new());
            }
        }

        (seeds, resource_mappings)
    }
    let (seeds, resource_mappings) = map_resources(&input);
    let mut lowest_location_num: u64 = u64::MAX;
    for seed in seeds {
        let mut key: u64 = seed;
        for (i, resource) in resource_mappings.iter().enumerate() {
            for (source, (destination, range)) in resource.iter() {
                if key >= *source && key < *source + *range {
                    key = destination + (key - source);
                    break;
                }
            }
            if i == resource_mappings.len() - 1 {
                if key < lowest_location_num {
                    lowest_location_num = key;
                }
            }
        }
    }
    Some(lowest_location_num)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut location_ranges: RangeSet = RangeSet { ranges: Vec::new() };
    let mut ranges_to_add: RangeSet = RangeSet { ranges: Vec::new() };
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            let parsed_seeds = line
                .split(": ")
                .filter(|s| !s.is_empty())
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .chunks(2)
                .map(|s| (s[0], s[1]))
                .collect::<Vec<(i64, i64)>>();

            for (start, range) in parsed_seeds {
                location_ranges.union(RangeSet {
                    ranges: vec![start..start + range],
                });
            }
            continue;
        }
        if line.chars().next().unwrap().is_numeric() {
            let mapping_line = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let destination_start = mapping_line[0];
            let source_start = mapping_line[1];
            let range = mapping_line[2];
            let mapping_difference = destination_start - source_start;
            let mapping_range = RangeSet {
                ranges: vec![source_start..source_start + range],
            };

            let range_diff = mapping_range.intersection(&location_ranges);
            for diff in range_diff.iter() {
                ranges_to_add.union(RangeSet {
                    ranges: vec![diff.start + mapping_difference..diff.end + mapping_difference],
                });
            }
            location_ranges = location_ranges.difference(&range_diff);
            continue;
        }
        location_ranges.union(ranges_to_add);
        ranges_to_add = RangeSet { ranges: Vec::new() };
    }
    location_ranges.union(ranges_to_add);

    location_ranges.get_first()
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
