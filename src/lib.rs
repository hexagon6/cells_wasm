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
    const N: usize = 8;
    let cells: [Cell; N * N] = init_random_cells64();
    let world = World {
        x: N as u32,
        y: N as u32,
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

    #[test]
    fn apply_naive_algorithm() {
        let cells = [
            Cell(0, 0, 0),
            Cell(0, 1, 0),
            Cell(0, 2, 0),
            Cell(1, 0, 0),
            Cell(1, 1, 0),
            Cell(1, 2, 0),
            Cell(2, 0, 0),
            Cell(2, 1, 0),
            Cell(2, 2, 0),
        ];
        let world = init_world9(cells);
        fn naive_algorithm(world: World) -> World {
            return World {
                x: world.x,
                y: world.y,
                cells: world.cells.to_vec(),
            };
        }
        assert_eq!(world.x, 3);
        assert_eq!(world.y, 3);
        assert_eq!(world.cells[0].0, 0);
        assert_eq!(world.cells[0].1, 0);
        assert_eq!(world.cells[0].2, 0);
        assert_eq!(world.cells[1].0, 0);
        assert_eq!(world.cells[1].1, 1);
        assert_eq!(world.cells[1].2, 0);
        assert_eq!(world.cells[2].0, 0);
        assert_eq!(world.cells[2].1, 2);
        assert_eq!(world.cells[2].2, 0);
        assert_eq!(world.cells[3].0, 1);
        assert_eq!(world.cells[3].1, 0);
        assert_eq!(world.cells[3].2, 0);
        assert_eq!(world.cells[4].0, 1);
        assert_eq!(world.cells[4].1, 1);
        assert_eq!(world.cells[4].2, 0);
        assert_eq!(world.cells[5].0, 1);
        assert_eq!(world.cells[5].1, 2);
        assert_eq!(world.cells[5].2, 0);
        assert_eq!(world.cells[6].0, 2);
        assert_eq!(world.cells[6].1, 0);
        assert_eq!(world.cells[6].2, 0);
        assert_eq!(world.cells[7].0, 2);
        assert_eq!(world.cells[7].1, 1);
        assert_eq!(world.cells[7].2, 0);
        assert_eq!(world.cells[8].0, 2);
        assert_eq!(world.cells[8].1, 2);
        assert_eq!(world.cells[8].2, 0);
        let outcome = apply_algorithm(world, naive_algorithm);
        assert_eq!(outcome.x, 3);
        assert_eq!(outcome.y, 3);
        assert_eq!(outcome.cells[0].0, 0);
        assert_eq!(outcome.cells[0].1, 0);
        assert_eq!(outcome.cells[0].2, 0);
        assert_eq!(outcome.cells[1].0, 0);
        assert_eq!(outcome.cells[1].1, 1);
        assert_eq!(outcome.cells[1].2, 0);
        assert_eq!(outcome.cells[2].0, 0);
        assert_eq!(outcome.cells[2].1, 2);
        assert_eq!(outcome.cells[2].2, 0);
        assert_eq!(outcome.cells[3].0, 1);
        assert_eq!(outcome.cells[3].1, 0);
        assert_eq!(outcome.cells[3].2, 0);
        assert_eq!(outcome.cells[4].0, 1);
        assert_eq!(outcome.cells[4].1, 1);
        assert_eq!(outcome.cells[4].2, 0);
        assert_eq!(outcome.cells[5].0, 1);
        assert_eq!(outcome.cells[5].1, 2);
        assert_eq!(outcome.cells[5].2, 0);
        assert_eq!(outcome.cells[6].0, 2);
        assert_eq!(outcome.cells[6].1, 0);
        assert_eq!(outcome.cells[6].2, 0);
        assert_eq!(outcome.cells[7].0, 2);
        assert_eq!(outcome.cells[7].1, 1);
        assert_eq!(outcome.cells[7].2, 0);
        assert_eq!(outcome.cells[8].0, 2);
        assert_eq!(outcome.cells[8].1, 2);
        assert_eq!(outcome.cells[8].2, 0);
    }

    #[test]
    fn get_neighbors9_test() {
        let cells = [
            Cell(0, 0, 0),
            Cell(0, 1, 0),
            Cell(0, 2, 0),
            Cell(1, 0, 0),
            Cell(1, 1, 0),
            Cell(1, 2, 0),
            Cell(2, 0, 0),
            Cell(2, 1, 0),
            Cell(2, 2, 0),
        ];
        let world = World {
            x: 3,
            y: 3,
            cells: cells.to_vec(),
        };

        let neighbors = get_neighbors(world, nb, (1, 1));
        assert_eq!(neighbors.len(), 8);
        // for c in neighbors {
        //     println!("{}x{}: {}", c.0, c.1, c.2);
        // }
    }

    /*

      0  1 2  3
    0 NW N NE *       * * * *
    1 W  x E  *   ->  * o o *
    2 SW S SE *   ->  * o O *
    3 *  * *  *       * * * *

    */
    #[test]
    fn get_neighbors_test16() {
        let cells = [
            Cell(0, 0, 0),
            Cell(0, 1, 0),
            Cell(0, 2, 0),
            Cell(0, 3, 0),
            Cell(1, 0, 0),
            Cell(1, 1, 0),
            Cell(1, 2, 0),
            Cell(1, 3, 0),
            Cell(2, 0, 0),
            Cell(2, 1, 0),
            Cell(2, 2, 0),
            Cell(2, 3, 0),
            Cell(3, 0, 0),
            Cell(3, 1, 0),
            Cell(3, 2, 0),
            Cell(3, 3, 0),
        ];
        let world = World {
            x: 4,
            y: 4,
            cells: cells.to_vec(),
        };

        let neighbors = get_neighbors(world, nb, (1, 1));
        assert_eq!(neighbors.len(), 8);
        // println!("neighbors: {}", neighbors.len());
        // for c in neighbors {
        //     println!("{}x{}: {}", c.0, c.1, c.2);
        // }
    }


    #[test]
    fn all_dead() {
        let cells = [Cell(0, 0, 0)].to_vec();
        let result = determine_survival(0, cells);
        assert_eq!(result, 0);
    }

    #[test]
    fn some_dead() {
        let cells = [Cell(0, 0, 0), Cell(0, 0, 2)].to_vec();
        let result = determine_survival(0, cells);
        assert_eq!(result, 0);
    }

    #[test]
    fn all_alive() {
        let cells = [Cell(0, 0, 1), Cell(1, 1, 1)].to_vec();
        let result = determine_survival(1, cells);
        assert_eq!(result, 1);
    }

    #[test]
    fn alive() {
        let cells = [Cell(0, 0, 1)].to_vec();
        let result = determine_survival(1, cells);
        assert_eq!(result, 0);
    }

    #[test]
    fn something() {
        let cells = [Cell(0, 0, 10)].to_vec();
        let result = determine_survival(1, cells);
        assert_eq!(result, 0);
    }

    #[test]
    fn something2() {
        let cells = [Cell(0, 0, 10)].to_vec();
        let result = determine_survival(0, cells);
        assert_eq!(result, 0);
    }

    /*

             Empty

        * * *       * * *
        * o *   ->  * * *
        * * *       * * *

    */

    #[test]
    fn game_of_life_empty() {
        let cells = [
            Cell(0, 0, 0),
            Cell(0, 1, 0),
            Cell(0, 2, 0),
            Cell(1, 0, 0),
            Cell(1, 1, 1),
            Cell(1, 2, 0),
            Cell(2, 0, 0),
            Cell(2, 1, 0),
            Cell(2, 2, 0),
        ];
        let world = init_world9(cells);
        assert_eq!(world.cells.len(), 9);
        assert_eq!(world.cells[4].2, 1);
        let next = apply_algorithm(world, game_of_life);
        assert_eq!(next.cells.len(), 9);
        assert_eq!(next.cells[4].2, 0);
    }

    /*

             Still life

        * * * *       * * * *
        * o o *   ->  * o o *
        * o * *   ->  * o O *
        * * * *       * * * *


    */

    #[test]
    fn game_of_life_still() {
        let cells = [
            Cell(0, 0, 0),
            Cell(0, 1, 0),
            Cell(0, 2, 0),
            Cell(0, 3, 0),
            Cell(1, 0, 0),
            Cell(1, 1, 1),
            Cell(1, 2, 1),
            Cell(1, 3, 0),
            Cell(2, 0, 0),
            Cell(2, 1, 1),
            Cell(2, 2, 0),
            Cell(2, 3, 0),
            Cell(3, 0, 0),
            Cell(3, 1, 0),
            Cell(3, 2, 0),
            Cell(3, 3, 0),
        ];
        let world = init_world16(cells);
        assert_eq!(world.cells.len(), 16);
        assert_eq!(world.cells[10].2, 0);
        let next = apply_algorithm(world, game_of_life);
        assert_eq!(next.cells.len(), 16);
        assert_eq!(next.cells[10].2, 1);
    }
}
