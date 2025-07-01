# LeetCode Solutions 2025

LeetCode solutions implemented in Rust, TypeScript, and Python with optimal
algorithms and idiomatic clean code patterns.

[Grind 75](https://www.techinterviewhandbook.org/grind75/)

## Usage

### Python

```bash
cd python

uv sync --dev                    # Install dependencies
uv run ruff format              # Format code
uv run ruff check               # Lint code
uv run mypy src/                # Type check
uv run pytest                  # Run tests

uv run ruff format && uv run ruff check && uv run mypy src/ && uv run pytest
```

### TypeScript

```bash
cd typescript

bun install                     # Install dependencies
bunx biome check --write .      # Format and lint
bunx tsc --noEmit              # Type check
bun test                       # Run tests

bunx biome check --write && bunx tsc --noEmit && bun test
```

### Rust

```bash
cd rust

cargo fmt                      # Format code
cargo clippy                   # Lint code
cargo check                    # Type check
cargo test                     # Run tests

cargo fmt && cargo clippy && cargo check && cargo test
```

## Testing Approach

### TypeScript

- **Separate test files**: `.test.ts` files alongside implementations
- **Test framework**: Bun test

### Python

- **Separate test files**: `test_*.py` files alongside implementations
- **pytest framework**: Standard Python testing with automatic discovery

### Rust

- **Inline test modules**: `#[cfg(test)] mod tests` in each source file

### Coverage

- **LeetCode examples**: All test cases use actual LeetCode problem examples
