use crate::grid::Grid;
use crate::{grid, util};

pub fn part_one() {
    let input = util::read(2024, 8, false);
    let grid = Grid::from_string(&input);
    let mut anti_nodes = Grid::new(grid.get_width().clone(), grid.get_height().clone(), '.');

    let values: Vec<char> = grid.distinct()
        .iter().filter(|&&val| val != '#' && val != '.')
        .cloned().collect();
    for value in &values {
        println!("Char: {}", value);
        let positions = grid.find_all(|item| { *item == *value });
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let distance = grid::calculate_distance(&positions[i], &positions[j]);
                let for_origin = grid::mirror(&distance);
                let for_target = distance.clone();
                if let Some(anti_node) = grid.translate(&positions[i], &for_origin, false) {
                    anti_nodes.set_value(&anti_node, '#');
                }
                if let Some(anti_node) = grid.translate(&positions[j], &for_target, false) {
                    anti_nodes.set_value(&anti_node, '#');
                }
            }
        }
    }

    anti_nodes.print(|x| x.to_string());

    let total = anti_nodes.find_all(|&x| x == '#')
        .iter().count();
    println!("Total: {}", total);
}

pub fn part_two() {
    let input = util::read(2024, 8, false);
    let grid = Grid::from_string(&input);
    let mut anti_nodes = Grid::new(grid.get_width().clone(), grid.get_height().clone(), '.');

    let values: Vec<char> = grid.distinct()
        .iter().filter(|&&val| val != '#' && val != '.')
        .cloned().collect();
    for value in &values {
        println!("Char: {}", value);
        let positions = grid.find_all(|item| { *item == *value });
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let distance = grid::calculate_distance(&positions[i], &positions[j]);

                let mut left = positions[i].clone();
                let mut right = positions[j].clone();

                // translations
                let for_origin = grid::mirror(&distance);
                let for_target = distance.clone();

                anti_nodes.set_value(&positions[i], '#');
                anti_nodes.set_value(&positions[j], '#');

                while let Some(translated) = grid.translate(&left, &for_origin, false) {
                    anti_nodes.set_value(&translated, '#');
                    left = translated;
                }
                while let Some(translated) = grid.translate(&right, &for_target, false) {
                    anti_nodes.set_value(&translated, '#');
                    right = translated;
                }
            }
        }
    }

    anti_nodes.print(|x| x.to_string());

    let total = anti_nodes.find_all(|&x| x == '#')
        .iter().count();
    println!("Total: {}", total);
}