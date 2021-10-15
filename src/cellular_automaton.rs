use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[derive(Serialize, Deserialize)]
pub struct World {
  pub x: u32,
  pub y: u32,
  pub cells: [Cell; 9],
}

// x: Position, y: Position, v: State
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell(pub u32, pub u32, pub u32);
