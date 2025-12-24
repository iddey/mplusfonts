use embassy_sync::pubsub::DynPublisher;
use embedded_io_async::Read;
use esp_hal::Async;
use esp_hal::usb_serial_jtag::UsbSerialJtagRx;
use heapless::Vec;

pub struct UsbHostReader<'a, const N: usize> {
    receiver: UsbSerialJtagRx<'a, Async>,
    publisher: DynPublisher<'a, Vec<u8, N>>,
}

impl<'a, const N: usize> UsbHostReader<'a, N> {
    pub fn new(
        receiver: UsbSerialJtagRx<'a, Async>,
        publisher: DynPublisher<'a, Vec<u8, N>>,
    ) -> Self {
        Self {
            receiver,
            publisher,
        }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let mut message = Vec::from_array([0; N]);

            let n = self.receiver.read(&mut message).await.unwrap();

            // SAFETY: The underlying slice has its elements initialized.
            unsafe { message.set_len(n) };

            self.publisher.publish(message).await;
        }
    }
}
