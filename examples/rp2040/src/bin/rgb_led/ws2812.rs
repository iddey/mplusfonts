use embassy_rp::Peri;
use embassy_rp::clocks;
use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::pio::{Common, Instance, PioPin, StateMachine};
use embassy_rp::pio::{Config, FifoJoin, ShiftConfig, ShiftDirection};
use embassy_time::Timer;
use fixed::types::U24F8;
use fixed_macro::fixed;
use rgb::{ComponentSlice, Rgb};

pub trait WriteMut {
    async fn write_mut(&mut self, colors: &mut [Rgb<u8>]);
}

pub struct Ws2812<'d, P: Instance, const S: usize> {
    sm: StateMachine<'d, P, S>,
    dma: Peri<'d, AnyChannel>,
}

impl<'d, P: Instance, const S: usize> Ws2812<'d, P, S> {
    pub fn new(
        pio: &mut Common<'d, P>,
        mut sm: StateMachine<'d, P, S>,
        dma: Peri<'d, impl Channel>,
        pin: Peri<'d, impl PioPin>,
    ) -> Self {
        let side_set = pio::SideSet::new(false, 1, false);
        let mut assembler: pio::Assembler<32> = pio::Assembler::new_with_side_set(side_set);

        const T1: u8 = 3;
        const T2: u8 = 3;
        const T3: u8 = 4;

        let clk_sys_freq = U24F8::from_num(clocks::clk_sys_freq() / 1000);
        let bit_freq = fixed!(800: U24F8) * (T1 + T2 + T3) as u32;

        let mut bitloop = assembler.label();
        let mut do_zero = assembler.label();
        let mut wrap = assembler.label();

        assembler.set_with_side_set(pio::SetDestination::PINDIRS, 1, 0);
        assembler.bind(&mut bitloop);
        assembler.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 - 1, 0);
        assembler.jmp_with_delay_and_side_set(pio::JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);
        assembler.jmp_with_delay_and_side_set(pio::JmpCondition::Always, &mut bitloop, T2 - 1, 1);
        assembler.bind(&mut do_zero);
        assembler.nop_with_delay_and_side_set(T2 - 1, 0);
        assembler.bind(&mut wrap);

        let program = assembler.assemble_with_wrap(wrap, bitloop);
        let program = pio.load_program(&program);
        let pin_out = pio.make_pio_pin(pin);
        let pins = [&pin_out];

        let mut config = Config::default();
        config.set_out_pins(&pins);
        config.set_set_pins(&pins);
        config.use_program(&program, &pins);
        config.clock_divider = clk_sys_freq / bit_freq;
        config.fifo_join = FifoJoin::TxOnly;
        config.shift_out = ShiftConfig {
            auto_fill: true,
            threshold: 8,
            direction: ShiftDirection::Left,
        };

        sm.set_config(&config);
        sm.set_enable(true);

        let dma = dma.into();

        Self { sm, dma }
    }
}

impl<P: Instance, const S: usize> WriteMut for Ws2812<'_, P, S> {
    async fn write_mut(&mut self, colors: &mut [Rgb<u8>]) {
        let dma = self.dma.reborrow();
        let bytes = colors.as_mut_slice();

        for chunk in bytes.chunks_mut(3) {
            chunk.swap(0, 1);
        }

        self.sm.tx().dma_push(dma, bytes, false).await;

        Timer::after_micros(55).await;
    }
}
