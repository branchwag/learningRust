# Old versions

Earlier iterations of the solver, kept here for reference. These aren't part
of the Cargo build; compile one directly with rustc if you want to run it,
e.g. `rustc v1_backtracking_count_all.rs -O -o v1 && ./v1 8`.

- **v1_backtracking_count_all.rs** — recursive backtracking using bitmasks
  for columns/diagonals. Explores the whole search tree and reports the total
  number of solutions for a given N, alongside the first one found. Capped at
  N=32 because the masks are `u32`.
- **v2_backtracking_stop_at_first.rs** — same bitmask backtracking, but
  returns as soon as it finds one valid placement instead of exhaustively
  counting all solutions.

Both were replaced by the closed-form O(n) construction in `src/main.rs`,
which doesn't search at all and scales to arbitrarily large boards.
