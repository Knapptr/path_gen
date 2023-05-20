use std::fmt::{self, Display, Formatter};
const WALL_CHAR: &str = "#";
const OPEN_CHAR: &str = " ";

#[derive(Clone)]
pub enum Wall {
    Empty,
    Void,
    Filled,
}
pub struct Cell {
    coords: Coords,
    walls: [Wall; 4],
    revealed: bool,
}
impl Cell {
    fn init(x: isize, y: isize) -> Self {
        Self {
            coords: Coords::from(x, y),
            revealed: false,
            walls: [Wall::Filled, Wall::Filled, Wall::Filled, Wall::Filled],
        }
    }
    fn render_wall(&self, wall_at: usize) -> String {
        let mut str = String::new();
        match self.walls[wall_at] {
            Wall::Empty => str.push_str(OPEN_CHAR),
            Wall::Filled => str.push_str(WALL_CHAR),
            Wall::Void => str.push_str(WALL_CHAR),
        }
        str
    }
    fn render_center(&self) -> String {
        match self.revealed {
            true => String::from(OPEN_CHAR),
            false => String::from(WALL_CHAR),
        }
    }
    // top, right, bottom, left
    pub fn open(&mut self, wall_at: usize) {
        self.walls[wall_at] = Wall::Empty
        // open neighbors wall if it exists
        // self.coords.get_wall_neighbor()
    }
    pub fn reveal(&mut self) {
        self.revealed = true;
    }
    fn wall_str(&self, wall_at: usize) -> String {
        let mut str = String::new();
        match wall_at {
            0 | 2 => {
                str.push_str(WALL_CHAR);
                str.push_str(&self.render_wall(wall_at));
                str.push_str(WALL_CHAR); // diagonal
            }
            1 => {
                str.push_str(&self.render_wall(3));
                str.push_str(&self.render_center());
                str.push_str(&self.render_wall(1));
            }
            _ => unreachable!(),
        }
        str
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();
        for cell_row in 0..3 {
            str.push_str(&self.wall_str(cell_row));
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    pub width: isize,
    pub height: isize,
}
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();

        // Go row by row of cells
        for row_num in 0..self.height {
            for wall_idx in 0..3 {
                for cell in self
                    .cells
                    .iter()
                    .skip((row_num * self.width) as usize)
                    .take(self.width as usize)
                {
                    str.push_str(&cell.wall_str(wall_idx))
                }
                str.push('\n'); // end of the wall
            }
            // str.push('\n'); // end of the row
        }
        write!(f, "{}", str)
    }
}
impl Grid {
    pub fn new(width: isize, height: isize) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::init(x, y))
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }
    pub fn reveal_at(&mut self, at: &Coords) {
        self.get_at_mut(at).unwrap().reveal();
    }
    pub fn get_unvisited_neighbors_at(&self, coords: &Coords) -> Option<Vec<Coords>> {
        let nbor_coords = coords.get_all_neighbors(self);
        let mut unvisited_nbors = Vec::new();
        for nbor_coord in nbor_coords {
            if let Some(nbor) = self.get_at(&nbor_coord) {
                if !nbor.revealed {
                    unvisited_nbors.push(nbor_coord)
                }
            }
        }
        if unvisited_nbors.len() < 1 {
            return None;
        }
        Some(unvisited_nbors)
    }

    pub fn get_at(&self, coords: &Coords) -> Option<&Cell> {
        self.cells.get(coords.to_index(self.width))
    }
    pub fn get_at_mut(&mut self, coords: &Coords) -> Option<&mut Cell> {
        self.cells.get_mut(coords.to_index(self.width))
    }
    pub fn open_wall_at(&mut self, coords_a: &Coords, wall_number: usize) {
        let coords_b_opt = coords_a.get_wall_neighbor_coords(wall_number, self);
        match coords_b_opt {
            Some(coords_b) => {
                let (wall_num_a, wall_num_b) = get_wall_numbers(coords_a, &coords_b);
                let cell_a = self.get_at_mut(coords_a).unwrap();
                cell_a.reveal();
                cell_a.open(wall_num_a);
                let cell_b = self.get_at_mut(&coords_b).unwrap();
                cell_b.reveal();
                cell_b.open(wall_num_b);
            }
            None => panic!(),
        }
    }
}

