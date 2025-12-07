advent_of_code::solution!(2);

fn parse_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.trim().split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        (start, end)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for (start, end) in parse_ranges(input) {
        for id in start..=end {
            let item = id.to_string();
            let (left, right) = item.split_at(item.len() / 2);
            if left == right {
                sum += id;
            }
        }
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
