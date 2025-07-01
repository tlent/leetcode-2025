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

```bash
# Install pre-commit hooks
uv run pre-commit install

# Run all static analysis and tests for changed files
uv run pre-commit run

# Run all static analysis and tests for all files
uv run pre-commit --all-files
```

## Testing Approach

- **LeetCode examples**: All test cases use actual LeetCode problem examples
- **Separate test files**: `_test.py`, `.test.ts`, `.test.rs` files alongside
  implementations
- **Test frameworks**: pytest, bun test, cargo test
