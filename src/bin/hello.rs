#![no_main]
#![no_std]

use template_stm32f3_rust as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    template_stm32f3_rust::exit()
}
