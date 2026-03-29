use lyo::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

/// Represents a task to be processed
#[derive(Debug)]
struct Task {
    id: u32,
    priority: u8,
    description: String,
}

/// Producer that generates tasks with different priorities
struct TaskProducer {
    task_id: u32,
    total_tasks: u32,
}

impl TaskProducer {
    fn new(total_tasks: u32) -> Self {
        Self {
            task_id: 0,
            total_tasks,
        }
    }
}

impl Producer<Task> for TaskProducer {
    async fn produce(&mut self) -> anyhow::Result<Task> {
        if self.task_id >= self.total_tasks {
            return Err(anyhow::anyhow!("All tasks produced"));
        }

        self.task_id += 1;
        let priority = match self.task_id % 3 {
            0 => 1,  // High priority
            1 => 2,  // Medium priority
            _ => 3,  // Low priority
        };

        // Simulate variable production rate
        sleep(Duration::from_millis(50)).await;

        Ok(Task {
            id: self.task_id,
            priority,
            description: format!("Process task #{} with priority {}", self.task_id, priority),
        })
    }
}

/// Consumer that processes tasks
struct TaskConsumer {
    processed_count: std::sync::atomic::AtomicU32,
}

impl TaskConsumer {
    fn new() -> Self {
        Self {
            processed_count: std::sync::atomic::AtomicU32::new(0),
        }
    }

    fn count(&self) -> u32 {
        self.processed_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Consumer<Task> for TaskConsumer {
    async fn consume(&self, task: &Task) {
        // Simulate processing time based on priority
        let processing_time = match task.priority {
            1 => Duration::from_millis(200), // High priority takes longer
            2 => Duration::from_millis(100),
            _ => Duration::from_millis(50),
        };

        println!(
            "[Task #{}] Processing (priority: {}, estimated time: {:?})...",
            task.id, task.priority, processing_time
        );

        sleep(processing_time).await;

        self.processed_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        println!("[Task #{}] Completed!", task.id);
    }

    async fn stop(&self) {
        println!(
            "Task queue shutting down. Total processed: {}",
            self.count()
        );
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Task Queue Example ===\n");

    let total_tasks = 10;
    println!("Creating producer for {} tasks...\n", total_tasks);

    let producer: Box<TaskProducer> = Box::new(TaskProducer::new(total_tasks));
    let consumer: Box<TaskConsumer> = Box::new(TaskConsumer::new());

    println!("Starting task processing...\n");

    let mut controller = Controller::new(consumer, producer);
    controller.run().await;

    println!("\n=== All tasks completed ===");

    Ok(())
}
