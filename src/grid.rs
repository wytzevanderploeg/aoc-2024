use std::cmp;
use std::collections::HashSet;
use std::hash::Hash;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: Clone + Eq + Hash> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        let data = vec![vec![default_value; width]; height];
        Grid { data }
    }

    pub fn get_width(&self) -> usize {
        self.data.len()
    }

    pub fn get_height(&self) -> usize {
        self.data[0].len()
    }

    pub fn contains(&self, point: &(i32, i32)) -> bool {
        (point.0 >= 0 && point.0 < self.data.len() as i32) &&
            (point.1 >= 0 && point.1 < self.data[0].len() as i32)
    }

    pub fn get_value(&self, point: &Point) -> &T {
        &self.data[point.x][point.y]
    }

    pub fn set_value(&mut self, point: &Point, value: T) {
        self.data[point.x][point.y] = value;
    }

    pub fn visit(&self, mut visit: impl FnMut(&T, Point)) {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                visit(&self.data[x][y], Point{x, y});
            }
        }
    }

    pub fn find_first(&self, predicate: fn(&T) -> bool) -> Option<Point> {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if predicate(&self.data[x][y]) {
                    return Some(Point{x, y});
                }
            }
        }
        None
    }

    pub fn find_all<P>(&self, predicate: P) -> Vec<Point>
    where
        P: Fn(&T) -> bool,
    {
        let mut result: Vec<Point> = Vec::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if predicate(&self.data[x][y]) {
                    result.push(Point{x, y});
                }
            }
        }
        result
    }

    pub fn distinct(&self) -> HashSet<T> {
        let mut result: HashSet<T> = HashSet::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if !result.contains(&self.data[x][y]) {
                    result.insert(self.data[x][y].clone());
                }
            }
        }
        result
    }

    pub fn translate(&self, origin: &Point, translation: &Translation, wrap: bool) -> Option<Point> {
        let mut translated_x = origin.x as i32 + translation.x;
        let mut translated_y = origin.y as i32 + translation.y;

        if wrap {
            let width = self.get_width() as i32;
            let height = self.get_height() as i32;
            translated_x = ((translated_x % width) + width) % width;
            translated_y = ((translated_y % height) + height) % height;
        } else if !self.contains(&(translated_x, translated_y)) {
            return None;
        }

        Some(Point{x: translated_x as usize, y: translated_y as usize})
    }

    pub fn get_translated(&self, origin: &Point, translation: &Translation, wrap: bool) -> Option<T> {
        if let Some(point) = self.translate(origin, translation, wrap) {
            return Some(self.data[point.x][point.y].clone());
        }
        None
    }

    pub fn print<P>(&self, print: P)
    where
        P: Fn(&T) -> String,
    {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                print!("{}", print(&self.data[x][y]));
            }
            println!();
        }
    }
}

impl Grid<char> {
    pub fn from_string(data: &String) -> Self {
        let grid_size = get_grid_size(data);
        println!("{:?}", grid_size);
        let data: Vec<Vec<char>> = create_and_fill_grid(data, &grid_size);
        Grid { data }
    }
}

/// Negates the given translation i.e. mirroring
pub fn mirror(translation: &Translation) -> Translation {
    Translation{x: -translation.x, y: -translation.y}
}

/// Calculates Manhattan Distance
/// Result is based on the viewpoint of the origin
pub fn calculate_distance(origin: &Point, target: &Point) -> Translation {
    let d_x = target.x as i32 - origin.x as i32;
    let d_y = target.y as i32 - origin.y as i32;
    Translation{x: d_x, y: d_y}
}

/// Makes a translation absolute (positive)
pub fn make_absolute(translation: &Translation) -> Translation {
    Translation{x: translation.x.abs(), y: translation.y.abs()}
}

