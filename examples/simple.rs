use lyo::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

// A simple message type
struct Message {
    id: u32,
    content: String,
}

// Producer that generates messages
struct MessageProducer {
    counter: u32,
    max_messages: u32,
}

impl MessageProducer {
    fn new(max_messages: u32) -> Self {
        Self {
            counter: 0,
            max_messages,
        }
    }
}

impl Producer<Message> for MessageProducer {
    async fn produce(&mut self) -> anyhow::Result<Message> {
        if self.counter >= self.max_messages {
            return Err(anyhow::anyhow!("No more messages to produce"));
        }

        self.counter += 1;
        sleep(Duration::from_millis(100)).await;

        Ok(Message {
            id: self.counter,
            content: format!("Message #{}", self.counter),
        })
    }
}

// Consumer that processes messages
struct MessageConsumer;

impl Consumer<Message> for MessageConsumer {
    async fn consume(&mut self, msg: &Message) {
        println!("Consuming: {} - {}", msg.id, msg.content);
    }

    async fn stop(&mut self) {
        println!("Consumer stopped");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Starting lyo example...");

    // Create producer and consumer
    let producer: Box<MessageProducer> = Box::new(MessageProducer::new(5));
    let consumer: Box<MessageConsumer> = Box::new(MessageConsumer);

    // Create and run the controller
    let mut controller = Controller::new(consumer, producer);
    controller.run().await;

    println!("Example completed!");

    Ok(())
}
