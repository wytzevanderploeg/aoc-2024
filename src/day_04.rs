use crate::{grid, util};
use crate::grid::{Grid, Translation};

pub fn part_one() {
    let input = util::read(2024, 4, false);

    let grid_size = grid::get_grid_size(&input);
    println!("{:?}", grid_size);
    let grid = grid::create_and_fill_grid(&input, &grid_size);

    grid::print(&grid, |val| val.to_string());

    let directions: [(i32, i32); 8] = [
        (1, 0), // east
        (1, 1), // south-east
        (0, 1), // south
        (-1, 1), // south-west
        (-1, 0), // west
        (-1, -1), // north-west
        (0, -1), // north
        (1, -1), // north-east
    ];
    let chars: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;

    grid::visit(&grid, |val, point| {
        if val != &chars[0] {
            return;
        }

        for direction in directions {
            let mut idx = 1;
            let mut neighbor_x = point.x.clone() as i32;
            let mut neighbor_y = point.y.clone() as i32;

            while idx < chars.len() {
                neighbor_x = neighbor_x + direction.0;
                neighbor_y = neighbor_y + direction.1;
                if neighbor_x < 0 || neighbor_x >= grid.len() as i32 || neighbor_y < 0 || neighbor_y >= grid[0].len() as i32 {
                    break;
                }
                let val = grid[neighbor_x as usize][neighbor_y as usize];
                if val == chars[idx] {
                    idx += 1;
                } else {
                    break;
                }
            }
            if idx == chars.len() {
                count += 1;
            }
        }
    });
    println!("Found: {}", count);
}

pub fn part_two() {
    let input = util::read(2024, 4, false);

    let grid = Grid::from_string(&input);
    // grid.print(|val| val.to_string());

    let north_west = Translation{ x: -1, y: -1 };
    let south_east = grid::mirror(&north_west);
    let south_west = Translation{ x: -1, y: 1 };
    let north_east = grid::mirror(&south_west);

    let mut count = 0;

    grid.visit(|val, point| {
        if *val != 'A' {
            return;
        }

        let nw_option = grid.get_translated(&point, &north_west, false);
        let se_option = grid.get_translated(&point, &south_east, false);
        let sw_option = grid.get_translated(&point, &south_west, false);
        let ne_option = grid.get_translated(&point, &north_east, false);

        if valid_axis(&nw_option, &se_option) && valid_axis(&sw_option, &ne_option) {
            println!("Found: {:?}", point);
            count += 1;
        }
    });
    println!("Found: {}", count);
}

fn valid_axis(left: &Option<char>, right: &Option<char>) -> bool {
    match (left, right) {
        (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
        _ => false,
    }
}