pub fn get_grid_size(input: &String) -> Dimension {
    let mut width = 0;
    let height = input.lines().count();
    for line in input.lines() {
        // println!("{}", line);
        width = cmp::max(width, line.len());
    }
    return Dimension{width, height};
}

pub fn create_and_fill_grid(input: &String, dimension: &Dimension) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; dimension.height]; dimension.width];
    let mut y = 0;
    for line in input.lines() {
        fill_row(&mut grid, line, y);
        y += 1;
    }
    grid
}

fn fill_row(grid: &mut Vec<Vec<char>>, line: &str, y: usize) {
    let chars: Vec<char> = line.chars().collect();
    for x in 0..chars.len() {
        // grid[x][y] = &line[x..x+1];
        grid[x][y] = chars[x];
    }
}

pub fn visit<T>(grid: &Vec<Vec<T>>, mut visit: impl FnMut(&T, Point)) {
    let width = grid.len();
    let height = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            visit(&grid[x][y], Point{x, y});
        }
    }
}

pub fn print<T>(grid: &Vec<Vec<T>>, print: fn(&T) -> String) {
    let width = grid.len();
    let height = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            print!("{}", print(&grid[x][y]));
        }
        println!();
    }
}

pub fn get_column<T>(grid: &Vec<Vec<T>>, idx: usize) -> Vec<&T> {
    let height = grid[0].len();

    let mut result: Vec<&T> = Vec::new();
    for i in 0..height {
        result.push(&grid[idx][i]);
    }
    result
}

pub fn columns<T>(grid: &Vec<Vec<T>>) -> Vec<(usize, Vec<&T>)> {
    let width = grid.len();

    let mut result: Vec<(usize, Vec<&T>)> = Vec::new();
    for i in 0..width {
        result.push((i, get_column(&grid, i)));
    }
    result
}

pub fn get_row<T>(grid: &Vec<Vec<T>>, idx: usize) -> Vec<&T> {
    let width = grid.len();

    let mut result: Vec<&T> = Vec::new();
    for i in 0..width {
        result.push(&grid[i][idx]);
    }
    result
}

pub fn rows<T>(grid: &Vec<Vec<T>>) -> Vec<(usize, Vec<&T>)> {
    let height = grid[0].len();
    let mut result: Vec<(usize, Vec<&T>)> = Vec::new();
    for i in 0..height {
        result.push((i, get_row(&grid, i)));
    }
    result
}

#[derive(Debug)]
pub struct Dimension {
    pub width: usize,
    pub height: usize
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Translation {
    pub x: i32,
    pub y: i32
}

// Cardinal directions
pub const UP: Translation = Translation { x: 0, y: -1 };
pub const DOWN: Translation = Translation { x: 0, y: 1 };
pub const LEFT: Translation = Translation { x: -1, y: 0 };
pub const RIGHT: Translation = Translation { x: 1, y: 0 };

// Wind directions (cardinal and intercardinal)
pub const NORTH: Translation = Translation { x: 0, y: -1 };
pub const SOUTH: Translation = Translation { x: 0, y: 1 };
pub const WEST: Translation = Translation { x: -1, y: 0 };
pub const EAST: Translation = Translation { x: 1, y: 0 };

pub const NORTHEAST: Translation = Translation { x: 1, y: -1 };
pub const NORTHWEST: Translation = Translation { x: -1, y: -1 };
pub const SOUTHEAST: Translation = Translation { x: 1, y: 1 };
pub const SOUTHWEST: Translation = Translation { x: -1, y: 1 };

impl Translation {
    pub fn all_cardinal_directions() -> [Translation; 4] {
        [NORTH, SOUTH, WEST, EAST]
    }

    pub fn all_wind_directions() -> [Translation; 8] {
        [NORTH, SOUTH, WEST, EAST, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST]
    }
}

impl Point {
    pub fn as_tuple(&self) -> (i32, i32) {
        (self.x.clone() as i32, self.y.clone() as i32)
    }

    pub fn encode(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}