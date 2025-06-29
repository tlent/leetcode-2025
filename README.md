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

Each language follows its idiomatic testing patterns:

- **Rust**: Inline `#[cfg(test)]` modules in source files (idiomatic Rust approach)
- **TypeScript**: Separate `.test.ts` files alongside implementations
- **Python**: Separate `test_*.py` files alongside implementations

This approach provides clean separation between implementation and tests while maintaining discoverability and organization.
