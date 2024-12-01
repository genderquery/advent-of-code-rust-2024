use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_lists(input);

    left.sort();
    right.sort();

    let sum_dist: u32 = left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum();

    Some(sum_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_lists(input);

    let mut right_freq: HashMap<u32, u32> = HashMap::new();

    for r in right.iter().copied() {
        right_freq.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    let score = left
        .iter()
        .map(|l| {
            let freq = right_freq.get(l).copied().unwrap_or(0);
            l * freq
        })
        .sum();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
