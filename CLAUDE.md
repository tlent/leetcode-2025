# Claude Development Notes

## Project Overview

LeetCode solutions repository with Rust, TypeScript, and Python implementations.
Focus on optimal algorithms and idiomatic clean code patterns.

## Development Commands

### Individual Language Workflows

Use the commands below for each language:

### Rust (Optimized)

```bash
cd rust

# Development workflow (optimized)
cargo fmt                   # Format code
cargo clippy               # Lint code  
cargo check                # Type check
cargo test                 # Run tests

# Sequential execution (cleaner output)
cargo fmt && cargo clippy && cargo check  # Run format/lint/check sequentially
```

### TypeScript (Optimized with Biome)

```bash
cd typescript

# Development workflow (optimized with Biome)
bun install                         # Install dependencies
bunx biome check --write .          # Format and lint code (replaces prettier + eslint)
bunx tsc --noEmit                   # Type check (with incremental)
bun test                            # Run tests (faster than vitest)

# Sequential execution (cleaner output)
bunx biome check --write . && bunx tsc --noEmit && bun test
```

### Python (Already Optimized)

```bash
cd python

# Development workflow (already optimized)
uv sync --dev              # Install dependencies
uv run ruff format         # Format code
uv run ruff check          # Lint code
uv run mypy src/           # Type check
uv run pytest             # Run tests

# Sequential execution (cleaner output)
uv run ruff format && uv run ruff check && uv run mypy src/
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

### Quality Checks

Run quality checks for each language individually:

### Rust Quality Checks (Optimized)

```bash
# Individual checks
cargo fmt --check
cargo clippy
cargo check  
cargo test                 # Run tests

# Sequential execution (cleaner output)
cargo fmt --check && cargo clippy && cargo check && cargo test
```

### TypeScript Quality Checks (Optimized)

```bash
# Individual checks (with Biome)
bunx biome check .              # Combined format/lint check (replaces prettier + eslint)
bunx tsc --noEmit              # With incremental compilation
bun test                       # Run tests (faster than vitest)

# Sequential execution (cleaner output)
bunx biome check . && bunx tsc --noEmit && bun test
```

### Python Quality Checks (Already Optimized)

```bash
# Individual checks
uv run ruff format --check src/
uv run ruff check src/
uv run mypy src/
uv run pytest

# Sequential execution (cleaner output)
uv run ruff format --check src/ && uv run ruff check src/ && uv run mypy src/ && uv run pytest
```

### Pre-commit Hooks

```bash
# Install pre-commit hooks (runs only on changed files)
pip install pre-commit
pre-commit install

# Run manually on all files
pre-commit run --all-files
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

### Algorithm Performance
- TypeScript: Manual character code operations outperform regex
- Python: `Counter` is highly optimized for character frequency
- Both: Early returns prevent unnecessary computation
- Consider space/time tradeoffs (sorting vs. hashing)

### Tooling Performance (Benchmarked)

**Overall Speed Ranking (Sequential Execution):**
1. **Python** (0.569s total) - fastest across all tools
2. **TypeScript** (0.812s total) - good performance with Biome + bun test
3. **Rust** (1.020s total) - slower due to cargo fmt compile time

**Tool-specific Performance:**
- **Format/Lint**: Python ruff (0.1s) > Biome check (0.03s) > Rust fmt+clippy (0.44s)
- **Typecheck**: Rust cargo check (0.00s) > Python mypy (0.24s) > TypeScript tsc (0.48s)
- **Test**: Python pytest (0.02s) > bun test (0.06s) > Rust cargo test (0.03s)

**Optimization Impact:**
- TypeScript: Biome + bun test provides unified tooling and good performance
- Rust: cargo toolchain + sequential execution for clean output
- Python: Already optimal with modern tooling (ruff, uv)
- Sequential execution provides cleaner, readable output without interspersed text
