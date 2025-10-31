use embassy_sync::pubsub::DynSubscriber;
use embassy_sync::pubsub::WaitResult::Message;

use super::SIGNAL;

pub struct UnitWriter<'a, T: Clone> {
    subscriber: DynSubscriber<'a, T>,
}

impl<'a, T: Clone> UnitWriter<'a, T> {
    pub fn new(subscriber: DynSubscriber<'a, T>) -> Self {
        Self { subscriber }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let result = self.subscriber.next_message().await;

            let Message(_) = result else {
                continue;
            };

            SIGNAL.signal(());
        }
    }
}
