# Asynchronous Programming - Personal Study Notes

This document captures my personal study and mental model of **asynchronous programming**, **concurrency**, and **task
scheduling** as they relate to modern programming languages and operating systems.

My objective is **deep systems understanding**, not surface-level syntax.

---

## Core Thesis

> **Async/Await, Futures, Fibers, Goroutines, Promises**  
> are **abstractions** that make asynchronous programming:
>
> - easier to user
> - hard to misuse
>
> These abstractions **do not change execution fundamentals**.

At the lowest level, everything still runs on:

- CPU cores
- threads
- stacks
- schedulers
- context switches

Async abstractions only change **how concurrency is modeled and scheduled**.

---

## Multithreading: The Physical Reality

Multithreading
├─ Concurrency (many tasks in progress)
└─ Parallelism (many tasks executing simultaneously)

### Concurrency vs Parallelism

- **Concurrency**
    - Multiple tasks are *in progress*
    - Possible on a **single CPU core**
    - Achieved via interleaving execution

- **Parallelism**
    - Multiple tasks execute *at the same time*
    - Requires **multiple CPU cores**

> A system can be concurrent without being parallel.  
> A system can be both concurrent and parallel.

---

## Fundamental Definitions

### 1. Cooperative (Yielding) Tasks

**Definition**  
Tasks voluntarily give up control of execution.

They explicitly signal:
> “I am waiting please run something else.”

**Characteristics**

- Scheduler does **not** forcibly interrupt tasks
- Tasks must **explicitly yield**
- Common in async runtimes

**Examples**

- `async/await`
- event loops
- coroutines
- fibers

**Trade-offs**

- Low overhead
- Predictable execution
- Risk of starvation if tasks never yield

---

### 2. Non-Cooperative (Preemptive) Tasks

**Definition**  
Tasks are interrupted by the scheduler without their consent.

**Characteristics**

- Scheduler can preempt execution at almost any point
- Requires saving and restoring CPU state

**Examples**

- OS threads
- POSIX threads
- Java native threads

**Trade-offs**

- Fair scheduling
- True CPU parallelism
- Higher overhead
- Requires synchronization primitives (locks, atomics)

---

### 3. Stackful Tasks

**Definition**  
Each task owns its **own stack**.

Task A → Stack A
Task B → Stack B

**Implications**

- Tasks can suspend at arbitrary points
- Local variables are naturally preserved
- Code appears synchronous

**Examples**

- OS threads
- Goroutines
- Fibers
- Green threads

**Cost**

- Per-task stack memory
- Stack switching overhead

---

### 4. Stackless Tasks

**Definition**  
Tasks do **not** have their own stack.

They are compiled into **Finite state machines** and resume execution using the thread’s stack.

**Examples**

- JavaScript Promises
- `async/await` (compiled form)
- Rust `Future`
- C# async state machines

**Threads**

Each Thread has it own stack

Note: Each Os thred comes with its own stack, and even though many systems allow this size to be configured, they are
still fixed in size and can't grow or shrink. They are after all, the cause of **STACK OVERFLOW**





