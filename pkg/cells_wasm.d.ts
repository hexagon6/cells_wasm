/* tslint:disable */
/* eslint-disable */
/**
* @returns {any}
*/
export function random_world(): any;
/**
* @param {any} world
* @returns {any}
*/
export function run_game_of_life(world: any): any;
/**
*/
export class Cell {
  free(): void;
/**
*/
  0: number;
/**
*/
  1: number;
/**
*/
  2: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_cell_free: (a: number) => void;
  readonly __wbg_get_cell_0: (a: number) => number;
  readonly __wbg_set_cell_0: (a: number, b: number) => void;
  readonly __wbg_get_cell_1: (a: number) => number;
  readonly __wbg_set_cell_1: (a: number, b: number) => void;
  readonly __wbg_get_cell_2: (a: number) => number;
  readonly __wbg_set_cell_2: (a: number, b: number) => void;
  readonly random_world: () => number;
  readonly run_game_of_life: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
