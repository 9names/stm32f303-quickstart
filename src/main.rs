#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f3::stm32f303;
const CPU_FREQ:u32 = 72_000_000; // 72 mhz
#[entry]
fn main() -> ! {
    info!("Program start");
    // These peripherals are an integral part of the cortex-m core
    let core = stm32f303::CorePeripherals::take().unwrap();
    // These peripherals are external to the core - all the vendor added peripherals (gpio/spi/i2c/uart) live here
    let _peris = stm32f303::Peripherals::take().unwrap();
    // Set up the core systick peripheral to allow delays specified in SI time units
    let mut delay = cortex_m::delay::Delay::new(core.SYST, CPU_FREQ);

    loop {
        // your code goes here
        // led.toggle();
        // sleep for 1 second between loops
        delay.delay_ms(1000);
    }
}
