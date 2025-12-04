use core::fmt::{Display, Formatter, Result, Write};

use embassy_sync::pubsub::DynPublisher;
use embassy_time::{Duration, Ticker};
use heapless::{String, format};

struct Component<const MAX: u8>(u8);

struct NaiveTime {
    hours: Component<23>,
    minutes: Component<59>,
    seconds: Component<59>,
}

impl<const MAX: u8> Component<MAX> {
    pub fn new(value: u8) -> Self {
        Self(value.clamp(0, MAX))
    }

    pub fn increment(&mut self) -> bool {
        let Self(value) = self;
        if *value < MAX {
            *value += 1;

            false
        } else {
            *value = 0;

            true
        }
    }
}

impl<const MAX: u8> Display for Component<MAX> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let Self(value) = self;
        if *value < 100 {
            for digit in [value / 10, value % 10].map(Into::into) {
                formatter.write_char(char::from_digit(digit, 10).unwrap())?;
            }
        } else {
            for digit in [value / 100, value / 10 % 10, value % 10].map(Into::into) {
                formatter.write_char(char::from_digit(digit, 10).unwrap())?;
            }
        };

        Ok(())
    }
}

impl NaiveTime {
    pub fn new(hours: u8, minutes: u8, seconds: u8) -> Self {
        Self {
            hours: Component::new(hours),
            minutes: Component::new(minutes),
            seconds: Component::new(seconds),
        }
    }

    pub fn increment(&mut self) -> bool {
        self.seconds.increment() && self.minutes.increment() && self.hours.increment()
    }
}

pub struct Clock<'a> {
    time: NaiveTime,
    ticker: Ticker,
    publisher: DynPublisher<'a, String<8>>,
}

impl<'a> Clock<'a> {
    pub fn new(
        hours: u8,
        minutes: u8,
        seconds: u8,
        publisher: DynPublisher<'a, String<8>>,
    ) -> Self {
        Self {
            time: NaiveTime::new(hours, minutes, seconds),
            ticker: Ticker::every(Duration::from_secs(1)),
            publisher,
        }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let NaiveTime {
                hours,
                minutes,
                seconds,
            } = &self.time;

            let digits = format!("{hours}:{minutes}:{seconds}").unwrap();

            self.publisher.publish_immediate(digits);

            let _ = self.time.increment();

            self.ticker.next().await;
        }
    }
}
