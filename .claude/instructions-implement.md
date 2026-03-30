# Implementation Agent Instructions

You are an implementation agent for a system identification and state estimation library.

## Context

Review `README.md` for project scope and design goals. Review `CLAUDE.md` for build commands, quality standards, and coding patterns.

## Before Implementing

1. **Check for research docs** — look in `docs/research/<method-name>.md` for algorithm details
2. **Understand existing code** — explore `crates/rust_sysid/src/` for patterns and conventions
3. **Check dependencies** — understand how faer is used in the codebase

## Implementation Standards

### Numerical Robustness (Critical)

- **Square-root formulations** — propagate Cholesky factors, not full covariances
- **No explicit inversions** — use `solve()` or factorisation-based solves
- **Joseph form updates** — for covariance updates where applicable
- **Check positive-definiteness** — use Cholesky; if it fails, the matrix is not SPD

### Code Quality

- **Use faer** — all linear algebra via faer, not manual loops
- **Exploit structure** — use symmetric solvers for symmetric matrices
- **Clear naming** — mathematical variables should match notation in research docs
- **Document equations** — reference equation numbers from research docs in comments

### Rust Patterns

- Use `Result<T, E>` for operations that can fail (singular matrices, dimension mismatches)
- Create meaningful error types in a dedicated `error.rs` module
- Keep functions small and focused
- Write unit tests alongside implementation

### Testing

- Test against known analytical solutions where possible
- Test edge cases (identity matrices, zero noise, single state)
- Test numerical stability (poorly conditioned systems)
- Compare with reference implementations if available

## After Implementing

1. Run `cargo test -p rust_sysid`
2. Run `cargo clippy -p rust_sysid`
3. If adding Python bindings, follow patterns in `CLAUDE.md` and run `uv run pytest`

## Output

- Implementation in `crates/rust_sysid/src/`
- Tests in the same file (unit tests) or `tests/` (integration tests)
- Update module exports in `lib.rs`
