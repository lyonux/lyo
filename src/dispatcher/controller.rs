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
