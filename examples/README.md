# Lyo Examples

This directory contains examples demonstrating how to use the `lyo` crate for automatic task distribution.

## Running Examples

Each example can be run using `cargo run`:

```bash
cargo run --example simple
cargo run --example task_queue
cargo run --example data_pipeline
```

## Examples

### simple.rs

A basic example that demonstrates the core producer-consumer pattern with the `Controller`.

**What it shows:**
- Creating a custom `Producer` that generates messages
- Creating a custom `Consumer` that processes messages
- Using `Controller::new()` to wire them together
- Running the controller with the `run()` method

**Output:** Prints each message as it's consumed, then stops when the producer is exhausted.

---

### task_queue.rs

A practical example simulating a task queue with prioritized tasks.

**What it shows:**
- Producing tasks with different priority levels
- Variable processing times based on task priority
- Tracking the total number of processed tasks
- Proper shutdown with statistics

**Output:** Shows task processing with priority levels and estimated processing times.

---

### data_pipeline.rs

A realistic example of a data processing pipeline that fetches data from multiple sources.

**What it shows:**
- Multiple data sources (simulated sensors)
- Data transformation and analysis
- Statistics tracking (total processed, sum, average)
- Simulated network delays and processing times

**Output:** Displays processed data with analysis results and a summary at the end.

## Creating Your Own Example

To create your own producer-consumer setup:

1. Define your data type `T` that will flow through the system
2. Implement `Producer<T>` for your producer with a `produce()` method
3. Implement `Consumer<T>` for your consumer with `consume()` and `stop()` methods
4. Create a `Controller` with `Controller::new(consumer, producer)`
5. Call `controller.run().await` to start processing

```rust
use lyo::prelude::*;

struct MyData { /* ... */ }

struct MyProducer;
impl Producer<MyData> for MyProducer {
    async fn produce(&mut self) -> anyhow::Result<MyData> {
        // Your production logic
    }
}

struct MyConsumer;
impl Consumer<MyData> for MyConsumer {
    async fn consume(&self, item: &MyData) {
        // Your consumption logic
    }

    async fn stop(&self) {
        // Cleanup logic
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer: Box<MyProducer> = Box::new(MyProducer);
    let consumer: Box<MyConsumer> = Box::new(MyConsumer);
    let mut controller = Controller::new(consumer, producer);
    controller.run().await;
    Ok(())
}
```
