# rayoff

Feature-gate [Rayon](https://github.com/rayon-rs/rayon) usage in a Rust project.

`rayoff` is a `#![no_std]` macro library that provides a single unified iterator-chain syntax that compiles to either sequential (`core::iter`) or parallel (`rayon::iter`) execution depending on whether the `rayon` feature flag is enabled. Switching between sequential and parallel iteration requires no changes to call sites.

## How it works

The central primitive is the `frames!` macro, which takes an iterable expression and a chain of iterator adapter names (with optional closures), then dispatches to the appropriate iterator trait:

- **Without `rayon` feature:** expands to `core::iter::IntoIterator::into_iter(expr).adapter(...)`
- **With `rayon` feature:** expands to `rayon::iter::IntoParallelIterator::into_par_iter(expr).adapter(...)`

The `rayon`-enabled branch contains a dead `if false` arm that invokes the sequential path. This ensures the sequential types still typecheck (and that the closure types are inferred correctly), while the parallel path is what actually executes.

## Macros

All macros are `#[macro_export]` and available at the crate root.

### `frames!`

The low-level dispatch macro. Accepts an iterable and a comma-separated list of adapter steps. Each step is either a bare method name or a method name with a closure argument using `:|pat| expr` syntax.

```rust
// Without rayon feature: sequential
// With rayon feature: parallel
frames!(collection => filter:|x| *x > 0, map:|x| x * 2, collect)
```

### `map!`

Shorthand for `frames!(expr => map:|pat| body, collect)`. Returns a collected sequence.

```rust
let doubled: Vec<_> = map!(items => |x| x * 2);
```

### `sum!`

Shorthand for `frames!(expr => map:|pat| body, sum)`. Maps then sums.

```rust
let total: i32 = sum!(items => |x| x.cost);
```

### `any!`

Shorthand for `frames!(expr => any:|pat| body)`. Short-circuits when a predicate matches.

```rust
let found: bool = any!(items => |x| x.is_valid());
```

### `flat_map!`

Shorthand for `frames!(expr => flat_map:|pat| body, collect)`. Returns a collected flattened sequence.

```rust
let flat: Vec<_> = flat_map!(groups => |g| g.items);
```

### `enumerate_map!`

Shorthand for `frames!(expr => enumerate, map:|pat| body, collect)`. Enumerates the iterator before mapping, so the closure receives `(usize, T)` tuples via the pattern.

```rust
let indexed: Vec<_> = enumerate_map!(items => |(i, x)| (i, x.name.clone()));
```

## Feature flags

| Feature | Effect |
|---------|--------|
| `rayon` | Pulls in `rayon 1.11` and switches all `frames!`-based iteration to `into_par_iter`. Also re-exports the `rayon` crate as `rayoff::rayon`. |

Without the feature, the only dependency is `core` (re-exported as `rayoff::core`).

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
rayoff = { version = "0.1.0", git = "https://github.com/portal-co/rayoff.git" }

# Enable parallel iteration:
# rayoff = { version = "0.1.0", git = "https://github.com/portal-co/rayoff.git", features = ["rayon"] }
```

If you want to propagate the feature flag from your own crate:

```toml
[features]
rayon = ["rayoff/rayon"]
```

## Limitations

- The adapter names in `frames!` must exist on both `Iterator` and `rayon::iter::ParallelIterator` / `rayon::iter::IndexedParallelIterator`. Adapters that exist on one but not the other will fail to compile when the feature is toggled.
- `enumerate` is only available in the parallel path via `IndexedParallelIterator`, so the iterable must implement `rayon::iter::IndexedParallelIterator` when using `enumerate_map!` with the `rayon` feature.
- There are no explicit tests in the source tree; correctness relies on the type system and the dual-arm expansion in the `rayon`-enabled `frames!` branch.
