+++
title = "Event Sourcing: Storing Truth as a Sequence of Facts"
date = "2026-10-30"
tags = ["event-sourcing", "architecture", "backend"]
excerpt = "Event sourcing stores every state change as an immutable event rather than the current state. This pattern unlocks powerful audit, replay, and temporal query capabilities."
+++

Event sourcing is an architectural pattern where every change to the application state is captured as an immutable event and stored in sequence. Instead of storing the current state of an entity, you store the series of events that led to that state.

## The Core Idea

In a traditional CRUD system, updating a user's email overwrites the previous value:

```sql
UPDATE users SET email = 'new@example.com' WHERE id = 42;
```

With event sourcing, you append a new event:

```json
{ "type": "EmailChanged", "userId": 42, "newEmail": "new@example.com", "timestamp": "2026-10-30T10:00:00Z" }
```

The current state is derived by replaying all events for that entity.

## Event Store

The event store is an append-only database that persists events.

```typescript

interface EventStore {
    appendEvents(
        streamId: string,
        expectedVersion: number,
        events: DomainEvent[]
    ): Promise<void>;

    readEvents(streamId: string): Promise<DomainEvent[]>;
}
```

Concurrency control is handled through optimistic locking with version numbers.

## Aggregates and Projections

An aggregate replays events to rebuild its current state. A projection reads events from the store and builds query-optimized views.


```typescript
class BankAccount {
  private balance = 0;

  constructor(events: DomainEvent[]) {
    for (const event of events) {
      this.apply(event);
    }
  }

  apply(event: DomainEvent): void {
    if (event instanceof MoneyDeposited) {
      this.balance += event.amount;
    } else if (event instanceof MoneyWithdrawn) {
      this.balance -= event.amount;
    }
  }

  deposit(amount: number): MoneyDeposited {
    return new MoneyDeposited(amount, Date.now());
  }
}
```

Projections can be rebuilt by replaying all events from scratch, giving you the ability to introduce new read models retroactively.

## Event Versioning

Events are immutable once written. Schema changes require versioning.

```typescript
class UserCreatedV1 {
    type = "UserCreated";
    version = 1;
    constructor(
        public userId: string,
        public name: string,
        public email: string,
    ) {}
}

class UserCreatedV2 {
    type = "UserCreated";
    version = 2;
    constructor(
        public userId: string,
        public name: string,
        public email: string,
        public phone?: string,
    ) {}
}

```
Upcasters convert old event versions to the current schema during replay.

```typescript

function upcast(event: any): DomainEvent {
    if (event.type === "UserCreated" && event.version === 1) {
        return new UserCreatedV2(event.userId, event.name, event.email);
    }
    return event;
}
```

## Benefits

| Benefit | Description |
|---|---|
| **Audit trail** | Every change is recorded with who and when |
| **Time travel** | Query the state at any point in history |
| **Debugging** | Reproduce bugs by replaying exact event sequences |
| **CQRS** | Separate read and write models naturally |
| **Analytics** | Derive business metrics from historical events |

## When to Use Event Sourcing

Event sourcing adds complexity. Use it when:

- You need a complete audit log (finance, compliance)
- Temporal queries are a core requirement
- You need to rebuild projections for new use cases
- Event-driven integrations are part of the architecture

Avoid it for simple CRUD applications, small projects, or domains where events have no business meaning.

## Common Pitfalls

1. **Event schema evolution** 鈥?without careful versioning, old events become unreadable
2. **Performance** 鈥?replaying millions of events requires snapshots
3. **Snapshot strategy** 鈥?periodic snapshots prevent unbounded replay


```typescript
function rebuildState(snapshot: Snapshot, subsequentEvents: DomainEvent[]): Aggregate {
  let aggregate = snapshot.aggregate;
  for (const event of subsequentEvents) {
    aggregate.apply(event);
  }
  return aggregate;
}
```

Snapshots store the aggregate state every N events, limiting replay to the events since the last snapshot.

## Conclusion

Event sourcing is not the right choice for every project, but when audit, temporal queries, or event-driven integration are first-class requirements, it is a powerful tool. Start with a clear boundary 鈥?a single aggregate 鈥?and expand only as the benefits materialize.
