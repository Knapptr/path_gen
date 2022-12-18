mod grid;
use std::collections::{HashSet, VecDeque};

use grid::{Cell, Coords, Grid};
use rand::seq::SliceRandom;

fn main() {
    // let mut stack = Vec::new();
    let mut rng = rand::thread_rng();
    let mut grid = Grid::new(15, 10);

    let mut current_coords = Coords::from(0, 0);
    grid.reveal_at(&current_coords);
    while let Some(neighbors) = grid.get_unvisited_neighbors_at(&current_coords) {
        let random_neighbor = neighbors.choose(&mut rng).unwrap();
        let wall_number = current_coords.wall_to(random_neighbor);
        grid.open_wall_at(&current_coords, wall_number);
        grid.reveal_at(random_neighbor);
        current_coords = *random_neighbor;
    }

    println!("{}", grid);
}

/*
   Randomly select a node (or cell) N.
   Push the node N onto a queue Q.
   Mark the cell N as visited.
   Randomly select an adjacent cell A of node N that has not been visited. If all the neighbors of N have been visited:
       Continue to pop items off the queue Q until a node is encountered with at least one non-visited neighbor - assign this node to N and go to step 4.
       If no nodes exist: stop.
   Break the wall between N and A.
   Assign the value A to N.
   Go to step 2.
*/
