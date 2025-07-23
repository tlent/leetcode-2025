# LeetCode Solutions 2025

LeetCode solutions implemented in Rust, TypeScript, and Python. Each solution follows
clean, idiomatic coding patterns with a focus on optimal time and space complexity.

## Usage

### Required Tools

- **Python**: [uv](https://docs.astral.sh/uv/getting-started/installation/)
- **TypeScript**: [deno](https://docs.deno.com/runtime/getting_started/installation/)
- **Rust**: [cargo](https://www.rust-lang.org/tools/install),
  [clippy](https://github.com/rust-lang/rust-clippy)

### Python Usage

```bash
cd python
uv sync --dev       # Install dependencies

uv run ruff format  # Format
uv run ruff check   # Lint
uv run mypy src/    # Type check
uv run pytest       # Run tests
```

### TypeScript Usage

```bash
cd typescript
deno install                # Install dependencies

deno fmt                    # Format
deno lint                   # Lint
deno check                  # Type check
deno test                   # Run tests
```

### Rust Usage

```bash
cd rust

cargo fmt     # Format
cargo clippy  # Lint
cargo check   # Type check
cargo test    # Run tests
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

Workflow files: [rust](https://github.com/tlent/leetcode-2025/blob/main/.github/workflows/rust-ci.yml), [typescript](https://github.com/tlent/leetcode-2025/blob/main/.github/workflows/python-ci.yml), [python](https://github.com/tlent/leetcode-2025/blob/main/.github/workflows/typescript-ci.yml)

## Testing Approach

- **LeetCode examples**: All test cases use actual LeetCode problem examples
- **Separate test files**: `.test.rs`, `.test.ts`, `_test.py` files alongside
  implementations
- **Test frameworks**: `cargo test`, `deno test`, `pytest`

## License

MIT License. See
[LICENSE](https://github.com/tlent/leetcode-2025/blob/main/LICENSE) for details.
