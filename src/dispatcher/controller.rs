use crate::{prelude::Consumer, prelude::Producer};
use tokio::select;
use tracing::info;
pub struct Controller<C, P, T>
where
    C: Consumer<T>,
    P: Producer<T>,
{
    consumer: Box<C>,
    producer: Box<P>,
    _marker: std::marker::PhantomData<T>,
}

impl<C, P, T> Controller<C, P, T>
where
    C: Consumer<T>,
    P: Producer<T>,
{
    pub fn new(consumer: Box<C>, producer: Box<P>) -> Self {
        Self {
            consumer: consumer,
            producer: producer,
            _marker: std::marker::PhantomData,
        }
    }
    pub async fn run(&mut self) {
        info!("Controller running");
        loop {
            select! {
                act = self.producer.produce() => {
                    match act {
                        Ok(act) => {
                            self.consumer.consume(&act).await;
                        }
                        Err(_) => {
                            info!("Producer stopped");
                            self.consumer.stop().await;

                            break;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestConsumer<T> {
        _marker: std::marker::PhantomData<T>,
    }

    struct TestProducer<T> {
        producer_rcv: tokio::sync::mpsc::Receiver<T>,
        producer_snd: tokio::sync::mpsc::Sender<T>,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for TestConsumer<T> {
        fn default() -> Self {
            Self {
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<T> Default for TestProducer<T> {
        fn default() -> Self {
            let (producer_snd, producer_rcv) = tokio::sync::mpsc::channel(32);
            Self {
                producer_rcv: producer_rcv,
                producer_snd: producer_snd,
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<T: Send + Sync> Consumer<T> for TestConsumer<T> {
        fn consume(&mut self, _action: &T) -> impl std::future::Future<Output = ()> + Send {
            tracing::info!("Consuming _action");
            async {}
        }

        fn stop(&mut self) -> impl std::future::Future<Output = ()> + Send {
            tracing::info!("Stopping consumer");
            async {}
        }
    }

    impl<T: Send + Sync> Producer<T> for TestProducer<T> {
        async fn produce(&mut self) -> anyhow::Result<T> {
            match self.producer_rcv.recv().await {
                Some(item) => Ok(item),
                None => Err(anyhow::anyhow!("Producer closed")),
            }
        }
    }

    #[tokio::test]
    async fn test_controller_new() {
        let consumer: Box<TestConsumer<String>> = Box::default();
        let producer: Box<TestProducer<String>> = Box::default();
        let controller = Controller::new(consumer, producer);

        // Verify controller is created successfully
        // This mainly ensures the trait bounds are satisfied
        let _ = controller;
    }

    #[tokio::test]
    async fn test_controller_run() {
        let consumer: Box<TestConsumer<String>> = Box::default();
        let producer: Box<TestProducer<String>> = Box::default();
        let sender = producer.producer_snd.clone();
        let mut controller = Controller::new(consumer, producer);
        tokio::spawn(async move {
            controller.run().await;
        });
        for i in 0..10 {
            let _ = sender.send(format!("{}", i)).await;
        }
    }
}
