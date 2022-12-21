#![no_main]
#![no_std]

use core::panic::PanicInfo;
use printf_compat as _; // ensure it gets linked

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
