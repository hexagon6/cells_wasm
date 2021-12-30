use getrandom::getrandom;

use wasm_bindgen::prelude::*;
mod cellular_automaton;

use cellular_automaton::{apply_algorithm, game_of_life, Cell, World};

pub fn init_world9(cells: [Cell; 9]) -> World {
    const X: u32 = 3;
    const Y: u32 = 3;
    World {
        x: X,
        y: Y,
        cells: cells.to_vec(),
    }
}

pub fn init_world16(cells: [Cell; 16]) -> World {
    const X: u32 = 4;
    const Y: u32 = 4;
    World {
        x: X,
        y: Y,
        cells: cells.to_vec(),
    }
}

pub fn init_world81(cells: [Cell; 81]) -> World {
    const X: u32 = 3;
    const Y: u32 = 3;
    World {
        x: X,
        y: Y,
        cells: cells.to_vec(),
    }
}

pub fn init_random_cells81() -> [Cell; 9 * 9] {
    const LEN: u32 = 9;
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

pub fn init_random_cells16() -> [Cell; 4 * 4] {
    const LEN: u32 = 4;
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

pub fn init_random_cells64() -> [Cell; 8 * 8] {
    const LEN: u32 = 8;
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
    let cells: [Cell; 81] = init_random_cells81();
    let world = World {
        x: 9,
        y: 9,
        cells: cells.to_vec(),
    };

    JsValue::from_serde(&world).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cellular_automaton::*;

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
        let cells: [Cell; 9 * 9] = init_random_cells81();
        let world: World = init_world81(cells);
        assert_eq!(world.x, 3);
        assert_eq!(world.y, 3);
        assert_eq!(world.cells.len(), 81);
        assert_eq!(world.cells[0].0, 0);
        assert_eq!(world.cells[0].1, 0);
        let v0 = world.cells[0].2;
        matches_value(v0);
        assert_eq!(world.cells[1].0, 0);
        assert_eq!(world.cells[1].1, 1);
        let v1 = world.cells[1].2;
        matches_value(v1);
        assert_eq!(world.cells[9].0, 1);
        assert_eq!(world.cells[9].1, 0);
        assert_eq!(world.cells[10].0, 1);
        assert_eq!(world.cells[10].1, 1);
        assert_eq!(world.cells[18].0, 2);
        assert_eq!(world.cells[18].1, 0);
        let v18 = world.cells[18].2;
        matches_value(v18);
    }
}
