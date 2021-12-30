use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[derive(Clone, Serialize, Deserialize)]
pub struct World {
  pub x: u32,
  pub y: u32,
  pub cells: Vec<Cell>,
}

// x: Position, y: Position, v: State
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell(pub u32, pub u32, pub u32);

#[allow(dead_code)]
pub fn apply_algorithm(world: World, algorithm: fn(World) -> World) -> World {
  let w = world.clone();
  return algorithm(w);
}

#[allow(dead_code)]
pub fn get_neighbors(
  world: World,
  nb: fn(world: World, range: (u32, u32)) -> Vec<Cell>,
  pos: (u32, u32),
) -> Vec<Cell> {
  return nb(world, pos);
}

/*
# von neumann neighborhood

  y\x |  -1  0  1
------+-------------
 -1   |   *  *  *
  0   |   *  o  *
  1   |   *  *  *

*/

// von neumann neighborhood
fn is_neighbor(dim: (u32, u32), c: Cell, focus: (u32, u32)) -> bool {
  let x_pos = c.0;
  let y_pos = c.1;
  let x_center = focus.0;
  let y_center = focus.1;
  let is_self = x_pos == x_center && y_pos == y_center;
  if is_self {
    return false;
  }
  let size = 1; // size of neighborhood search
  let x_dim = dim.0;
  let y_dim = dim.1;
  let is_w = x_pos == (x_dim + x_center - size) % x_dim;
  let is_e = x_pos == (x_dim + x_center + size) % x_dim;
  let is_n = y_pos == (y_dim + y_center - size) % y_dim;
  let is_s = y_pos == (y_dim + y_center + size) % y_dim;
  let is_nbr = is_w && (is_n || is_s || x_center == x_pos || y_center == y_pos)
    || is_e && (is_n || is_s || x_center == x_pos || y_center == y_pos)
    || (is_n && x_center == x_pos)
    || (is_s && x_center == x_pos);
  // print!(
  //   "{}x{} of {}x{} in dim {}x{}: ",
  //   x_pos, y_pos, x_center, y_center, x_dim, y_dim
  // );
  // if is_n {
  //   print!("N")
  // }
  // if is_s {
  //   print!("S")
  // }
  // if is_w {
  //   print!("W")
  // }
  // if is_e {
  //   print!("E")
  // }
  // println!(" - {}", is_nbr);

  // println!(
  //   "{}x{} is neighbor of {}x{} -> {}",
  //   x_pos, y_pos, x_center, y_center, n
  // );
  return is_nbr;
}

#[allow(dead_code)]
pub fn nb(world: World, focus: (u32, u32)) -> Vec<Cell> {
  // println!("{}@{}x{}", world.cells.len(), focus.0, focus.1);
  return world
    .cells
    .iter()
    .cloned()
    .filter(|&c| is_neighbor((world.x, world.y), c, focus))
    .collect();
}

#[allow(dead_code)]
// Survival is > 0, Death is 0
pub fn determine_survival(state: u32, neighbors: Vec<Cell>) -> u32 {
  if neighbors.len() == 0 {
    return 0;
  }
  let num_alive = neighbors
    .iter()
    .cloned()
    .map(|c| c.2)
    .reduce(|acc, c| c + acc)
    .unwrap();

  match state {
    0 => match num_alive {
      3 => 1,
      _ => 0,
    },
    _ => {
      return match num_alive {
        3 => 1,
        2 => 1,
        _ => 0,
      };
    }
  }
}

#[allow(dead_code)]
pub fn game_of_life(w: World) -> World {
  let cells: Vec<Cell> = w
    .cells
    .iter()
    .map(|c| {
      Cell(
        c.0,
        c.1,
        determine_survival(c.2, get_neighbors(w.clone(), nb, (c.0, c.1))),
      )
    })
    .collect();
  return World {
    x: w.x,
    y: w.y,
    cells: cells,
  };
}
