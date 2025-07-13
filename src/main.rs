#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop {}
}
