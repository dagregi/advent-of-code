use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|digit: &str| {
            digit
                .chars()
                .map(|num| num.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let mut visable_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree)| {
            let size = tree.len() - 1;
            tree.iter()
                .enumerate()
                .map(|(inner_idx, _)| {
                    i == 0 || i == trees.len() - 1 || inner_idx == 0 || inner_idx == size
                })
                .collect()
        })
        .collect();

    // X-axis
    for y in 0..trees.len() {
        let mut current_size = 0;
        for x in 0..trees[0].len() {
            if x == 0 {
                current_size = trees[y][x] as usize;
            } else if trees[y][x] > current_size as u32 {
                current_size = trees[y][x] as usize;
                visable_trees[y][x] = true;
            }
        }
    }
    for y in (0..trees.len()).rev() {
        let mut current_size = 0;
        for x in (0..trees[0].len()).rev() {
            if x == trees.len() - 1 {
                current_size = trees[y][x] as usize;
            } else if trees[y][x] > current_size as u32 {
                current_size = trees[y][x] as usize;
                visable_trees[y][x] = true;
            }
        }
    }

    // Y-axis
    for x in 0..trees.len() {
        let mut current_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_size = trees[y][x] as usize;
            } else if trees[y][x] > current_size as u32 {
                current_size = trees[y][x] as usize;
                visable_trees[y][x] = true;
            }
        }
    }
    for x in (0..trees.len()).rev() {
        let mut current_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == trees.len() - 1 {
                current_size = trees[y][x] as usize;
            } else if trees[y][x] > current_size as u32 {
                current_size = trees[y][x] as usize;
                visable_trees[y][x] = true;
            }
        }
    }

    // Part 1
    let visible_tree_count = visable_trees
        .iter()
        .flatten()
        .filter(|&&v| v)
        .count()
        .to_string();

    let mut high_score = 0;
    let x_max = trees[0].len();
    let y_max = trees.len();

    for (y_idx, tree) in trees.iter().enumerate() {
        for (x_idx, tree_height) in tree.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];
            // left
            for x_pos in (0..x_idx).rev() {
                if trees[y_idx][x_pos] < *tree_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }
            // right
            for x_pos in (x_idx + 1)..x_max {
                if trees[y_idx][x_pos] < *tree_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }
            // up
            for y_pos in (0..y_idx).rev() {
                if trees[y_pos][x_idx] < *tree_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }
            // down
            for y_pos in (y_idx + 1)..y_max {
                if trees[y_pos][x_idx] < *tree_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }

            // Part 2
            let scenic_score = scores.iter().product();
            if scenic_score > high_score {
                high_score = scenic_score;
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", visible_tree_count, high_score);
}
