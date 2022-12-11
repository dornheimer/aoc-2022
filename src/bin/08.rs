pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|t| t.to_digit(10).unwrap()).collect())
        .collect();
    let width = grid[1].len();
    let height = grid.len();
    let num_trees_edge = 2 * height + (width - 2) * 2;

    let grid_iter = grid
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, _l)| i + 1 < height);

    let mut num_visible_trees = 0;
    for (i, row) in grid_iter {
        for (j, tree) in row
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(i, _l)| i + 1 < width)
        {
            let visible_left = row
                .iter()
                .enumerate()
                .filter(|(ri, _t)| *ri < j)
                .all(|(_ri, other)| other < tree);

            let visible_right = row
                .iter()
                .enumerate()
                .filter(|(ri, _t)| *ri > j)
                .all(|(_ri, other)| other < tree);

            let visible_top = grid
                .iter()
                .enumerate()
                .filter(|(ri, _l)| i + 1 < height && i > *ri)
                .all(|(_x, r)| &r[j] < tree);

            let visible_bottom = grid
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(ri, _l)| i < *ri)
                .all(|(_x, r)| &r[j] < tree);

            if visible_left || visible_right || visible_top || visible_bottom {
                num_visible_trees += 1;
            }
        }
    }

    Some((num_trees_edge + num_visible_trees) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|t| t.to_digit(10).unwrap()).collect())
        .collect();

    let mut score_grid = grid.clone();
    for (i, row) in grid.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let num_visible_left = get_num_visible_left(&row, j, tree);
            let num_visible_right = get_num_visible_right(&row, j, tree);
            let num_visible_top = get_num_visible_top(&grid, i, j, tree);
            let num_visible_bottom = get_num_visible_bottom(&grid, i, j, tree);

            let score = num_visible_left * num_visible_right * num_visible_top * num_visible_bottom;
            score_grid[i][j] = score as u32;
        }
    }

    Some(*score_grid.iter().flatten().max().unwrap())
}

fn get_num_visible_left(row: &Vec<u32>, j: usize, tree: &u32) -> usize {
    let mut count = 0;
    for (_i, other) in row.iter().enumerate().filter(|(ri, _t)| *ri < j).rev() {
        count += 1;
        if other >= tree {
            break;
        }
    }

    count
}

fn get_num_visible_right(row: &Vec<u32>, j: usize, tree: &u32) -> usize {
    let mut count = 0;
    for (_i, other) in row.iter().enumerate().filter(|(ri, _t)| *ri > j) {
        count += 1;
        if other >= tree {
            break;
        }
    }

    count
}

fn get_num_visible_top(grid: &Vec<Vec<u32>>, i: usize, j: usize, tree: &u32) -> usize {
    let mut count = 0;
    for (_ri, r) in grid.iter().enumerate().filter(|(ri, _l)| i > *ri).rev() {
        count += 1;
        if &r[j] >= tree {
            break;
        }
    }

    count
}

fn get_num_visible_bottom(grid: &Vec<Vec<u32>>, i: usize, j: usize, tree: &u32) -> usize {
    let mut count = 0;
    for (_ri, r) in grid.iter().enumerate().filter(|(ri, _l)| i < *ri) {
        count += 1;
        if &r[j] >= tree {
            break;
        }
    }

    count
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
