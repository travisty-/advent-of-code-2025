advent_of_code::solution!(1);

fn parse_rotations(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines().map(|line| {
        let (rotation, distance) = line.split_at(1);
        let distance: i32 = distance.parse().unwrap();
        if rotation == "L" { -distance } else { distance }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut count = 0;

    for rotation in parse_rotations(input) {
        let position = dial + rotation;
        dial = position.rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    Some(count)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
