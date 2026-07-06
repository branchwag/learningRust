# nqueens

Solves the N-Queens problem in Rust: place N queens on an N×N board so that
no two attack each other.

Instead of a backtracking search, this places all N queens directly using a
closed-form construction (Hoffman, Loessi & Moore, 1969) — split `1..=n`
into evens and odds, concatenate them, and apply a small fixed rearrangement
when `n mod 6` is 2 or 3 to avoid diagonal clashes. No recursion, no search,
just one O(n) pass. `n = 2` and `n = 3` are the only sizes with no solution.

## Usage

```
cargo run -- <n>
```

Defaults to `n = 8` if no argument is given.

```
$ cargo run -- 8
Found a solution for N = 8:
. . . . . Q . .
Q . . . . . . .
. . . . Q . . .
. Q . . . . . .
. . . . . . . Q
. . Q . . . . .
. . . . . . Q .
. . . Q . . . .
```

## Tests

```
cargo test
```

Covers the trivial `n = 0`/`n = 1` cases, confirms `n = 2`/`n = 3` correctly
report no solution, validates every size from 1 to 500 for row/column/
diagonal conflicts, checks known sequences from the source construction, and
confirms `n = 100,000` still resolves instantly.

## old/

Earlier recursive-backtracking versions of the solver, kept for reference —
see [`old/README.md`](old/README.md) for details on why they were replaced.
