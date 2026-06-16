+++
title = "SOLID Principles: Building Maintainable Object-Oriented Systems"
date = "2025-08-08"
tags = ["design-patterns", "oop", "architecture"]
excerpt = "The five SOLID principles guide developers toward code that is easy to maintain, extend, and test. Here is how to apply them in practice."
+++

SOLID is an acronym for five object-oriented design principles introduced by Robert C. Martin. They help create systems that are easy to maintain and extend over time.

## Single Responsibility Principle

A class should have one, and only one, reason to change:

```python
# Bad: handles both data and persistence
class User:
    def save(self): ...

# Good: separate concerns
class User:
    pass

class UserRepository:
    def save(self, user): ...
```

## Open/Closed Principle

Software entities should be open for extension but closed for modification:

```python
from abc import ABC, abstractmethod

class PaymentProcessor(ABC):
    @abstractmethod
    def process(self, amount): ...

class CreditCardProcessor(PaymentProcessor):
    def process(self, amount):
        ...  # credit card logic

class PayPalProcessor(PaymentProcessor):
    def process(self, amount):
        ...  # PayPal logic
```

## Liskov Substitution Principle

Derived classes must be substitutable for their base classes:

```python
class Rectangle:
    def set_width(self, w): ...
    def set_height(self, h): ...

class Square(Rectangle):
    # Violates LSP: changing width changes height
    def set_width(self, w):
        self.width = w
        self.height = w
```

A `Square` is not a `Rectangle` because mutating dimensions violates the base class contract.

## Interface Segregation Principle

No client should be forced to depend on methods it does not use:

| Violation | Solution |
|-----------|----------|
| One fat interface with many methods | Split into smaller, focused interfaces |
| `Worker` with `eat()` and `work()` | `Workable` and `Eatable` interfaces |
| Unimplemented methods throw exceptions | Segregate by client need |

```python
class Workable:
    def work(self): ...

class Eatable:
    def eat(self): ...
```

## Dependency Inversion Principle

High-level modules should not depend on low-level modules. Both should depend on abstractions:

```python
# Bad: high-level depends on low-level
class EmailService:
    def send(self, msg): ...

class Notification:
    def __init__(self):
        self.email = EmailService()

# Good: depend on abstraction
class MessageService(ABC):
    @abstractmethod
    def send(self, msg): ...

class Notification:
    def __init__(self, service: MessageService):
        self.service = service
```

## Putting It All Together

Apply SOLID iteratively. Start with SRP and OCP, which give the most immediate benefits. ISP and DIP become important as your system grows. LSP is a contract design principle that prevents subtle bugs.

## Trade-offs

Over-applying SOLID leads to excessive abstraction and indirection. Use these principles as guidelines, not rules. A small script does not need interfaces and dependency injection.

Know the principles so you know when to break them.
