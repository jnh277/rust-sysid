# rust-sysid
A dynamic system identification project built in Rust with Python bindings.

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
