# LeetCode Study Notes

## Overview

This document contains comprehensive study notes for LeetCode problems solved
across Rust, TypeScript, and Python. Each problem includes key insights,
algorithm patterns, and memory tricks to help with interview preparation.

## Algorithm Patterns

### Hash Map/Set Pattern (3 problems)

- **Core idea**: Use hash table for O(1) lookups
- **When to use**: Need fast lookups, complement searches, or deduplication
- **Template**: Store seen values, check for target/complement

### Two Pointers (3 problems)

- **Core idea**: Use two pointers moving in coordinated way
- **When to use**: Sorted arrays, palindromes, cycle detection
- **Template**: Initialize pointers, move based on comparison

### Stack Pattern (2 problems)

- **Core idea**: Last-in-first-out for matching/nesting
- **When to use**: Bracket matching, tree traversal, undo operations
- **Template**: Push opening items, pop on closing items

### Linked List Manipulation (3 problems)

- **Core idea**: Pointer manipulation with careful edge cases
- **When to use**: List operations, cycle detection, reversal
- **Template**: Use dummy head, track prev/current/next

### Tree Traversal (3 problems)

- **Core idea**: Recursive or iterative traversal
- **When to use**: Tree operations, depth/height calculations
- **Template**: Base case + recursive calls, or stack/queue

### Dynamic Programming (1 problem)

- **Core idea**: Build solution using previous results
- **When to use**: Optimization problems with overlapping subproblems
- **Template**: Track optimal so far, update incrementally

### Binary Search (1 problem)

- **Core idea**: Divide and conquer on sorted data
- **When to use**: Search in sorted arrays, find boundaries
- **Template**: Left/right pointers, compare with mid

### Graph Traversal (1 problem)

- **Core idea**: Visit connected components systematically
- **When to use**: Connected components, path finding, flood fill
- **Template**: Use stack/queue, mark visited

---

## Problem Solutions

### N0001 - Two Sum

**Key insight**: Use hash map to store complements for O(n) solution **Algorithm
rule**: For each number, check if complement exists in map **Code pattern**:

```py
map = {}
for i, num in enumerate(nums):
    complement = target - num
    if complement in map:
        return [map[complement], i]
    map[num] = i
```

**Memory tricks**: Think "complement search" - what do I need to find?
**Time/space**: O(n) time, O(n) space **Common mistakes**: Forgetting to check
if complement exists before adding to map **Language notes**:

- Rust: Returns `Option<(usize, usize)>` for safety
- TypeScript: Uses `Map<number, number>`
- Python: Simple `dict[int, int]`

### N0020 - Valid Parentheses

**Key insight**: Use stack to match opening/closing brackets **Algorithm rule**:
Push expected closing bracket, pop when found **Code pattern**:

```py
stack = []
for char in s:
    if char in opening:
        stack.append(closing[char])
    elif not stack or stack.pop() != char:
        return False
return len(stack) == 0
```

**Memory tricks**: Think "what am I expecting to close this?" **Time/space**:
O(n) time, O(n) space **Common mistakes**: Not checking if stack is empty before
popping **Language notes**:

- Rust: Uses bytes for performance, pattern matching
- TypeScript: Array as stack with explicit conditionals
- Python: List as stack, most concise

### N0021 - Merge Two Sorted Lists

**Key insight**: Use dummy head to simplify edge cases **Algorithm rule**:
Always pick smaller value, advance that pointer **Code pattern**:

```py
dummy = ListNode(0)
current = dummy
while list1 and list2:
    if list1.val <= list2.val:
        current.next = list1
        list1 = list1.next
    else:
        current.next = list2
        list2 = list2.next
    current = current.next
current.next = list1 or list2
return dummy.next
```

**Memory tricks**: Think "pick the smaller one, keep going" **Time/space**:
O(n+m) time, O(1) space **Common mistakes**: Forgetting to handle remaining
nodes **Language notes**:

- Rust: Complex ownership with `Box<ListNode>` and `take()`
- TypeScript: Nullish coalescing `??` for clean syntax
- Python: `or` operator for remaining nodes

### N0104 - Maximum Depth of Binary Tree

**Key insight**: Recursively find max depth of left/right subtrees **Algorithm
rule**: Depth = 1 + max(left_depth, right_depth) **Code pattern**:

```py
def maxDepth(root):
    if not root:
        return 0
    return 1 + max(maxDepth(root.left), maxDepth(root.right))
```

**Memory tricks**: Think "how deep can I go?" **Time/space**: O(n) time, O(h)
space (h = height) **Common mistakes**: Forgetting base case (null node)
**Language notes**:

- Rust: `Rc<RefCell<TreeNode>>` for shared ownership
- TypeScript: Simple recursive approach
- Python: Most concise implementation

### N0121 - Best Time to Buy and Sell Stock

**Key insight**: Track minimum price and maximum profit in single pass
**Algorithm rule**: Update min_price, then update max_profit **Code pattern**:

