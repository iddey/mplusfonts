use embassy_sync::pubsub::DynSubscriber;
use embassy_sync::pubsub::WaitResult::Message;
use embedded_io_async::Write;
use esp_hal::Async;
use esp_hal::usb_serial_jtag::UsbSerialJtagTx;
use heapless::Vec;

pub struct UsbHostWriter<'a, const N: usize> {
    sender: UsbSerialJtagTx<'a, Async>,
    subscriber: DynSubscriber<'a, Vec<u8, N>>,
}

impl<'a, const N: usize> UsbHostWriter<'a, N> {
    pub fn new(
        sender: UsbSerialJtagTx<'a, Async>,
        subscriber: DynSubscriber<'a, Vec<u8, N>>,
    ) -> Self {
        Self { sender, subscriber }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let result = self.subscriber.next_message().await;

            let Message(message) = result else {
                continue;
            };

            self.sender.write_all(&message).await.unwrap();
            self.sender.flush().await.unwrap();
        }
    }
}
