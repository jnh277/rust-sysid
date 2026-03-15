# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

You are an expert software engineer experienced in domain driven development.

## Project Overview

This is a system identification library written in Rust with Python bindings via PyO3. See `README.md` for project structure, prerequisites, and usage examples.

## Build Commands

| Task | Command |
|------|---------|
| Build Rust library | `cargo build -p rust_sysid` |
| Run Rust binary | `cargo run -p rust_sysid` |
| Run all Rust tests | `cargo test -p rust_sysid` |
| Run a single Rust test | `cargo test -p rust_sysid test_name` |
| Rust linting | `cargo clippy -p rust_sysid` |
| Build Python module (dev) | `uv run maturin develop` |
| Build release wheel | `uv run maturin build --release` |
| Run Python | `uv run python` |
| Run Python tests | `uv run pytest` |
| Python tests with coverage | `uv run pytest --cov` |
| Python linting | `uv run ruff check` |

## Quality Standards

**Rust changes**: Always run `cargo test` and `cargo clippy` to verify correctness and code quality.

**Python changes**: Always run `uv run pytest`, `uv run ruff check`, and check test coverage with `uv run pytest --cov`. Aim for 95%+ test coverage.

## Architecture

**Cargo Workspace** (`Cargo.toml`):
- `crates/rust_sysid/` - Core Rust library with system identification logic, also includes a binary target
- `crates/rust_sysid_py/` - Thin PyO3 wrapper that exposes the Rust library to Python as a native module

**Python Build**:
- Managed by maturin (configured in `pyproject.toml`)
- The Python module imports as `rust_sysid` (not `rust_sysid_py`)
- Uses Rust edition 2024 and PyO3 0.23

## Python Bindings Patterns

When exposing Rust types to Python:

- **Wrapper classes**: Create `PyFoo` structs with `#[pyclass(name = "Foo")]` that wrap core Rust types via an `inner` field
- **Constructors**: Use `#[new]` for `__init__`, converting Python values to Rust types with proper error handling
- **Properties**: Use `#[getter]` for read-only attributes
- **Functions**: Use `#[pyfunction]` with `#[pyo3(name = "python_name")]` for renaming

For NumPy integration:
- Use `PyReadonlyArray1<f64>` for input arrays
- Use `PyArray1::from_vec(py, vec)` for output arrays
- Validate array lengths early with helper functions

Error handling:
- Convert Rust errors to `PyValueError` for invalid inputs
- Convert Rust errors to `PyRuntimeError` for execution failures
- Use `.map_err(|e| PyValueError::new_err(e.to_string()))?` pattern

Organization in `rust_sysid_py/src/lib.rs`:
- Group code with section headers (classes, helper functions, Python functions, module registration)
- Keep helper functions as pure Rust (not `#[pyfunction]`) for testability
