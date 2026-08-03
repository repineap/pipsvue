# PipsVue

A browser implementation of the Pips domino puzzle, built with Vue 3 + TypeScript and Vite,
alongside `pips_rust` — a Rust crate that solves the same puzzles.

## The puzzle

You are given a set of dominoes and a board divided into coloured regions. Place every domino
on the board so that each region satisfies its constraint:

| Region  | Constraint                                    |
| ------- | --------------------------------------------- |
| `n`     | The pips in the region sum to exactly `n`      |
| `<n`    | The pips in the region sum to less than `n`    |
| `>n`    | The pips in the region sum to greater than `n` |
| `=`     | Every square in the region has the same value  |
| `!=`    | Every square in the region has a distinct value |
| (blank) | No constraint — any values are accepted        |

Dominoes are dragged onto the grid; regions revalidate on every drop, and the puzzle is solved
once all constrained regions are satisfied.

## Project layout

```
src/
  App.vue                     game state: placement, removal, region validation
  components/
    PipsGrid.vue              the board and its regions
    DraggableDomino.vue       drag/rotate interaction
    DominoSVG.vue             domino rendering
    types/domino.ts           Domino, Region, RegionType, Rotation
pips_rust/
  src/game_v2.rs              bitboard/graph representation of a puzzle
  src/solver.rs               depth-limited search over legal moves
  src/loader.rs               deserializes games.json
  benches/solver.rs           Criterion benchmarks
  games.json                  puzzle definitions
```

The Rust solver is currently a standalone crate — it is not yet wired into the front end,
though `vite-plugin-wasm` and the `wasm-bindgen` dependency are in place for that.

## Recommended IDE setup

[VS Code](https://code.visualstudio.com/) + [Vue (Official)](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

TypeScript cannot type `.vue` imports on its own, so `tsc` is replaced with `vue-tsc` for type
checking, and Volar makes the editor's language service aware of `.vue` types.

## Front end

The package manager is pnpm, pinned via `packageManager` in `package.json`. Enable it with
`corepack enable` if you don't already have it.

```sh
pnpm install
```

| Command            | Description                                          |
| ------------------ | ---------------------------------------------------- |
| `pnpm dev`         | Start the dev server with hot reload                  |
| `pnpm build`       | Type-check, compile and minify for production         |
| `pnpm build-only`  | Build without type checking                           |
| `pnpm type-check`  | Run `vue-tsc` only                                    |
| `pnpm preview`     | Serve the production build locally                    |
| `pnpm lint`        | Run oxlint and ESLint with `--fix`                    |
| `pnpm format`      | Format `src/` with oxfmt                              |

Requires Node `^20.19.0 || >=22.12.0`.

## Rust solver

Building requires a C linker. On a fresh Debian/Ubuntu (or WSL) install:

```sh
sudo apt-get install build-essential
```

Then, from `pips_rust/`:

```sh
cargo run --release     # profiling harness: solves games.json[2] at depth 3, 150 times
cargo bench             # Criterion benchmarks at search depths 3 and 4
```

`main.rs` is a profiling entry point rather than a CLI — it discards the solution via
`black_box`, so it produces no output.

`games.json` is read relative to the working directory, so run these from `pips_rust/`.
The release profile keeps debug symbols enabled for profiling.

## Browser devtools

- Chromium-based browsers (Chrome, Edge, Brave, etc.):
  - [Vue.js devtools](https://chromewebstore.google.com/detail/vuejs-devtools/nhdogjmejiglipccpnnnanhbledajbpd)
  - [Turn on Custom Object Formatter in Chrome DevTools](http://bit.ly/object-formatters)
- Firefox:
  - [Vue.js devtools](https://addons.mozilla.org/en-US/firefox/addon/vue-js-devtools/)
  - [Turn on Custom Object Formatter in Firefox DevTools](https://fxdx.dev/firefox-devtools-custom-object-formatters/)

## Configuration

See the [Vite Configuration Reference](https://vite.dev/config/).
