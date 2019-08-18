## Dining Philosophers Problem

That's it.

Ref: https://www.wikiwand.com/en/Dining_philosophers_problem

### What I learned

- [`Thread`](https://doc.rust-lang.org/std/thread/index.html) in Rust
- Usage of [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) and [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

### Run

```
$ cargo run
```

### Output: DEADLOCK 😇

```
[Running 'cargo run']
   Compiling dining-philosophers v0.1.0 (/Users/kuy/Work/dining-philosophers)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/dining-philosophers`
Philosopher 1: Thinking...
Philosopher 2: Thinking...
Philosopher 4: Thinking...
Philosopher 3: Thinking...
Philosopher 5: Thinking...
Chopstick 3: Picked up by Philosopher 3
Philosopher 3: OOPS! Right chopstick not available. Waiting...
Chopstick 1: Picked up by Philosopher 1
Philosopher 1: OOPS! Right chopstick not available. Waiting...
Chopstick 2: Picked up by Philosopher 2
Philosopher 2: OOPS! Right chopstick not available. Waiting...
Chopstick 5: Picked up by Philosopher 5
Philosopher 5: OOPS! Right chopstick not available. Waiting...
Chopstick 4: Picked up by Philosopher 4
Philosopher 4: OOPS! Right chopstick not available. Waiting...
```

### Output: Resolved 😆

```
[Running 'cargo run']
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/dining-philosophers`
Philosopher 1: Thinking...
Philosopher 5: Thinking...
Philosopher 2: Thinking...
Philosopher 4: Thinking...
Philosopher 3: Thinking...
Chopstick 1: Picked up by Philosopher 1
Chopstick 2: Picked up by Philosopher 2
Chopstick 3: Picked up by Philosopher 2
Philosopher 3: OOPS! Left chopstick not available. Waiting...
Philosopher 1: OOPS! Right chopstick not available. Waiting...
Philosopher 2: Eating...
Chopstick 5: Picked up by Philosopher 5
Philosopher 5: OOPS! Right chopstick not available. Waiting...
Chopstick 4: Picked up by Philosopher 4
Philosopher 4: OOPS! Right chopstick not available. Waiting...
Chopstick 3: Released by Philosopher 2
Chopstick 2: Released by Philosopher 2
Philosopher 2: Thinking...
Chopstick 3: Picked up by Philosopher 3
Philosopher 3: OOPS! Right chopstick not available. Waiting...
Chopstick 2: Picked up by Philosopher 1
Philosopher 1: Eating...
Philosopher 2: OOPS! Left chopstick not available. Waiting...
Chopstick 2: Released by Philosopher 1
Chopstick 1: Released by Philosopher 1
Philosopher 1: Thinking...
Chopstick 2: Picked up by Philosopher 2
Philosopher 2: OOPS! Right chopstick not available. Waiting...
Chopstick 1: Picked up by Philosopher 5
Philosopher 5: Eating...
Philosopher 1: OOPS! Left chopstick not available. Waiting...
Chopstick 1: Released by Philosopher 5
Chopstick 5: Released by Philosopher 5
Philosopher 5: Thinking...
Chopstick 1: Picked up by Philosopher 1
Philosopher 1: OOPS! Right chopstick not available. Waiting...
Chopstick 5: Picked up by Philosopher 4
Philosopher 4: Eating...

...
```
