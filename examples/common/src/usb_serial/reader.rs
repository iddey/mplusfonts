use embassy_sync::pubsub::DynPublisher;
use embassy_usb::class::cdc_acm::Receiver;
use embassy_usb::driver::Driver;
use embassy_usb::driver::EndpointError::{BufferOverflow, Disabled};
use heapless::Vec;

pub struct UsbHostReader<'d, D: Driver<'d>, const N: usize> {
    receiver: Receiver<'d, D>,
    publisher: DynPublisher<'d, Vec<u8, N>>,
}

impl<'d, D: Driver<'d>, const N: usize> UsbHostReader<'d, D, N> {
    pub fn new(receiver: Receiver<'d, D>, publisher: DynPublisher<'d, Vec<u8, N>>) -> Self {
        Self {
            receiver,
            publisher,
        }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            self.receiver.wait_connection().await;

            loop {
                let mut message = Vec::from_array([0; N]);
                let result = self.receiver.read_packet(&mut message).await;

                match result {
                    // SAFETY: The underlying slice has its elements initialized.
                    Ok(n) if n <= N => unsafe { message.set_len(n) },
                    Ok(_) => unreachable!(),
                    Err(BufferOverflow) => panic!("Buffer overflow: USB control request data"),
                    Err(Disabled) => break,
                }

                self.publisher.publish(message).await;
            }
        }
    }
}
