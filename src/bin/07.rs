use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
struct Item {
    file_type: FileType,
    path: String,
    size: usize,
    parent: Option<String>,
    depth: usize,
    children_dirs: Vec<String>,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, name) = s.split_once(' ').unwrap();

        return if id == "dir" {
            Ok(Self {
                file_type: FileType::Directory,
                path: name.to_owned(),
                size: 0,
                parent: None,
                depth: 0,
                children_dirs: vec![],
            })
        } else {
            Ok(Self {
                file_type: FileType::File,
                path: name.to_owned(),
                size: id.parse::<usize>().unwrap(),
                parent: None,
                depth: 0,
                children_dirs: vec![],
            })
        };
    }
}

enum Command {
    LS,
    CD(String),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        parts.next().unwrap();
        let command_type = parts.next().unwrap();
        let argument = parts.next();

        match command_type {
            "ls" => Ok(Command::LS),
            "cd" => Ok(Command::CD(argument.unwrap().to_string())),
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let file_system = parse_file_system(input);

    Some(
        file_system
            .values()
            .filter(|i| i.size <= 100000)
            .map(|i| i.size)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let file_system = parse_file_system(input);
    let total_space: usize = 70000000;
    let target_unused: usize = 30000000;
    let used = file_system.get("/").unwrap().size;

    let actual_unused = total_space - used;
    let required_to_free = target_unused - actual_unused;

    Some(
        file_system
            .values()
            .filter(|i| i.size >= required_to_free)
            .min_by(|a, b| Ord::cmp(&a.size, &b.size))
            .unwrap()
            .size as u32,
    )
}

fn parse_file_system(input: &str) -> HashMap<String, Item> {
    let mut file_system = HashMap::<String, Item>::new();
    let mut collecting = false;

    let mut current_dir = "/".to_owned();
    let mut current_depth: usize = 0;
    file_system.insert(
        current_dir.to_string(),
        Item {
            file_type: FileType::Directory,
            path: current_dir.clone(),
            size: 0,
            parent: None,
            depth: current_depth,
            children_dirs: vec![],
        },
    );

    for line in input.lines().skip(1) {
        if !collecting || line.starts_with('$') {
            let command = Command::from_str(line).unwrap();
            match command {
                Command::LS => collecting = true,
                Command::CD(dir) => {
                    if dir == ".." {
                        let parent = file_system
                            .get(&current_dir)
                            .unwrap()
                            .parent
                            .as_ref()
                            .unwrap();
                        current_dir = parent.clone();
                        current_depth -= 1;
                    } else {
                        if current_dir != "/" {
                            current_dir.push_str("/");
                        }
                        current_dir.push_str(dir.as_str());
                        current_depth += 1;
                    }

                    collecting = false;
                }
            }
        } else if collecting {
            let mut item = Item::from_str(line).unwrap();
            item.parent = Some(current_dir.clone());
            let parent = file_system.get_mut(&current_dir).unwrap();

            match item.file_type {
                FileType::File => {
                    file_system.get_mut(&current_dir).unwrap().size += item.size;
                }
                FileType::Directory => {
                    let path = if current_dir == "/" {
                        current_dir.clone() + item.path.as_str()
                    } else {
                        current_dir.clone() + "/" + item.path.as_str()
                    };

                    parent.children_dirs.push(path.clone());

                    item.path = path;
                    item.depth = current_depth + 1;
                    file_system.insert(item.path.clone(), item);
                }
            }
        }
    }

    let mut totals = file_system.clone();
    for item in file_system
        .values()
        .sorted_by(|a, b| Ord::cmp(&b.depth, &a.depth))
    {
        let children_sum: usize = item
            .children_dirs
            .iter()
            .map(|c| totals.get(c).unwrap().size)
            .sum();

        let size = children_sum + item.size;
        totals.get_mut(&item.path).unwrap().size = size;

        if let Some(parent) = item.parent.as_ref() {
            totals.get_mut(parent).unwrap().size += size;
        }
    }

    totals
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_dir_size() {
        let input = advent_of_code::read_file("examples", 7);
        let file_system = parse_file_system(&input);
        assert_eq!(file_system.get("/a/e").unwrap().size, 584);
        assert_eq!(file_system.get("/a").unwrap().size, 94853);
        assert_eq!(file_system.get("/d").unwrap().size, 24933642);
        assert_eq!(file_system.get("/").unwrap().size, 48381165);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
