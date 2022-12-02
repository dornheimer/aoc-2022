extern crate core;

#[derive(Debug, PartialEq)]
enum Shape {
    ROCK,
    PAPER,
    SCISSOR,
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_points: u32 = input
        .lines()
        .map(|r| r.split_once(" ").unwrap())
        .map(|(opponent, me)| get_result(get_move(opponent), get_move(me)))
        .sum();

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_result(opponent: Shape, me: Shape) -> u32 {
    // draw
    if me == opponent {
        return 3 + get_points(me);
    }

    // win
    if (me == Shape::ROCK && opponent == Shape::SCISSOR)
        || (me == Shape::PAPER && opponent == Shape::ROCK)
        || (me == Shape::SCISSOR && opponent == Shape::PAPER)
    {
        return 6 + get_points(me);
    }

    // loss
    get_points(me)
}

fn get_move(char: &str) -> Shape {
    match char {
        "A" | "X" => Shape::ROCK,
        "B" | "Y" => Shape::PAPER,
        "C" | "Z" => Shape::SCISSOR,
        _ => panic!("unknown input"),
    }
}

fn get_points(shape: Shape) -> u32 {
    match shape {
        Shape::ROCK => 1,
        Shape::PAPER => 2,
        Shape::SCISSOR => 3,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
