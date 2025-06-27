# LeetCode Solutions 2025

LeetCode solutions implemented in TypeScript and Python with optimal algorithms and idiomatic code patterns.

## Setup & Testing

### TypeScript
```bash
cd typescript
bun install
bun lint
bun type-check
bun test
```

### Python  
```bash
cd python
uv sync --dev
uv run ruff check
uv run mypy src/
uv run pytest

uv run src/arrays_and_hashing/two_sum.py  # Run individual file tests
```

## Testing Approach

Tests are collocated with solutions to reduce file count and improve maintainability:

- **Python**: Uses pytest-compatible `test_*` functions that can also run individually via `if __name__ == "__main__"`
- **TypeScript**: Uses Vitest in-source testing with `import.meta.vitest`

Each solution file is self-contained with both implementation and tests. You can run all tests with `pytest`/`bun test` or individual files directly.

## LeetCode CLI Usage

Install and setup:
```bash
npm install -g leetcode-cli
leetcode user -l  # Login with credentials
```

Submit solutions:
```bash
# From typescript/ or python/
leetcode submit ./src/arrays-and-hashing/two-sum.ts
leetcode submit ./src/arrays_and_hashing/two_sum.py
```

View progress:
```bash
leetcode stat
leetcode list
```
