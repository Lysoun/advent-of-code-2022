use std::collections::HashMap;
use std::fs;

fn safe_file_read(path: &str) -> String {
    return match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
}

fn is_tree_visible_on_line(
    start_line_number: u16,
    end_line_number: u16,
    tree_column_number: u16,
    tree_height: &u8,
    trees: &HashMap<(u16, u16), u8>,
) -> bool {
    let mut tree_is_visible = true;
    let mut line_number = start_line_number;
    while tree_is_visible && line_number <= end_line_number {
        let checked_tree_height = trees.get(&(line_number, tree_column_number)).unwrap();
        tree_is_visible = checked_tree_height < tree_height;
        line_number += 1;
    }

    return tree_is_visible;
}

fn is_tree_visible_on_column(
    start_column_number: u16,
    end_column_number: u16,
    tree_line_number: u16,
    tree_height: &u8,
    trees: &HashMap<(u16, u16), u8>) -> bool {
    let mut tree_is_visible = true;
    let mut column_number = start_column_number;
    while tree_is_visible && column_number <= end_column_number {
        let checked_tree_height = trees.get(&(tree_line_number, column_number)).unwrap();
        tree_is_visible = checked_tree_height < tree_height;
        column_number += 1;
    }

    return tree_is_visible;
}

fn is_tree_visible(tree: (&(u16, u16), &u8), trees: &HashMap<(u16, u16), u8>) -> bool {
    let tree_line_number = tree.0.0;
    let tree_column_number = tree.0.1;
    let tree_height = tree.1;

    if tree_column_number == 0 || tree_line_number == 0 {
        return true;
    }

    let grid_size: u16 = 98;

    return is_tree_visible_on_line(0, tree_line_number - 1, tree_column_number, &tree_height, trees) ||
        is_tree_visible_on_line(tree_line_number + 1, grid_size, tree_column_number, &tree_height, trees) ||
        is_tree_visible_on_column(0, tree_column_number - 1, tree_line_number, &tree_height, trees) ||
        is_tree_visible_on_column(tree_column_number + 1, grid_size, tree_line_number, &tree_height, trees);
}

fn compute_scenic_score(tree: (&(u16, u16), &u8), trees: &HashMap<(u16, u16), u8>) -> u32 {
    let tree_line_number = tree.0.0;
    let tree_column_number = tree.0.1;
    let tree_height = tree.1;
    let mut trees_visible: (u32, u32, u32, u32) = (0, 0, 0, 0);

    for i in (0..tree_line_number).rev() {
        trees_visible.0 += 1;
        if trees.get(&(i, tree_column_number)).unwrap() >= tree_height {
            break;
        }
    }

    for i in (0..tree_column_number).rev() {
        trees_visible.1 += 1;
        if trees.get(&(tree_line_number, i)).unwrap() >= tree_height {
            break;
        }
    }

    for i in (tree_line_number + 1)..99 {
        trees_visible.2 += 1;
        if trees.get(&(i, tree_column_number)).unwrap() >= tree_height {
            break;
        }
    }

    for i in (tree_column_number + 1)..99 {
        trees_visible.3 += 1;
        if trees.get(&(tree_line_number, i)).unwrap() >= tree_height {
            break;
        }
    }

    return trees_visible.0 * trees_visible.1 * trees_visible.2 * trees_visible.3;
}

fn main() {
    let input = safe_file_read("input.txt");
    let mut trees: HashMap<(u16, u16), u8> = HashMap::new();
    let mut line_number: u16 = 0;
    let mut column_number: u16 = 0;
    let input_lines = input.lines();

    for line in input_lines {
        let line_chars = line.chars();
        for tree_height in line_chars {
            trees.insert((line_number, column_number), tree_height.to_digit(10).unwrap() as u8);
            column_number += 1;
        }
        column_number = 0;
        line_number += 1;
    }

    let mut iter = trees.iter();
    let mut max_scenic_score: u32 = 0;

    let mut tree = iter.next();
    while tree.is_some() {
        let unwrapped_tree = tree.unwrap();
        if is_tree_visible(unwrapped_tree, &trees) {
            let tree_scenic_score = compute_scenic_score(unwrapped_tree, &trees);
            if tree_scenic_score > max_scenic_score {
                max_scenic_score = tree_scenic_score;
            }
        }
        tree = iter.next();
    }

    println!("Maximum scenic score: {}", max_scenic_score);
}
