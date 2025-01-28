#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32l5::stm32l552;
use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
  let peripherals = stm32l552::Peripherals::take().unwrap();
  let gpioc = &peripherals.GPIOC;
  let rcc = &peripherals.RCC;

  // Setup clocks
  rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());
  rcc.apb1enr1.modify(|_, w| w.pwren().set_bit());

  rcc.ahb2enr.modify(|_, w| w.gpiocen().set_bit());
  rcc.ahb2enr.read().gpiocen(); // read to let enabling be done

  // Setup button PC13
    // No pull
  gpioc.pupdr.modify(|_, w| w.pupdr13().floating());
    // Mode input
  gpioc.moder.modify(|_, w| w.moder13().input());
  // Setup LED PC7
    // Push pull
  gpioc.otyper.modify(|_, w| w.ot7().push_pull());
    // Mode output
  gpioc.moder.modify(|_, w| w.moder7().output());

  // infinite loop; just so we don't leave this stack frame
  loop 
  {
    // if button pressed (PC13), light up led(PC7), otherwise no
    if gpioc.idr.read().idr13().bit_is_set()
    {
      gpioc.odr.write(|w| w.odr7().clear_bit());
    }
    else
    {
      gpioc.odr.write(|w| w.odr7().set_bit());
    }
  }
}
