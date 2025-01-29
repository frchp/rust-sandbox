#![no_main]
#![no_std]

use stm32l5::stm32l552::{self, interrupt, Interrupt, NVIC};
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::interrupt::{free, Mutex};
use core::cell::RefCell;
use core::ops::DerefMut;

// Create mutex to access register during interrupts
static G_TIM3: Mutex<RefCell<Option<stm32l552::TIM3>>> = Mutex::new(RefCell::new(None));
static G_GPIOC: Mutex<RefCell<Option<stm32l552::GPIOC>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
  let peripherals = stm32l552::Peripherals::take().unwrap();
  let gpioc = peripherals.GPIOC;
  let rcc = &peripherals.RCC;
  let timer3 = peripherals.TIM3;

  // Setup clocks
  rcc.ahb2enr.modify(|_, w| w.gpiocen().set_bit());
  rcc.ahb2enr.read().gpiocen(); // read to let enabling be done
  rcc.apb1enr1.modify(|_, w| w.tim3en().set_bit());
  rcc.apb1enr1.read().tim3en(); // read to let enabling be done

  // Setup TIMER3
  free(|cs|
    {
    timer3.cr1.write(|w| w.cen().clear_bit()); // disable timer
    timer3.psc.write(|w| w.psc().variant(30)); // set prescaler for 500ms period @f 4MHz = MSI @rst
    timer3.arr.modify(|_, w| w.arr_l().variant(0xFFFF)); // set arr value for 500ms period @f 4MHz = MSI @rst
    timer3.dier.write(|w| w.uie().set_bit()); // enable update interrupt
    timer3.cr1.modify(|_, w| w.arpe().set_bit()); // enable arr preload
    timer3.cr1.modify(|_, w| w.cen().set_bit()); // enable timer
    timer3.egr.write(|w| w.ug().set_bit()); // update generation
    // We can no longer use this variable, and instead have to access it via the mutex.
    G_TIM3.borrow(cs).replace(Some(timer3));
  });
  // Setup LED PC7
  free(|cs|
  {
    gpioc.otyper.modify(|_, w| w.ot7().push_pull()); // Push pull
    gpioc.moder.modify(|_, w| w.moder7().output()); // Mode output
    // We can no longer use this variable, and instead have to access it via the mutex.
    G_GPIOC.borrow(cs).replace(Some(gpioc));
  });
  // Setup NVIC interrupt
  unsafe
  {
    NVIC::unmask(Interrupt::TIM3);
  };

  loop
  {
    cortex_m::asm::wfi();
  }
}

#[interrupt]
fn TIM3()
{
  // Clear interrupt
  free(|cs|
  {
    if let Some(ref mut tim3) = G_TIM3.borrow(cs).borrow_mut().deref_mut()
    {
      if tim3.sr.read().uif().bit_is_set()
      {
        tim3.sr.modify(|_, w| w.uif().clear_bit()); // Clear update interrupt flag
      }
    }
  });

  // Toggle LED
  free(|cs|
  {
    if let Some(ref mut gpioc) = G_GPIOC.borrow(cs).borrow_mut().deref_mut()
    {
      if gpioc.odr.read().odr7().bit_is_set()
      {
        gpioc.odr.write(|w| w.odr7().clear_bit());
      }
      else
      {
        gpioc.odr.write(|w| w.odr7().set_bit());
      }
    }
  });
}