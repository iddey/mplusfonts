mod writer;

use core::arch::asm;

use embassy_futures::block_on;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, with_timeout};
use esp_hal_smartled::SmartLedsAdapter;
use smart_leds_trait::RGB as Rgb;
use smart_leds_trait::SmartLedsWrite;

pub use writer::*;

const RED: Rgb<u8> = Rgb::new(12, 0, 0);
const GREEN: Rgb<u8> = Rgb::new(0, 7, 0);
const BLUE: Rgb<u8> = Rgb::new(0, 0, 15);

static ADAPTER: Mutex<CriticalSectionRawMutex, Option<SmartLedsAdapter<'_, 25>>> = Mutex::new(None);
static SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

async fn set_color(color: Rgb<u8>) {
    let mut guard = ADAPTER.lock().await;

    if let Some(adapter) = guard.as_mut() {
        adapter.write([color]).unwrap();
    }
}

async fn reset() {
    set_color(Default::default()).await;
}

pub fn init(mut adapter: SmartLedsAdapter<'static, 25>) {
    if let Ok(ref mut guard) = ADAPTER.try_lock() {
        guard.replace(adapter);
    } else {
        adapter.write([RED]).unwrap();

        loop {
            unsafe { asm!("nop") }
        }
    }
}

pub fn red_halt() -> ! {
    block_on(set_color(RED));

    loop {
        unsafe { asm!("nop") }
    }
}

pub async fn blink() {
    'start: loop {
        set_color(BLUE).await;

        let result = with_timeout(Duration::from_millis(300), SIGNAL.wait()).await;

        if let Ok(()) = result {
            continue 'start;
        }

        loop {
            reset().await;

            let result = with_timeout(Duration::from_millis(2400), SIGNAL.wait()).await;

            if let Ok(()) = result {
                continue 'start;
            }

            set_color(GREEN).await;

            let result = with_timeout(Duration::from_millis(600), SIGNAL.wait()).await;

            if let Ok(()) = result {
                continue 'start;
            }
        }
    }
}
