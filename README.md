# rust-sysid
A dynamic system identification library built in Rust with Python bindings.

## Intent

This library provides system identification and state estimation tools for dynamic systems. The core algorithms are implemented in Rust for performance and safety, with Python bindings via PyO3 for ease of use in scientific workflows.

## Scope

The library covers both **linear** and **non-linear** dynamic systems.

### State Estimation

A progression of filtering and smoothing algorithms:

- **Kalman filter/smoother** — optimal for linear-Gaussian systems
- **Extended Kalman filter (EKF)** — first-order linearisation for non-linear systems
- **Unscented Kalman filter (UKF)** — sigma-point methods for improved non-linear handling
- **Particle filters / Sequential Monte Carlo** — general non-linear, non-Gaussian systems

### System Identification

Parameter estimation methods for state-space models:

- **Expectation Maximisation (EM)** — maximum likelihood via iterative filtering/smoothing
- **Sampling approaches** — MCMC and other Bayesian methods for posterior inference

## Design Goals

### Numerical Robustness

Numerical stability is a primary concern throughout the implementation:

- **Square-root formulations** — filters propagate Cholesky factors rather than full covariance matrices, preserving positive-definiteness by construction
- **Stable updates** — Joseph form and other numerically robust covariance update equations
- **Avoiding explicit inversions** — linear solves instead of computing matrix inverses

### Computational Efficiency

Performance without sacrificing correctness:

- **faer** — all linear algebra uses [faer](https://github.com/sarah-ek/faer-rs), a high-performance pure-Rust library, chosen for tight Rust integration over BLAS/LAPACK
- **Structure exploitation** — algorithms exploit matrix symmetry, sparsity, and bandedness where applicable
- **Appropriate factorizations** — Cholesky for symmetric positive-definite systems, QR for least-squares problems

## Project Structure

```
rust-sysid/
├── Cargo.toml                    # Workspace definition
├── pyproject.toml                # Python/maturin build config
├── .python-version               # Python version (3.12)
└── crates/
    ├── rust_sysid/               # Pure Rust library + binary
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs            # Library entry point
    │       └── main.rs           # Binary entry point
    └── rust_sysid_py/            # Python bindings (PyO3)
        ├── Cargo.toml
        └── src/
            └── lib.rs            # PyO3 module definition
```

### How It Works

This project uses a **Cargo workspace** with two crates:

1. **`rust_sysid`** - The core Rust library containing all the system identification logic. It also includes a binary target for running Rust code directly.

2. **`rust_sysid_py`** - A thin wrapper that exposes the Rust library to Python using [PyO3](https://pyo3.rs/). This crate compiles to a `cdylib` (shared library) that Python can import as a native module.

The Python build is managed by [maturin](https://github.com/PyO3/maturin), configured in `pyproject.toml`.

## Development

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Python 3.12+
- [uv](https://github.com/astral-sh/uv)

### Commands

| Task | Command |
|------|---------|
| Build Rust library | `cargo build -p rust_sysid` |
| Run Rust binary | `cargo run -p rust_sysid` |
| Run Rust tests | `cargo test -p rust_sysid` |
| Build Python module | `uv run maturin develop` |
| Build release wheel | `uv run maturin build --release` |
| Run Python | `uv run python` |

### Quick Start

```bash
# Install dependencies and build Python module
uv run maturin develop

# Use in Python
uv run python -c "import rust_sysid; print(rust_sysid.add(2, 3))"
```
