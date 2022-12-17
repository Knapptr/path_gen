mod grid;
use std::collections::{HashSet, VecDeque};

use grid::{Cell, Coords, Grid};

fn main() {
    let grid = Grid::new(5, 5);
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
