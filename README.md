# LeetCode Solutions 2025

LeetCode solutions implemented in Rust, TypeScript, and Python with optimal
algorithms and idiomatic clean code patterns.

[Grind 75](https://www.techinterviewhandbook.org/grind75/)

## Usage

### Python Usage

```bash
cd python

uv sync --dev                    # Install dependencies
uv run ruff format              # Format code
uv run ruff check               # Lint code
uv run mypy src/                # Type check
uv run pytest                  # Run tests

uv run ruff format && uv run ruff check && uv run mypy src/ && uv run pytest
```

### TypeScript Usage

```bash
cd typescript

bun install                     # Install dependencies
bunx biome check --write .      # Format and lint
bunx tsc --noEmit              # Type check
bun test                       # Run tests

bunx biome check --write && bunx tsc --noEmit && bun test
```

### Rust Usage

```bash
cd rust

cargo fmt                      # Format code
cargo clippy                   # Lint code
cargo check                    # Type check
cargo test                     # Run tests

cargo fmt && cargo clippy && cargo check && cargo test
```

## Pre-commit Hooks

Use pre-commit hooks to run all static analysis and tests:

```bash
# Install and run pre-commit hooks (runs only on changed files)
pip install pre-commit
pre-commit install
pre-commit run

# Run all checks and tests manually on all files
pre-commit run --all-files
```

## Testing Approach

- **LeetCode examples**: All test cases use actual LeetCode problem examples
- **Separate test files**: `_tyest.py`, `.test.ts`, `.test.rs` files alongside
  implementations
- **Test frameworks**: pytest, bun test, cargo test
