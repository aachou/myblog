+++
title = "Concurrency Patterns"
date = "2022-07-22"
tags = ["concurrency", "go", "patterns"]
excerpt = "Explore common concurrency patterns including worker pools, fan-out/fan-in, pipelines, and or-channel in Go."
+++

Concurrency is about dealing with multiple things at once. Go makes this elegant with goroutines and channels, but the patterns you use matter more than the primitives.

## Worker Pool

A worker pool distributes work across a fixed number of goroutines:

```go
func WorkerPool(jobs <-chan Job, results chan<- Result, numWorkers int) {
    var wg sync.WaitGroup
    for i := 0; i < numWorkers; i++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            for job := range jobs {
                results <- process(job)
            }
        }(i)
    }
    wg.Wait()
    close(results)
}
```

## Fan-Out, Fan-In

Fan-out starts multiple goroutines to read from the same channel. Fan-in merges multiple channels into one:

```go
func FanIn(channels ...<-chan int) <-chan int {
    out := make(chan int)
    var wg sync.WaitGroup
    for _, ch := range channels {
        wg.Add(1)
        go func(c <-chan int) {
            defer wg.Done()
            for v := range c {
                out <- v
            }
        }(ch)
    }
    go func() {
        wg.Wait()
        close(out)
    }()
    return out
}
```

## Pipeline Pattern

A pipeline connects stages with channels:

```go
func generate(nums ...int) <-chan int {
    out := make(chan int)
    go func() {
        for _, n := range nums {
            out <- n
        }
        close(out)
    }()
    return out
}

func square(in <-chan int) <-chan int {
    out := make(chan int)
    go func() {
        for n := range in {
            out <- n * n
        }
        close(out)
    }()
    return out
}
```

## Or-Channel

Combine multiple done channels into one:

```go
func Or(channels ...<-chan struct{}) <-chan struct{} {
    switch len(channels) {
    case 0:
        return nil
    case 1:
        return channels[0]
    }
    orDone := make(chan struct{})
    go func() {
        defer close(orDone)
        select {
        case <-channels[0]:
        case <-channels[1]:
        case <-Or(channels[2:]...):
        }
    }()
    return orDone
}
```

## Pattern Comparison

| Pattern | Use Case | Complexity |
|---------|----------|------------|
| Worker Pool | Bounded parallelism | Low |
| Fan-out/in | Map-reduce style | Medium |
| Pipeline | Data processing stages | Medium |
| Or-channel | Graceful shutdown | High |

## Common Mistakes

- **Leaking goroutines** 鈥?always ensure goroutines exit
- **Closing channels twice** 鈥?use sync.Once
- **Mixing concurrency and parallelism** 鈥?understand the difference

Concurrency patterns help you write correct, composable, and maintainable concurrent code.
