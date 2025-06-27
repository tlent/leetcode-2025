# LeetCode Solutions 2025

LeetCode solutions implemented in TypeScript and Python with optimal algorithms and idiomatic code patterns.

## Setup

### TypeScript
```bash
cd typescript
bun install
bun run type-check
bun test
```

### Python  
```bash
cd python
uv sync --dev
uv run pytest
uv run mypy src/
```

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