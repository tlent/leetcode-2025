# Claude Development Notes

## Project Overview
LeetCode solutions repository with TypeScript and Python implementations. Focus on optimal algorithms and idiomatic clean code patterns.

## Development Commands

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
- **Collocated tests**: Tests live alongside implementation
- **Coverage**: Example test cases from leetcode

## Quality Checks
Before committing, ensure all checks pass:

```bash
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
