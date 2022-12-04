pub fn part_one(input: &str) -> Option<u32> {
    let fully_containing = input
        .lines()
        .filter(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (min_a, max_a) = parse_sections(a);
            let (min_b, max_b) = parse_sections(b);

            (min_a >= min_b && max_a <= max_b) || (min_b >= min_a && max_b <= max_a)
        })
        .count();

    Some(fully_containing as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let overlapping = input
        .lines()
        .filter(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (min_a, max_a) = parse_sections(a);
            let (min_b, max_b) = parse_sections(b);

            !((max_a < min_b) || (max_b < min_a))
        })
        .count();

    Some(overlapping as u32)
}

fn parse_sections(s: &str) -> (u32, u32) {
    let (min, max) = s.split_once('-').unwrap();
    (min.parse::<u32>().unwrap(), max.parse::<u32>().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
