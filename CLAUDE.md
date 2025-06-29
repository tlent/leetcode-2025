# Claude Development Notes

## Project Overview

LeetCode solutions repository with Rust, TypeScript, and Python implementations.
Focus on optimal algorithms and idiomatic clean code patterns.

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
bun install                 # Install dependencies
bunx prettier -w .          # Format code
bunx eslint                 # Lint code
bunx tsc --noEmit          # Type check
bunx vitest run            # Run tests
```

### Python

```bash
cd python

# Development workflow
uv sync --dev              # Install dependencies
uv run ruff format         # Format code
uv run ruff check          # Lint code
uv run mypy src/           # Type check
uv run pytest             # Run tests
```

## Code Standards

### Always

- Use optimal time/space complexity algorithms
- Provide multiple solution approaches when beneficial
- Use descriptive variable names
- Code should always be simple and clean

### TypeScript

- Use Vitest in-source testing with `import.meta.vitest`
- Follow strict TypeScript typing
- Use consistent naming: camelCase functions

### Python

- Use modern Python 3.11+ features (list[int], dict[str, int])
- Leverage standard library (Counter, collections)
- Use pytest-compatible test functions
- Follow PEP 8 style via ruff

## Testing Approach

Each language follows its idiomatic testing patterns:

### TypeScript
- **Separate test files**: `.test.ts` files alongside implementations
- **Test utilities**: Shared helpers in `src/test-utils.ts` for complex setups
- **Vitest framework**: Standard testing framework with separate test files

### Python
- **Separate test files**: `test_*.py` files alongside implementations in `src/`
- **pytest framework**: Standard Python testing with automatic discovery
- **Clean imports**: Import functions directly from implementation modules

### Rust
- **Inline test modules**: `#[cfg(test)] mod tests` in each source file (idiomatic)
- **Standardized naming**: All test modules use `mod tests` (plural)
- **Comprehensive coverage**: Including utility function tests in `list.rs`

### Coverage
- **LeetCode examples**: All test cases use actual LeetCode problem examples
- **Edge cases**: Multiple test scenarios per algorithm variant
- **Clean separation**: Implementation files contain only logic, no test code

## Quality Checks

Before committing, ensure all checks pass:

```bash
# Rust checks
cargo fmt --check
cargo clippy
cargo check
cargo test

# TypeScript checks
bunx prettier --check .
bunx eslint
bunx tsc --noEmit
bunx vitest run

# Python checks
uv run ruff format --check src/
uv run ruff check src/
uv run mypy src/
uv run pytest
```

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

## Performance Notes

- TypeScript: Manual character code operations outperform regex
- Python: `Counter` is highly optimized for character frequency
- Both: Early returns prevent unnecessary computation
- Consider space/time tradeoffs (sorting vs. hashing)
