use std::collections::HashSet;
use crate::grid::{Grid, Point};
use crate::{grid, util};

pub fn part_one() {
    let input = util::read(2024, 6, false);
    let mut grid = Grid::from_string(&input);
    // grid.print(|val| val.to_string());
    let mut guard = grid.find(|val| {
        *val == '^' || *val == '>' || *val == '<' || *val == 'v'
    }).unwrap();

    let directions = [grid::NORTH, grid::EAST, grid::SOUTH, grid::WEST];
    let mut current_direction: usize = match grid.get_value(&guard) {
        '^' => 0,
        '>' => 1,
        'v' => 2,
        '<' => 3,
        _ => panic!("Unsupported direction")
    };

    // println!("Guard direction: {:?}", current_direction);

    grid.set_value(&guard, 'X');

    while let Some(translated) = grid.translate(&guard, &directions[current_direction], false) {
        if *grid.get_value(&translated) == '#' {
            current_direction = (current_direction + 1) % directions.len();
        } else {
            guard = translated;
            grid.set_value(&guard, 'X');
        }
    }

    let mut count = 0;
    grid.visit(|val, _| {
        if *val == 'X' {
            count = count + 1;
        }
    });
    println!("Visited: {}", count);

    // grid.print(|char| char.to_string());
}

pub fn part_two() {
    let input = util::read(2024, 6, false);
    let mut grid = Grid::from_string(&input);
    // grid.print(|val| val.to_string());
    let start_guard = grid.find(|val| {
        *val == '^' || *val == '>' || *val == '<' || *val == 'v'
    }).unwrap();

    let directions = [grid::NORTH, grid::EAST, grid::SOUTH, grid::WEST];
    let start_direction: usize = match grid.get_value(&start_guard) {
        '^' => 0,
        '>' => 1,
        'v' => 2,
        '<' => 3,
        _ => panic!("Unsupported direction")
    };

    // Traverse one time marking possible entry locations with an X
    let mut guard = start_guard.clone();
    let mut current_direction = start_direction.clone();
    while let Some(translated) = grid.translate(&guard, &directions[current_direction], false) {
        if *grid.get_value(&translated) == '#' {
            current_direction = (current_direction + 1) % directions.len();
        } else {
            guard = translated;
            grid.set_value(&guard, 'X');
        }
    }

    let mut count = 0;

    for y in 0..grid.get_height() {
        for x in 0..grid.get_width() {
            let point = Point{x, y};
            let val = grid.get_value(&point).clone();
            if val != 'X' {
                continue;
            }

            grid.set_value(&point, '#');

            guard = start_guard.clone();
            current_direction = start_direction.clone();
            let mut edges: HashSet<String> = HashSet::new();

            while let Some(translated) = grid.translate(&guard, &directions[current_direction], false) {
                if *grid.get_value(&translated) == '#' {
                    current_direction = (current_direction + 1) % directions.len();
                } else {
                    let edge = format!("{:?}|{:?}", guard.encode(), translated.encode());
                    if edges.contains(&edge) {
                        count = count + 1;
                        break;
                    }
                    edges.insert(edge);
                    guard = translated;
                }
            }

            grid.set_value(&point, val.clone());
        }
    }

    println!("#Loops: {}", count);

    // grid.print(|char| char.to_string());
}