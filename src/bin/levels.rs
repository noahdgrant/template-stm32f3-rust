#![no_main]
#![no_std]

use template_stm32f3_rust as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    // try setting the DEFMT_LOG environment variable
    // e.g. `export DEFMT_LOG=info` or `DEFMT_LOG=trace cargo rb levels`
    defmt::error!("error");
    defmt::warn!("warn");
    defmt::info!("info");
    defmt::debug!("debug");
    defmt::trace!("trace");

    template_stm32f3_rust::exit()
}
