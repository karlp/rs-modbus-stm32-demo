//#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
use panic_rtt_target as _;
// use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
extern crate rtic;
extern crate stm32l1xx_hal as hal;

use rtic::app;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::ToggleableOutputPin;
use hal::exti::TriggerEdge;
use hal::gpio::gpiob::{PB8, PB7};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use hal::timer::Timer;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        // resources
        #[init(0)]
        DELTA: u32,

        // late resources
        TIMER: Timer<stm32::TIM2>,
        TICKS_LED: PB8<Output<PushPull>>,
        EXTI: stm32::EXTI,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_init_print!(NoBlockSkip, 4096);

        let mut rcc = cx.device.RCC.freeze(Config::hsi());

        let gpiob = cx.device.GPIOB.split();
        let mut timer = cx.device.TIM2.timer(1.hz(), &mut rcc);

        timer.listen();
        cx.device.EXTI.listen(0, TriggerEdge::Rising);

        let TICKS_LED = gpiob.pb8.into_push_pull_output();
        let TIMER = timer;
        let EXTI = cx.device.EXTI;

        init::LateResources {
            TIMER,
            TICKS_LED,
            EXTI,
        }
    }

    #[task(binds = TIM2, resources = [TIMER, TICKS_LED, DELTA])]
    fn tim2_handler(cx: tim2_handler::Context) {
        *cx.resources.DELTA += 1;

        cx.resources.TICKS_LED.toggle().unwrap();
        cx.resources.TIMER.clear_irq();
        rprintln!("blink: {}", cx.resources.DELTA)
    }


    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
};