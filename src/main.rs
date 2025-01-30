#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32f4::stm32f407;
use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
  let peripherals = stm32f407::Peripherals::take().unwrap();
  let gpiod = &peripherals.GPIOD;
  let gpioa = &peripherals.GPIOA;
  let rcc = &peripherals.RCC;

  // Setup clocks
  rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
  rcc.ahb1enr.read().gpioaen(); // read to let enabling be done
  rcc.ahb1enr.modify(|_, w| w.gpioden().set_bit());
  rcc.ahb1enr.read().gpioden(); // read to let enabling be done

  // Setup button PC13
    // No pull
  gpioa.pupdr.modify(|_, w| w.pupdr0().floating());
    // Mode input
  gpioa.moder.modify(|_, w| w.moder0().input());
  // Setup LED PC7
    // Push pull
  gpiod.otyper.modify(|_, w| w.ot12().push_pull());
    // Mode output
  gpiod.moder.modify(|_, w| w.moder12().output());

  // infinite loop; just so we don't leave this stack frame
  loop
  {
    // if button pressed (PA0), light up led(PD12), otherwise no
    let _bits = gpioa.idr.read().idr0().bit_is_set();
    if gpioa.idr.read().idr0().bit_is_set()
    {
      gpiod.odr.write(|w| w.odr12().clear_bit());
    }
    else
    {
      gpiod.odr.write(|w| w.odr12().set_bit());
    }
  }
}