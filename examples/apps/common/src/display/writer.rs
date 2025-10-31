use core::fmt::Debug;

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::pubsub::Subscriber;
use embassy_sync::pubsub::WaitResult::Message;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use heapless::Vec;

use super::rect::RectangleExt;

pub struct TextWriter<
    'a,
    D: DrawTarget,
    T: TextRenderer<Color = D::Color> + Clone,
    M: RawMutex,
    const N: usize,
    const CAP: usize,
    const SUBS: usize,
    const PUBS: usize,
> where
    D::Error: Debug,
{
    pub target: D,
    pub position: Point,
    pub max_size: Size,
    pub renderer: T,
    pub baseline: Baseline,
    pub bg_color: D::Color,
    pub flush_fn: fn(&mut D) -> Result<(), D::Error>,
    pub subscriber: Subscriber<'a, M, Vec<u8, N>, CAP, SUBS, PUBS>,
}

impl<
    D: DrawTarget,
    T: TextRenderer<Color = D::Color> + Clone,
    M: RawMutex,
    const N: usize,
    const CAP: usize,
    const SUBS: usize,
    const PUBS: usize,
> TextWriter<'_, D, T, M, N, CAP, SUBS, PUBS>
where
    D::Error: Debug,
{
    pub fn clear(&mut self) {
        let clip_area = Rectangle::new(self.position, self.max_size);
        let mut target = self.target.clipped(&clip_area);
        target.clear(self.bg_color).unwrap();

        let flush = self.flush_fn;
        flush(&mut self.target).unwrap();
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let result = self.subscriber.next_message().await;

            let Message(message) = result else {
                continue;
            };

            let clip_area = Rectangle::new(self.position, self.max_size);
            let mut target = self.target.clipped(&clip_area);
            if let Ok(text) = str::from_utf8(&message) {
                let renderer = self.renderer.clone();
                let metrics = renderer.measure_string(text, self.position, self.baseline);
                let text = Text::with_baseline(text, self.position, renderer, self.baseline);
                let next_position = text.draw(&mut target).unwrap();
                assert_eq!(next_position, metrics.next_position);

                let left = target.bounding_box().left_of(&metrics.bounding_box);
                let right = target.bounding_box().right_of(&metrics.bounding_box);
                let middle = target.bounding_box().left_of(&right).right_of(&left);
                let above = middle.above(&metrics.bounding_box);
                let below = middle.below(&metrics.bounding_box);
                for area in [left, right, above, below] {
                    target.fill_solid(&area, self.bg_color).unwrap();
                }
            } else {
                target.clear(self.bg_color).unwrap();
            }

            let flush = self.flush_fn;
            flush(&mut self.target).unwrap();
        }
    }
}