```py
min_price = float('inf')
max_profit = 0
for price in prices:
    min_price = min(min_price, price)
    max_profit = max(max_profit, price - min_price)
return max_profit
```

**Memory tricks**: Think "buy low, sell high" - track lowest seen
**Time/space**: O(n) time, O(1) space **Common mistakes**: Calculating profit
before updating min_price **Language notes**:

- Rust: Slice iteration with explicit bounds
- TypeScript: Traditional for loop
- Python: Clean slice notation

### N0125 - Valid Palindrome

**Key insight**: Use two pointers from start/end, ignore non-alphanumeric
**Algorithm rule**: Move pointers inward, compare characters **Code pattern**:

```py
left, right = 0, len(s) - 1
while left < right:
    if not s[left].isalnum():
        left += 1
    elif not s[right].isalnum():
        right -= 1
    elif s[left].lower() != s[right].lower():
        return False
    else:
        left += 1
        right -= 1
return True
```

**Memory tricks**: Think "squeeze from both ends" **Time/space**: O(n) time,
O(1) space **Common mistakes**: Not handling non-alphanumeric characters
**Language notes**:

- Rust: Byte-level processing for performance
- TypeScript: Custom alphanumeric check
- Python: Built-in `isalnum()` method

### N0141 - Linked List Cycle

**Key insight**: Floyd's cycle detection (tortoise and hare) **Algorithm rule**:
Fast pointer moves 2x, slow moves 1x, they meet if cycle **Code pattern**:

```py
slow = fast = head
while fast and fast.next:
    slow = slow.next
    fast = fast.next.next
    if slow == fast:
        return True
return False
```

**Memory tricks**: Think "tortoise and hare race" **Time/space**: O(n) time,
O(1) space **Common mistakes**: Not checking if fast.next exists **Language
notes**:

- Rust: Complex with `Rc<RefCell<>>` and `ptr_eq()`
- TypeScript: Optional chaining for safety
- Python: Simplest pointer comparison

### N0206 - Reverse Linked List

**Key insight**: Use three pointers: prev, current, next **Algorithm rule**:
Reverse direction of current.next, advance all pointers **Code pattern**:

```py
prev = None
current = head
while current:
    next_temp = current.next
    current.next = prev
    prev = current
    current = next_temp
return prev
```

**Memory tricks**: Think "flip the arrows" **Time/space**: O(n) time, O(1) space
**Common mistakes**: Losing reference to next node **Language notes**:

- Rust: Iterative efficient, recursive O(n²) due to ownership
- TypeScript: Both iterative and recursive O(n)
- Python: Both approaches work cleanly

### N0217 - Contains Duplicate

**Key insight**: Use set to track seen values **Algorithm rule**: If value
already in set, duplicate found **Code pattern**:

```py
seen = set()
for num in nums:
    if num in seen:
        return True
    seen.add(num)
return False
```

**Memory tricks**: Think "have I seen this before?" **Time/space**: O(n) time,
O(n) space **Common mistakes**: Using O(n²) nested loops instead **Language
notes**:

- Rust: `HashSet` with `any()` combinator option
- TypeScript: `Set` with size comparison
- Python: Most concise with `set()` conversion

### N0226 - Invert Binary Tree

**Key insight**: Recursively swap left and right subtrees **Algorithm rule**:
Swap children, then invert each subtree **Code pattern**:

```py
def invertTree(root):
    if not root:
        return None
    root.left, root.right = root.right, root.left
    invertTree(root.left)
    invertTree(root.right)
    return root
```

**Memory tricks**: Think "flip like a mirror" **Time/space**: O(n) time, O(h)
space **Common mistakes**: Not handling null nodes **Language notes**:

- Rust: Stack-based with complex `RefCell` borrowing
- TypeScript: Array destructuring for swapping
- Python: Tuple swapping, most elegant

### N0242 - Valid Anagram

**Key insight**: Count character frequencies, must match **Algorithm rule**:
Same length and same character counts **Code pattern**:

```py
if len(s) != len(t):
    return False
count = [0] * 26
for c in s:
    count[ord(c) - ord('a')] += 1
for c in t:
    count[ord(c) - ord('a')] -= 1
return all(c == 0 for c in count)
```

**Memory tricks**: Think "same ingredients, same amounts" **Time/space**: O(n)
time, O(1) space (fixed size array) **Common mistakes**: Not checking string
lengths first **Language notes**:

- Rust: Fixed-size array with byte arithmetic
- TypeScript: `charCodeAt(0) - 97` for indexing
- Python: `Counter` from collections most concise

### N0704 - Binary Search

**Key insight**: Divide search space in half each iteration **Algorithm rule**:
Compare with mid, adjust left/right bounds **Code pattern**:

```py
left, right = 0, len(nums) - 1
while left <= right:
    mid = left + (right - left) // 2
    if nums[mid] == target:
        return mid
    elif nums[mid] < target:
        left = mid + 1
    else:
        right = mid - 1
return -1
```

**Memory tricks**: Think "cut in half, go left or right" **Time/space**: O(log
n) time, O(1) space **Common mistakes**: Integer overflow in mid calculation
**Language notes**:

