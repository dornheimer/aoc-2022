use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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
    let mut position_head = Coordinate { x: 0, y: 0 };
    let mut position_tail = Coordinate { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(position_tail);

    input
        .lines()
        .map(|l| Motion::from_str(l).unwrap())
        .for_each(|m| {
            for _step in 0..m.amount {
                match m.direction {
                    Direction::Left => position_head.x -= 1,
                    Direction::Right => position_head.x += 1,
                    Direction::Up => position_head.y += 1,
                    Direction::Down => position_head.y -= 1,
                };

                let diff = position_head - position_tail;
                if !is_touching(&diff) {
                    position_tail = position_tail + diff_to_unit(&diff);
                }

                visited.insert(position_tail.clone());
            }
        });

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
