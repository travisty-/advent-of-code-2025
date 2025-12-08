use std::sync::LazyLock;
use fancy_regex::Regex;

advent_of_code::solution!(2);

static PART_ONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\1$").unwrap());
static PART_TWO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\1+$").unwrap());

fn parse_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.trim().split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        (start, end)
    })
}

fn sum_invalid_ids(input: &str, pattern: &Regex) -> u64 {
    parse_ranges(input)
        .flat_map(|(start, end)| start..=end)
        .filter(|id| pattern.is_match(&id.to_string()).unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(sum_invalid_ids(input, &PART_ONE))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(sum_invalid_ids(input, &PART_TWO))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