- Rust: `Ordering` enum with `match` for three-way comparison
- TypeScript: `Math.floor()` for integer division
- Python: Floor division `//` operator

### N0733 - Flood Fill

**Key insight**: Use DFS to visit all connected cells with same color
**Algorithm rule**: If same color, change it and visit neighbors **Code
pattern**:

```py
def floodFill(image, sr, sc, color):
    original_color = image[sr][sc]
    if original_color == color:
        return image

    stack = [(sr, sc)]
    while stack:
        r, c = stack.pop()
        if image[r][c] == original_color:
            image[r][c] = color
            for dr, dc in [(0,1), (0,-1), (1,0), (-1,0)]:
                nr, nc = r + dr, c + dc
                if 0 <= nr < len(image) and 0 <= nc < len(image[0]):
                    stack.append((nr, nc))
    return image
```

**Memory tricks**: Think "paint bucket fill" **Time/space**: O(n) time, O(n)
space **Common mistakes**: Not checking if original color equals new color
**Language notes**:

- Rust: `wrapping_sub(1)` for underflow safety
- TypeScript: Explicit bounds checking
- Python: Chained comparison for bounds

### N0235 - Lowest Common Ancestor of BST

**Key insight**: Use BST property to navigate directly to LCA **Algorithm
rule**: If p and q on different sides of node, that's LCA **Code pattern**:

```py
min_val = min(p.val, q.val)
max_val = max(p.val, q.val)
current = root
while current:
    if max_val < current.val:
        current = current.left
    elif min_val > current.val:
        current = current.right
    else:
        return current
```

**Memory tricks**: Think "am I at the split point?" **Time/space**: O(log n)
time, O(1) space **Common mistakes**: Using generic tree LCA instead of BST
property **Language notes**:

- Rust: Complex `Rc<RefCell<TreeNode>>` handling, `clone()` for reference counting
- TypeScript: Non-null assertions after null checks
- Python: Clean min/max calculations, Optional types

### N0110 - Balanced Binary Tree

**Key insight**: Combine height calculation with balance checking in single pass
**Algorithm rule**: Return -1 (or None/null) to signal imbalance, propagate up
**Code pattern**:

```py
def isBalanced(root):
    def height(node):
        if not node:
            return 0
        left = height(node.left)
        right = height(node.right)
        if left == -1 or right == -1 or abs(left - right) > 1:
            return -1
        return 1 + max(left, right)
    return height(root) != -1
```

**Memory tricks**: Think "height with early exit" - if imbalanced, bail out
**Time/space**: O(n) time, O(h) space
**Common mistakes**: Calculating height multiple times instead of single pass
**Language notes**:
- Rust: Uses `Option<u32>` with `?` operator, `abs_diff()` for unsigned math
- TypeScript: Uses `number | null` with explicit null checks
- Python: Uses `Optional[int]` with -1 sentinel value

### N0232 - Implement Queue using Stacks

**Key insight**: Use two stacks - input for enqueue, output for dequeue
**Algorithm rule**: Transfer from input to output only when output is empty
**Code pattern**:

```py
class MyQueue:
    def __init__(self):
        self.input = []
        self.output = []
    
    def push(self, x):
        self.input.append(x)
    
    def pop(self):
        self._transfer()
        return self.output.pop()
    
    def _transfer(self):
        if not self.output:
            while self.input:
                self.output.append(self.input.pop())
```

**Memory tricks**: Think "lazy transfer" - only move when output is empty
**Time/space**: Amortized O(1) for all operations, O(n) space
**Common mistakes**: Transferring on every operation instead of lazy transfer
**Language notes**:
- Rust: Uses `Vec<i32>` with `unwrap()` for guaranteed non-empty pops
- TypeScript: Uses arrays with explicit null assertions
- Python: Uses lists as stacks, cleanest implementation

---

## Quick Reference by Pattern

### Hash Map/Set Problems

- N0001 Two Sum - complement search
- N0217 Contains Duplicate - deduplication
- N0242 Valid Anagram - frequency counting

### Two Pointers Problems

- N0001 Two Sum - sorted array variation
- N0125 Valid Palindrome - center outward
- N0141 Linked List Cycle - Floyd's algorithm

### Stack Problems

- N0020 Valid Parentheses - bracket matching
- N0232 Implement Queue using Stacks - two-stack pattern
- N0733 Flood Fill - DFS with stack

### Linked List Problems

- N0021 Merge Two Sorted Lists - merge pattern
- N0141 Linked List Cycle - cycle detection
- N0206 Reverse Linked List - reversal pattern

### Tree Problems

- N0104 Maximum Depth - DFS traversal
- N0110 Balanced Binary Tree - height-based balance check
- N0226 Invert Binary Tree - tree transformation
- N0235 LCA of BST - BST property navigation

### Other Patterns

- N0121 Best Time to Buy/Sell Stock - Dynamic Programming
- N0704 Binary Search - Divide and Conquer
- N0733 Flood Fill - Graph Traversal
