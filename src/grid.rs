use std::fmt::{self, Display, Formatter};
const WALL_CHAR: &str = "#";
const OPEN_CHAR: &str = ".";

#[derive(Clone)]
pub enum Wall {
    Empty,
    Void,
    Filled,
}
#[derive(Clone)]
pub struct Cell {
    walls: [Wall; 4],
    revealed: bool,
}
impl Cell {
    fn init() -> Self {
        Self {
            revealed: false,
            walls: [Wall::Filled, Wall::Filled, Wall::Filled, Wall::Filled],
        }
    }
    // top, right, bottom, left
    fn open(&mut self, wall_at: usize) {
        self.walls[wall_at] = Wall::Empty
    }
    fn reveal(&mut self) {
        self.revealed = true;
    }
    fn wall_str(&self, wall_at: usize) -> String {
        let mut str = String::from(WALL_CHAR); // init w diag
        match self.walls[wall_at] {
            Wall::Empty => str.push_str(OPEN_CHAR),
            Wall::Filled => str.push_str(WALL_CHAR),
            Wall::Void => str.push_str(WALL_CHAR),
        }
        str.push_str(WALL_CHAR); // diagonal
        str
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "###\n#.#\n###")
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
        let cells = vec![Cell::init(); (width * height) as usize];
        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn get_at_mut(&mut self, coords: &Coords) -> Option<&mut Cell> {
        self.cells.get_mut(coords.to_index(self.width))
    }
}

#[derive(Debug)]
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
        let x = y + self.x;
        (x + y) as usize
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
