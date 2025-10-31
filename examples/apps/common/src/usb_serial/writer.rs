use embassy_sync::pubsub::DynSubscriber;
use embassy_sync::pubsub::WaitResult::Message;
use embassy_usb::class::cdc_acm::Sender;
use embassy_usb::driver::Driver;
use embassy_usb::driver::EndpointError::{BufferOverflow, Disabled};
use heapless::Vec;

pub struct UsbHostWriter<'d, D: Driver<'d>, const N: usize> {
    sender: Sender<'d, D>,
    subscriber: DynSubscriber<'d, Vec<u8, N>>,
}

impl<'d, D: Driver<'d>, const N: usize> UsbHostWriter<'d, D, N> {
    pub fn new(sender: Sender<'d, D>, subscriber: DynSubscriber<'d, Vec<u8, N>>) -> Self {
        Self { sender, subscriber }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let result = self.subscriber.next_message().await;

            let Message(message) = result else {
                continue;
            };

            loop {
                let result = self.sender.write_packet(&message).await;

                match result {
                    Ok(()) => break,
                    Err(BufferOverflow) => panic!("Buffer overflow: USB control request data"),
                    Err(Disabled) => self.sender.wait_connection().await,
                }
            }
        }
    }
}
