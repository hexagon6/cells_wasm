use getrandom::getrandom;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

// x: Position, y: Position, v: State
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell(pub u32, pub u32, pub u32);

#[derive(Serialize, Deserialize)]
pub struct World {
    pub x: u32,
    pub y: u32,
    pub cells: [Cell; 9],
}

pub fn init_world9(cells: [Cell; 9]) -> World {
    const X: u32 = 3;
    const Y: u32 = 3;
    World {
        x: X,
        y: Y,
        cells: cells,
    }
}

pub fn init_random_cells9() -> [Cell; 3 * 3] {
    const LEN: u32 = 3;
    const SIZE: u32 = LEN.pow(2) as u32;
    let mut index = 0;
    let cells: [Cell; SIZE as usize] = [0; SIZE as usize].map(|_| {
        let x = index / LEN;
        let y = index % LEN;
        let r = get_random_buf().unwrap();
        let v: u32 = (r[0] % 2).into();
        index = index + 1;
        Cell(x, y, v)
    });
    cells
}

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom(&mut buf)?;
    Ok(buf)
}

#[wasm_bindgen]
pub fn random_world() -> JsValue {
    let cells: [Cell; 9] = init_random_cells9();
    let world = World {
        x: 3,
        y: 3,
        cells: cells,
    };

    JsValue::from_serde(&world).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matches_value(v: u32) {
        match v {
            0 => assert_eq!(v, 0),
            1 => assert_eq!(v, 1),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_cell() {
        let c1: Cell = Cell(0, 0, 0);
        assert_eq!(c1.0, 0);
        assert_eq!(c1.1, 0);
        assert_eq!(c1.2, 0);
        let c2: Cell = Cell(0, 0, 1);
        assert_eq!(c2.2, 1);
        let c3: Cell = Cell(0, 1, 0);
        assert_eq!(c3.1, 1);
        let Cell(_, _, state) = c1;
        assert_eq!(state, 0);
    }
    #[test]
    fn update_cell_value() {
        let mut c1: Cell = Cell(0, 0, 0);
        assert_eq!(c1.2, 0);
        c1.2 = 1;
        assert_eq!(c1.2, 1);
    }
    #[test]
    fn init_random_world() {
        let cells: [Cell; 9] = init_random_cells9();
        let world: World = init_world9(cells);
        assert_eq!(world.x, 3);
        assert_eq!(world.y, 3);
        assert_eq!(world.cells.len(), 9);
        assert_eq!(world.cells[0].0, 0);
        assert_eq!(world.cells[0].1, 0);
        let v0 = world.cells[0].2;
        matches_value(v0);
        assert_eq!(world.cells[1].0, 0);
        assert_eq!(world.cells[1].1, 1);
        let v1 = world.cells[1].2;
        matches_value(v1);
        assert_eq!(world.cells[3].0, 1);
        assert_eq!(world.cells[3].1, 0);
        assert_eq!(world.cells[4].0, 1);
        assert_eq!(world.cells[4].1, 1);
        assert_eq!(world.cells[8].0, 2);
        assert_eq!(world.cells[8].1, 2);
        let v8 = world.cells[8].2;
        matches_value(v8);
    }
}
