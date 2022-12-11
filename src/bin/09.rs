use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Motion {
    direction: Direction,
    amount: usize,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').unwrap();
        let direction = match direction {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };

        Ok(Self {
            direction,
            amount: amount.parse().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_tail_positions(input, 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_tail_positions(input, 10) as u32)
}

fn count_tail_positions(input: &str, num_knots: usize) -> usize {
    let mut knots = vec![Coordinate::default(); num_knots];
    let mut visited = HashSet::new();
    visited.insert(knots[knots.len() - 1].clone());

    input
        .lines()
        .map(|l| Motion::from_str(l).unwrap())
        .for_each(|m| {
            for _step in 0..m.amount {
                let mut position_head = &mut knots[0];
                match m.direction {
                    Direction::Left => position_head.x -= 1,
                    Direction::Right => position_head.x += 1,
                    Direction::Up => position_head.y += 1,
                    Direction::Down => position_head.y -= 1,
                };

                for i in (0..knots.len()).skip(1) {
                    let diff = knots[i - 1] - knots[i];
                    if !is_touching(&diff) {
                        let diff_unit = diff_to_unit(&diff);
                        knots[i] = knots[i] + diff_unit;
                    }
                }

                visited.insert(knots[knots.len() - 1].clone());
            }
        });

    visited.len()
}

fn is_touching(diff: &Coordinate) -> bool {
    (diff.x == 0 && diff.y == 0)
        || (diff.x.abs() == 1 && diff.y.abs() == 0)
        || (diff.x.abs() == 0 && diff.y.abs() == 1)
        || (diff.x.abs() == 1 && diff.y.abs() == 1)
}

fn diff_to_unit(diff: &Coordinate) -> Coordinate {
    Coordinate {
        x: if diff.x == 0 {
            0
        } else if diff.x.is_negative() {
            -1
        } else {
            1
        },
        y: if diff.y == 0 {
            0
        } else if diff.y.is_negative() {
            -1
        } else {
            1
        },
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two_simple() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(&input), Some(36));
    }
}
