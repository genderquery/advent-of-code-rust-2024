advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(levels: &[u32]) -> bool {
    is_gradual(levels) && (is_increasing(levels) || is_decreasing(levels))
}

fn is_safe_with_dampener(levels: &[u32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut levels = levels.to_vec();
        levels.remove(i);
        if is_safe(&levels) {
            return true;
        }
    }

    false
}

fn is_gradual(levels: &[u32]) -> bool {
    !levels
        .windows(2)
        .any(|w| w[0].abs_diff(w[1]) < 1 || w[0].abs_diff(w[1]) > 3)
}

fn is_increasing(levels: &[u32]) -> bool {
    !levels.windows(2).any(|w| w[0] >= w[1])
}

fn is_decreasing(levels: &[u32]) -> bool {
    !levels.windows(2).any(|w| w[0] <= w[1])
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input);
    let safe_count = reports.iter().filter(|levels| is_safe(levels)).count();
    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input);
    let safe_count = reports
        .iter()
        .filter(|levels| is_safe_with_dampener(levels))
        .count();
    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing(&[1, 2, 3, 4, 5]));
        assert!(!is_increasing(&[5, 4, 3, 2, 1]));
        assert!(is_increasing(&[1, 3, 5, 7, 9]));
        assert!(!is_increasing(&[9, 7, 5, 3, 1]));
        assert!(!is_increasing(&[1, 2, 3, 4, 3, 2]));
        assert!(!is_increasing(&[1, 1, 1, 1, 2]));
    }

    #[test]
    fn test_is_decreasing() {
        assert!(!is_decreasing(&[1, 2, 3, 4, 5]));
        assert!(is_decreasing(&[5, 4, 3, 2, 1]));
        assert!(!is_decreasing(&[1, 3, 5, 7, 9]));
        assert!(is_decreasing(&[9, 7, 5, 3, 1]));
        assert!(!is_decreasing(&[1, 2, 3, 4, 3, 2]));
        assert!(!is_decreasing(&[2, 1, 1, 1, 1]));
    }

    #[test]
    fn test_is_gradual() {
        assert!(is_gradual(&[1, 2, 3, 4, 5]));
        assert!(is_gradual(&[5, 4, 3, 2, 1]));
        assert!(is_gradual(&[1, 3, 5, 7, 9]));
        assert!(!is_gradual(&[1, 5, 9, 13, 17]));
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[7, 6, 4, 2, 1]));
        assert!(!is_safe(&[1, 2, 7, 8, 9]));
        assert!(!is_safe(&[9, 7, 6, 2, 1]));
        assert!(!is_safe(&[1, 3, 2, 4, 5]));
        assert!(!is_safe(&[8, 6, 4, 4, 1]));
        assert!(is_safe(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_is_safe_with_dampener() {
        assert!(is_safe_with_dampener(&[7, 6, 4, 2, 1]));
        assert!(!is_safe_with_dampener(&[1, 2, 7, 8, 9]));
        assert!(!is_safe_with_dampener(&[9, 7, 6, 2, 1]));
        assert!(is_safe_with_dampener(&[1, 3, 2, 4, 5]));
        assert!(is_safe_with_dampener(&[8, 6, 4, 4, 1]));
        assert!(is_safe_with_dampener(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