fn get_wall_numbers(a: &Coords, b: &Coords) -> (usize, usize) {
    let dx = (a.x - b.x).signum();
    let dy = (a.y - b.y).signum();
    match (dx, dy) {
        (0, 1) => (0, 2),
        (0, -1) => (2, 0),
        (1, 0) => (3, 1),
        (-1, 0) => (1, 3),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coords {
    x: isize,
    y: isize,
}
impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }
}
impl Coords {
    pub fn from(x: isize, y: isize) -> Self {
        Coords { x, y }
    }
    fn to_index(&self, width: isize) -> usize {
        let y = self.y * width;
        (self.x + y) as usize
    }
    pub fn wall_to(&self, other: &Coords) -> usize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        if dx == 1 {
            return 3;
        };
        if dx == -1 {
            return 1;
        };
        if dy == 1 {
            return 0;
        };
        if dy == -1 {
            return 2;
        };
        unreachable!()
    }
    pub fn get_wall_neighbor_coords(&self, wall_number: usize, grid: &Grid) -> Option<Coords> {
        match wall_number {
            0 => {
                if self.y == 0 {
                    return None;
                }
                Some(Coords::from(self.x, self.y - 1))
            }
            1 => {
                if self.x == grid.width - 1 {
                    return None;
                }
                Some(Coords::from(self.x + 1, self.y))
            }
            2 => {
                if self.y == grid.height - 1 {
                    return None;
                }
                Some(Coords::from(self.x, self.y + 1))
            }
            3 => {
                if self.x == 0 {
                    return None;
                }
                Some(Coords::from(self.x - 1, self.y))
            }
            _ => panic!("Not a wall direction"),
        }
    }
    pub fn get_all_neighbors(&self, grid: &Grid) -> Vec<Coords> {
        let mut nbors = self.get_x_neighbors(grid.width);
        nbors.append(&mut self.get_y_neighbors(grid.height));
        nbors
    }
    pub fn get_x_neighbors(&self, width: isize) -> Vec<Coords> {
        let mut nbors = Vec::new();
        if self.x % width > 0 {
            nbors.push(Coords::from(self.x - 1, self.y))
        }
        if self.x % width < width - 1 {
            nbors.push(Coords::from(self.x + 1, self.y))
        }
        nbors
    }
    pub fn get_y_neighbors(&self, height: isize) -> Vec<Coords> {
        let mut nbors = Vec::new();
        if self.y % height > 0 {
            nbors.push(Coords::from(self.x, self.y - 1))
        }
        if self.y % height < height - 1 {
            nbors.push(Coords::from(self.x, self.y + 1))
        }
        nbors
    }
}

#[cfg(test)]
#[test]
fn neighbors_x() {
    let non_x_border_coords = Coords::from(1, 0);
    let x_border_coords = Coords::from(0, 0);
    assert_eq!(
        non_x_border_coords.get_x_neighbors(10),
        vec![Coords::from(0, 0), Coords::from(2, 0)]
    );
    assert_eq!(
        x_border_coords.get_x_neighbors(10),
        vec![Coords::from(1, 0)]
    )
}
#[test]
fn neighbors_y() {
    let non_y_border_coords = Coords::from(0, 1);
    let y_border_coords = Coords::from(0, 0);
    assert_eq!(
        non_y_border_coords.get_y_neighbors(10),
        vec![Coords::from(0, 0), Coords::from(0, 2)]
    );
    assert_eq!(
        y_border_coords.get_y_neighbors(10),
        vec![Coords::from(0, 1)]
    )
}
#[test]
fn to_idx() {
    let idx = Coords::from(0, 1).to_index(2);

    assert_eq!(idx, 2);
}
#[test]
fn disp_cell() {
    let mut cell = Cell::init(0, 0);
    cell.reveal();
    assert_eq!(format!("{}", cell), "###\n#.#\n###\n")
}
#[test]
fn disp_cell_top() {
    let mut cell = Cell::init(0, 0);
    cell.open(0);
    assert_eq!(format!("{}", cell), "#.#\n###\n###\n")
}
#[test]
fn disp_grid() {
    let expected_string = "######\n#.##.#\n######\n######\n#.##.#\n######\n";
    let mut grid = Grid::new(2, 2);
    for cell in &mut grid.cells {
        cell.reveal();
    }
    assert_eq!(format!("{}", grid), expected_string)
}
