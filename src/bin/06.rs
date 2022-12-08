use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    find_message_start(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_message_start(input, 14)
}

fn find_message_start(input: &str, length: usize) -> Option<u32> {
    for (i, _c) in input.chars().enumerate() {
        let mut unique = HashSet::with_capacity(length);
        if i >= (length - 1) {
            let is_unique = input[i - (length - 1)..=i]
                .chars()
                .all(|c| unique.insert(c));
            if is_unique {
                return Some(i as u32 + 1);
            }

            unique.clear();
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let mut lines = input.lines();

        assert_eq!(part_one(lines.next().unwrap()), Some(7));
        assert_eq!(part_one(lines.next().unwrap()), Some(5));
        assert_eq!(part_one(lines.next().unwrap()), Some(6));
        assert_eq!(part_one(lines.next().unwrap()), Some(10));
        assert_eq!(part_one(lines.next().unwrap()), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let mut lines = input.lines();

        assert_eq!(part_two(lines.next().unwrap()), Some(19));
        assert_eq!(part_two(lines.next().unwrap()), Some(23));
        assert_eq!(part_two(lines.next().unwrap()), Some(23));
        assert_eq!(part_two(lines.next().unwrap()), Some(29));
        assert_eq!(part_two(lines.next().unwrap()), Some(26));
    }
}
