# Claude Development Notes

## Project Overview

LeetCode solutions implemented in Rust, TypeScript, and Python with optimal
algorithms and idiomatic clean code patterns.

[Grind 75](https://www.techinterviewhandbook.org/grind75/)

## Usage

### Required Tools

- **Python**: [uv](https://docs.astral.sh/uv/getting-started/installation/)
- **TypeScript**: [bun](https://bun.sh/docs/installation)
- **Rust**: [cargo](https://www.rust-lang.org/tools/install),
  [clippy](https://github.com/rust-lang/rust-clippy)

### Python Usage

```bash
cd python
uv sync --dev       # Install dependencies

uv run ruff format  # Format code
uv run ruff check   # Lint code
uv run mypy src/    # Type check
uv run pytest       # Run tests

uv run ruff format && uv run ruff check && uv run mypy src/ && uv run pytest
```

### TypeScript Usage

```bash
cd typescript
bun install                 # Install dependencies

bunx biome check --write .  # Format and lint
bunx tsc --noEmit           # Type check
bun test                    # Run tests

bunx biome check --write && bunx tsc --noEmit && bun test
```

### Rust Usage

```bash
cd rust

cargo fmt     # Format code
cargo clippy  # Lint code
cargo check   # Type check
cargo test    # Run tests

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

## Code Standards

### Global Standards

- Use optimal time/space complexity algorithms
- Provide multiple solution approaches when beneficial
- Use descriptive variable names
- Code should always be simple and clean
- Avoid over-engineering

### Rust Standards

- Use Cargo fmt, clippy, check, test
- Leverage standard library

### TypeScript Standards

- Use bun test
- Follow strict TypeScript typing
- Use consistent naming: camelCase functions
- Leverage bun runtime standard library

### Python Standards

- Use modern Python 3.11+ features (list[int], dict[str, int])
- Leverage standard library (Counter, collections)
- Use pytest-compatible test functions
- Follow PEP 8 style via ruff

## Solution Patterns

Collect helpful notes here of solution patterns we discover

### Arrays & Hashing

- Use `Set` for deduplication and O(1) lookups
- Use `Map`/`dict` for complement searches (Two Sum pattern)
- Consider character counting arrays for string problems

### Two Pointers

- Clean input first vs. skip-while-processing approaches
- Character code manipulation for performance
- Consider both directions (start/end pointers)
