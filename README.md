# LeetCode Solutions 2025

LeetCode solutions implemented in Rust, TypeScript, and Python. Each solution
follows clean, idiomatic coding patterns with a focus on optimal time and space
complexity.

Currently working through the
[Grind 75 problem set](https://www.techinterviewhandbook.org/grind75/), a
curated list of LeetCode problems for coding interview prep.

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

## Continuous Integration (CI)

All code is automatically checked on push and pull requests to the main branch
using GitHub Actions.

The CI pipeline runs:

- Dependency installation for Python, TypeScript, and Rust
- Static analysis and tests for all files
- Caching for faster runs
- Workflow file:
  [.github/workflows/ci.yml](https://github.com/tlent/leetcode-2025/blob/main/.github/workflows/ci.yml)

## Testing Approach

- **LeetCode examples**: All test cases use actual LeetCode problem examples
- **Separate test files**: `_test.py`, `.test.ts`, `.test.rs` files alongside
  implementations
- **Test frameworks**: `pytest`, `bun test`, `cargo test`

## License

MIT License. See
[LICENSE](https://github.com/tlent/leetcode-2025/blob/main/LICENSE) for details.
