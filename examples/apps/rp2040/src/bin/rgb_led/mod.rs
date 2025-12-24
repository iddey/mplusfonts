mod writer;
mod ws2812;

use cortex_m::asm;
use embassy_futures::block_on;
use embassy_rp::peripherals::PIO0;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, with_timeout};
use rgb::Rgb;

pub use writer::*;
pub use ws2812::*;

const RED: Rgb<u8> = Rgb::new(12, 0, 0);
const GREEN: Rgb<u8> = Rgb::new(0, 7, 0);
const BLUE: Rgb<u8> = Rgb::new(0, 0, 15);

static DRIVER: Mutex<CriticalSectionRawMutex, Option<Ws2812<'_, PIO0, 0>>> = Mutex::new(None);
static SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

async fn set_color(color: Rgb<u8>) {
    let mut guard = DRIVER.lock().await;

    if let Some(driver) = guard.as_mut() {
        driver.write_mut(&mut [color]).await;
    }
}

async fn reset() {
    set_color(Default::default()).await;
}

pub fn init(mut driver: Ws2812<'static, PIO0, 0>) {
    if let Ok(ref mut guard) = DRIVER.try_lock() {
        guard.replace(driver);
    } else {
        block_on(driver.write_mut(&mut [RED]));

        loop {
            asm::nop();
        }
    }
}

pub fn red_halt() -> ! {
    block_on(set_color(RED));

    loop {
        asm::nop();
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
