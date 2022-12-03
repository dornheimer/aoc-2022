use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let total_priorities: u32 = input
        .lines()
        .map(|r| {
            let (compartment1, compartment2) = r.split_at(r.len() / 2);

            compartment1
                .chars()
                .find(|&c| compartment2.contains(c))
                .map(|c| get_priority(c))
                .unwrap()
        })
        .sum();

    Some(total_priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_priorities = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let elf_a = group.next().unwrap();
            let elf_b = group.next().unwrap();
            let elf_c = group.next().unwrap();

            elf_a
                .chars()
                .find(|&char| elf_b.contains(char) && elf_c.contains(char))
                .map(|c| get_priority(c))
                .unwrap()
        })
        .sum();

    Some(total_priorities)
}

fn get_priority(char: char) -> u32 {
    match char.is_ascii_uppercase() {
        true => char as u32 - 38, // (64 - 26)
        false => char as u32 - 96,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
