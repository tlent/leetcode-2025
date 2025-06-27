# LeetCode Solutions 2025

LeetCode solutions implemented in Rust, TypeScript, and Python with optimal
algorithms and idiomatic clean code patterns.

## Development Commands

### Rust

```bash
cd rust

# Development workflow
cargo fmt     # Format code
cargo clippy  # Lint code
cargo check   # Type check
cargo test    # Run tests
```

### TypeScript

```bash
cd typescript

# Development workflow
bun install         # Install dependencies
bunx prettier -w .  # Format code
bunx eslint         # Lint code
bunx tsc --noEmit   # Type check
bunx vitest run     # Run tests
```

### Python

```bash
cd python

# Development workflow
uv sync --dev       # Install dependencies
uv run ruff format  # Format code
uv run ruff check   # Lint code
uv run mypy src/    # Type check
uv run pytest       # Run tests
```

## Testing

Tests are collocated with solutions to reduce file count and improve
maintainability:

- **Rust**: Uses Cargo test with test module in each source file
- **TypeScript**: Uses Vitest in-source testing in each source file
- **Python**: Uses pytest-compatible `test_*` functions in each source file
