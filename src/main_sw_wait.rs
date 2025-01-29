#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32l5::stm32l552;
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::asm::nop;

#[entry]
fn main() -> ! {
  let peripherals = stm32l552::Peripherals::take().unwrap();
  let gpioc = &peripherals.GPIOC;
  let rcc = &peripherals.RCC;

  // Setup clocks
  rcc.ahb2enr.modify(|_, w| w.gpiocen().set_bit());
  rcc.ahb2enr.read().gpiocen(); // read to let enabling be done

  // Setup LED PC7
    // Push pull
  gpioc.otyper.modify(|_, w| w.ot7().push_pull());
    // Mode output
  gpioc.moder.modify(|_, w| w.moder7().output());

  loop 
  {
    let mut i = 4000000;

    while i != 0 {
      nop();
      i = i - 1;
    }
    if gpioc.odr.read().odr7().bit_is_set()
    {
      gpioc.odr.write(|w| w.odr7().clear_bit());
    }
    else
    {
      gpioc.odr.write(|w| w.odr7().set_bit());
    }
  }
}