+++
title = "Unit Testing Best Practices for Modern Codebases"
date = "2026-01-15"
tags = ["testing", "javascript", "best-practices"]
excerpt = "Unit tests are the foundation of a reliable software project. This post covers actionable patterns that keep your test suite fast, maintainable, and trustworthy."
+++

Unit testing is one of those topics every developer agrees is important, yet few codebases actually get right. After spending years maintaining test suites that ranged from beautiful to terrifying, I have collected a set of principles that consistently lead to better outcomes.

## The Arrange-Act-Assert Pattern

Every test should follow the three-phase structure. It sounds trivial, but you would be surprised how many tests mix setup with verification.

```javascript
describe("OrderService.calculateTotal", () => {
  it("applies discount for premium members", () => {
    // Arrange
    const user = new User({ tier: "premium" });
    const order = new Order({ items: [{ price: 100 }] });
    const service = new OrderService();

    // Act
    const total = service.calculateTotal(user, order);

    // Assert
    expect(total).toBe(90);
  });
});
```

Keeping these sections visually separated makes the intent of each test obvious.

## Avoid Testing Implementation Details

Tests that break when you refactor internals are a liability. They erode confidence and slow down development.

- Test public **behavior**, not private methods
- Mock at the boundary of your system, not deep inside
- Prefer real instances over mocks when the dependency is fast and deterministic

```javascript
// 鉂?Fragile 鈥?tests internal state
expect(counter.getState().count).toBe(1);

// 鉁?Robust 鈥?tests observable behavior
expect(counter.increment()).toBe(1);
```

## One Assertion Concept Per Test

A test that checks five different things will fail in confusing ways. When the first assertion fails, you lose visibility into the rest.

| Wrong Approach | Better Approach |
|---|---|
| `expect(a).toBe(1); expect(b).toBe(2)` | Split into `it("returns 1 for a")` and `it("returns 2 for b")` |
| Chained `.toThrow().toHaveLength()` | Descriptive test names with one check |

## Use Factories over Fixtures

Hardcoded fixture files spread across your test directory make it hard to understand what a specific test relies on. Factory functions solve this.

```typescript
function createUser(overrides: Partial<User> = {}): User {
  return {
    id: 1,
    name: "Default Name",
    email: "default@example.com",
    isActive: true,
    ...overrides,
  };
}
```

Now each test can specify only the fields that matter, and reading the test tells you exactly what is important.

## Test Behaviors, Not Methods

Do **not** mirror your source file structure in your test file structure. Instead, organize tests around user-observable behaviors.

A single method often has multiple behaviors:

- Happy path
- Edge case with empty input
- Error condition with invalid data
- Performance threshold

Each of these should be a separate `it` block with a descriptive name.

## Watch for Flaky Tests

A test that passes 90 % of the time is worse than a test that always fails. Flaky tests destroy trust in the entire suite.

Common causes of flakiness:

1. Uncontrolled timing (setTimeout, racing promises)
2. Shared mutable state between tests
3. Dependence on external services
4. Randomness without a seeded generator

When you encounter a flaky test, either fix it immediately or delete it. Do not leave it in the suite.

## Keep Tests Fast

A unit test suite that takes more than a few seconds discourages developers from running it. Every millisecond matters when you are in a flow state.

- Use in-memory databases instead of real ones
- Avoid network calls in unit tests 鈥?that is what integration tests are for
- Parallelize test execution where possible

## Write Tests Before Code

Test-driven development is not about dogma; it is about design feedback. Writing the test first forces you to think about the API from the caller perspective.

The result is often simpler, more composable interfaces.

## Conclusion

Good unit tests are an investment that pays compounding interest. They document behavior, catch regressions, and give you the confidence to refactor aggressively. The practices above have served me well across many projects and I hope they serve you too.
