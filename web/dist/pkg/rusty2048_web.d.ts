/* tslint:disable */
/* eslint-disable */
/**
 * Initialize panic hook for better error messages
 */
export function init_panic_hook(): void;
export class Rusty2048Web {
  free(): void;
  constructor();
  make_move(direction: string): boolean;
  get_board(): any;
  get_score(): any;
  get_state(): string;
  new_game(): void;
  undo(): void;
  get_moves(): number;
  get_stats(): any;
  get_theme(): any;
  set_theme(theme_name: string): void;
  get_available_themes(): any;
  get_max_tile(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_rusty2048web_free: (a: number, b: number) => void;
  readonly rusty2048web_new: () => [number, number, number];
  readonly rusty2048web_make_move: (a: number, b: number, c: number) => [number, number, number];
  readonly rusty2048web_get_board: (a: number) => [number, number, number];
  readonly rusty2048web_get_score: (a: number) => [number, number, number];
  readonly rusty2048web_get_state: (a: number) => [number, number, number, number];
  readonly rusty2048web_new_game: (a: number) => [number, number];
  readonly rusty2048web_undo: (a: number) => [number, number];
  readonly rusty2048web_get_moves: (a: number) => number;
  readonly rusty2048web_get_stats: (a: number) => [number, number, number];
  readonly rusty2048web_get_theme: (a: number) => [number, number, number];
  readonly rusty2048web_set_theme: (a: number, b: number, c: number) => [number, number];
  readonly rusty2048web_get_available_themes: (a: number) => [number, number, number];
  readonly rusty2048web_get_max_tile: (a: number) => number;
  readonly init_panic_hook: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
