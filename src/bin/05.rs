use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let captures = RE.captures(s).unwrap();
        let from = (&captures[2]).parse::<usize>().unwrap();
        let to = (&captures[3]).parse::<usize>().unwrap();

        Ok(Instruction {
            from: from - 1,
            to: to - 1,
            amount: (&captures[1]).parse().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut supplies = parse_stacks(stacks);

    instructions.lines().for_each(|l| {
        let i = Instruction::from_str(l).unwrap();
        (0..i.amount).for_each(|_n| {
            let cargo = supplies[i.from].pop().unwrap();
            supplies[i.to].push(cargo);
        });
    });

    Some(get_top_crates(supplies))
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut supplies = parse_stacks(stacks);

    instructions.lines().for_each(|l| {
        let i = Instruction::from_str(l).unwrap();
        let index = supplies[i.from].len() - i.amount;
        let cargo = supplies[i.from].split_off(index);
        supplies[i.to].extend(cargo);
    });

    Some(get_top_crates(supplies))
}

fn parse_stacks(input: &str) -> Vec<Vec<&str>> {
    let mut stack_iter = input.lines().rev();
    let no_stacks = stack_iter
        .next()
        .map(|l| l.trim().chars().filter(|c| !c.is_whitespace()).count())
        .unwrap();

    let mut stacks: Vec<Vec<&str>> = vec![vec![]; no_stacks];
    stack_iter.for_each(|row| {
        get_stacks(row)
            .iter()
            .enumerate()
            .for_each(|(ix, cargo)| match cargo {
                None => (),
                Some(cargo) => stacks[ix].push(cargo),
            })
    });

    stacks
}

fn get_top_crates(supplies: Vec<Vec<&str>>) -> String {
    supplies.iter().filter_map(|stack| stack.last()).join("")
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn get_stacks(row: &str) -> Vec<Option<&str>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s{3})\s?|\[(\w)?]\s?").unwrap();
    }

    RE.captures_iter(row)
        .map(|c| c.get(2).map_or(None, |m| Some(m.as_str())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }

    #[test]
    fn test_get_stacks() {
        assert_eq!(get_stacks("[Z] [M]    "), vec![Some("Z"), Some("M"), None]);
        assert_eq!(get_stacks("[Z]        "), vec![Some("Z"), None, None]);
        assert_eq!(get_stacks("    [M] [P]"), vec![None, Some("M"), Some("P")]);
        assert_eq!(get_stacks("        [P]"), vec![None, None, Some("P")]);
        assert_eq!(
            get_stacks("[Z] [M] [P]"),
            vec![Some("Z"), Some("M"), Some("P")]
        );
    }
}
