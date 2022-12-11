use std::str::FromStr;

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

enum Instruction {
    Add(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts_iter = s.split_whitespace();
        let instruction = parts_iter.next().unwrap();

        Ok(match instruction {
            "noop" => Self::Noop,
            "addx" => Self::Add(parts_iter.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instruction_buffer = parse_instructions(input);

    let mut register = 1;
    let measure_points = [20, 60, 100, 100, 140, 180, 220];
    let mut signal_strengths = vec![];
    let mut cycle = 0;
    for instruction in instruction_buffer.iter() {
        cycle += 1;

        if measure_points.contains(&cycle) {
            signal_strengths.push(register * cycle as i32);
        }

        if let Some(value_to_add) = instruction {
            register += value_to_add;
        }
    }

    Some(signal_strengths.iter().sum())
}

pub fn part_two(input: &str) -> Option<String> {
    let instruction_buffer = parse_instructions(input);

    let mut crt = [['.'; WIDTH]; HEIGHT];

    let mut cycle = 0;
    let mut register: i32 = 1;
    let mut current_row = 0;
    for instruction in instruction_buffer.iter() {
        cycle += 1;

        draw_sprite(
            &mut crt,
            (cycle - 1) % 40,
            (register).max(0) as usize,
            current_row,
        );

        if let Some(value_to_add) = instruction {
            register += value_to_add;
        }

        println!("cycle {cycle} instruction {instruction:?} register {register} current row {current_row}");

        if cycle % 40 == 0 {
            current_row += 1;
        }
    }

    Some(print_crt(&crt))
}

fn draw_sprite(crt: &mut [[char; 40]], position: usize, register: usize, current_row: usize) {
    println!("{}", register);
    if position == register {
        crt[current_row][register] = '#';
    }

    if position == register + 1 {
        crt[current_row][register + 1] = '#';
    }

    if register > 0 && position == register - 1 {
        crt[current_row][register - 1] = '#';
    }

    let row: String = (0..WIDTH).map(|x| crt[current_row][x]).collect();
    println!("position {position} {}", row);
}

fn print_crt(crt: &[[char; 40]]) -> String {
    let mut output = "".to_string();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            output += &crt[y][x].to_string();
        }
        output += "\n";
    }

    output
}

fn parse_instructions(input: &str) -> Vec<Option<i32>> {
    let mut instruction_buffer: Vec<Option<i32>> = vec![];
    for instruction in input.lines() {
        let instruction = Instruction::from_str(instruction).unwrap();
        match instruction {
            Instruction::Noop => instruction_buffer.push(None),
            Instruction::Add(value) => {
                instruction_buffer.push(None);
                instruction_buffer.push(Some(value));
            }
        };
    }

    instruction_buffer
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);

        let image = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        let output = part_two(&input);
        println!("{}", output.clone().unwrap());
        assert_eq!(output, Some(image.to_string()));
    }
}
