+++
title = "Refactoring Legacy Code Without Breaking Everything"
date = "2025-04-05"
tags = ["refactoring", "best-practices", "testing"]
excerpt = "Practical strategies for safely improving legacy codebases, from building a safety net to incremental improvements."
+++

Legacy code is code that works, is poorly understood, and is terrifying to change. Every developer encounters it. The key to refactoring it is discipline and incrementalism.

## Step 1: Build a Safety Net

Before changing anything, you need tests. Legacy code often has none, so start with characterization tests:

```python
def test_existing_behavior():
    result = legacy_function(42, "test")
    assert result == expected_output
```

These tests capture current behavior, not necessarily correct behavior. They ensure your refactoring doesn't change what the code does.

## Step 2: Identify Seams

Seams are places where you can alter behavior without changing the code. Dependency injection is the most powerful seam:

```python
# Before: hardcoded dependency
def process_data():
    db = DatabaseConnection()
    return db.query(...)

# After: injectable dependency
def process_data(db=None):
    db = db or DatabaseConnection()
    return db.query(...)
```

## The Strangler Fig Pattern

Gradually replace legacy components with new ones while keeping the system running:

1. Identify a component to replace
2. Route requests to both old and new implementations
3. Compare results
4. Switch over when confident
5. Remove the old code

```python
def get_user(user_id):
    # Old implementation
    # return old_db.query(user_id)

    # New implementation
    result = new_api.get_user(user_id)
    return result
```

## Common Code Smells

| Smell | Symptom | Fix |
|-------|---------|-----|
| God class | Single class does everything | Extract smaller classes |
| Long method | Method exceeds 30 lines | Extract methods |
| Shotgun surgery | One change affects many files | Consolidate logic |
| Feature envy | Method uses another class excessively | Move method |
| Primitive obsession | Using primitives instead of objects | Create value objects |

## The Golden Master Technique

For code paths that are hard to test in isolation, capture input/output pairs from production:

```python
# Record production behavior
with open("golden_master.json", "w") as f:
    for input_data in production_samples:
        output = legacy_system(input_data)
        json.dump({"input": input_data, "output": output}, f)
```

## Extract Method Safely

The safest refactoring is extracting a method. It's mechanical, reversible, and compilers verify it:

```python
def complex_function(data):
    validate(data)
    transformed = transform_data(data)
    result = calculate_result(transformed)
    return format_output(result)

def validate(data):
    if not data:
        raise ValueError("Data required")

def transform_data(data):
    return [x * 2 for x in data]
```

## Dealing with Global State

Global state makes testing impossible. Wrap it in a context object:

```python
class AppContext:
    def __init__(self):
        self.config = load_config()
        self.db = Database(self.config)

# Pass context everywhere instead of accessing globals
```

## When to Stop Refactoring

Stop when the code is good enough. Perfect code is a myth. Refactor with a specific goal: improve testability, add a feature, or reduce bugs. Refactoring for its own sake wastes time.

The goal is not perfect code. The goal is code you can change safely.
