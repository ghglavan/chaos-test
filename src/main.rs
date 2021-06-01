#![no_std]
#![no_main]
#![feature(asm)]

use chaos;

pub use stm32f3_discovery::{leds::Leds, stm32f3xx_hal, switch_hal};
use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::{
    delay::Delay,
    gpio::{gpioe, Output, PushPull},
    hal::blocking::delay::DelayMs,
    stm32,
};

use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::{self, Mutex};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use cortex_m::Peripherals;
use cortex_m::{asm, register};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::*;
use panic_semihosting as _;


static mut TASK0_COUNTER: u32 = 0;
static mut TASK1_COUNTER: u32 = 0;
static mut TASK2_COUNTER: u32 = 0;


#[chaos::os(quanta_us = 10_000, scheduler = chaos::scheduler::RRScheduler)]
mod chaos {
    #[task(stack_size = 500)]
    fn task0() {
        loop {
            unsafe {
                TASK0_COUNTER += 1;
                chaos::syscalls::sleep();
            }
            continue;
        }
    }

    #[task(stack_size = 200)]
    fn task1() {
        loop {
            unsafe {
                TASK1_COUNTER += 1;
            }
            continue;
        }
    }

    #[task(stack_size = 200)]
    fn task2() {
        loop {
            unsafe {
                TASK2_COUNTER += 1;
            }
            continue;
        }
    }

    #[init]
    fn init() {
    }
}
