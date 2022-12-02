extern crate core;

#[derive(PartialEq)]
enum Shape {
    ROCK,
    PAPER,
    SCISSOR,
}

enum Strategy {
    LOSS,
    DRAW,
    WIN,
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_points: u32 = input
        .lines()
        .map(|r| r.split_once(" ").unwrap())
        .map(|(opponent, me)| get_result(get_shape(opponent), get_shape(me)))
        .sum();

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_points: u32 = input
        .lines()
        .map(|r| r.split_once(" ").unwrap())
        .map(|(opponent, me)| follow_strategy(get_shape(opponent), get_strategy(me)))
        .sum();

    Some(total_points)
}

fn follow_strategy(opponent: Shape, me: Strategy) -> u32 {
    match me {
        Strategy::WIN => 6 + get_points(use_winning(opponent)),
        Strategy::DRAW => 3 + get_points(opponent),
        Strategy::LOSS => get_points(use_losing(opponent)),
    }
}

fn get_result(opponent: Shape, me: Shape) -> u32 {
    // draw
    if me == opponent {
        return 3 + get_points(me);
    }

    // win
    if me == use_winning(opponent) {
        return 6 + get_points(me);
    }

    // loss
    get_points(me)
}

fn get_shape(char: &str) -> Shape {
    match char {
        "A" | "X" => Shape::ROCK,
        "B" | "Y" => Shape::PAPER,
        "C" | "Z" => Shape::SCISSOR,
        _ => panic!("unknown input"),
    }
}

fn get_strategy(char: &str) -> Strategy {
    match char {
        "X" => Strategy::LOSS,
        "Y" => Strategy::DRAW,
        "Z" => Strategy::WIN,
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

fn use_winning(shape: Shape) -> Shape {
    match shape {
        Shape::ROCK => Shape::PAPER,
        Shape::PAPER => Shape::SCISSOR,
        Shape::SCISSOR => Shape::ROCK,
    }
}

fn use_losing(shape: Shape) -> Shape {
    match shape {
        Shape::ROCK => Shape::SCISSOR,
        Shape::PAPER => Shape::ROCK,
        Shape::SCISSOR => Shape::PAPER,
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
        assert_eq!(part_two(&input), Some(12));
    }
}
