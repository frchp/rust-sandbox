#![no_main]
#![no_std]

use stm32l5::stm32l552::{self, interrupt, Interrupt, NVIC};
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::interrupt::{free, Mutex};
use core::cell::RefCell;
use core::ops::DerefMut;

// Create mutex to access register during interrupts
static G_EXTI: Mutex<RefCell<Option<stm32l552::EXTI>>> = Mutex::new(RefCell::new(None));
static G_GPIOC: Mutex<RefCell<Option<stm32l552::GPIOC>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
  let peripherals = stm32l552::Peripherals::take().unwrap();
  let gpioc = &peripherals.GPIOC;
  let rcc = &peripherals.RCC;
  let exti = &peripherals.EXTI;

  // Setup clocks
  rcc.ahb2enr.modify(|_, w| w.gpiocen().set_bit());
  rcc.ahb2enr.read().gpiocen(); // read to let enabling be done

  // Setup button PC13
    // No pull
  gpioc.pupdr.modify(|_, w| w.pupdr13().floating());
    // Mode input
  gpioc.moder.modify(|_, w| w.moder13().input());
  // Setup EXTI for PC13
  exti.exticr4.modify(|_, w| w.exti8_15().variant(2)); // Port C
  exti.imr1.modify(|_, w| w.im13().set_bit());
  exti.ftsr1.modify(|_, w| w.ft13().set_bit());
  exti.fpr1.write(|w| w.fpif13().clear_bit());
  // Setup LED PC7
    // Push pull
  gpioc.otyper.modify(|_, w| w.ot7().push_pull());
    // Mode output
  gpioc.moder.modify(|_, w| w.moder7().output());
  // Setup NVIC interrupt
  unsafe
  {
    NVIC::unmask(Interrupt::EXTI13);
  };

  // We can no longer use those variables, and instead have to access it via the mutex.
  free(|cs| G_GPIOC.borrow(cs).replace(Some(peripherals.GPIOC)));
  free(|cs| G_EXTI.borrow(cs).replace(Some(peripherals.EXTI)));

  loop
  {
  }
}

#[interrupt]
fn EXTI13()
{
  // Clear interrupt
  free(|cs| {
    if let Some(ref mut exti) = G_EXTI.borrow(cs).borrow_mut().deref_mut() {
      exti.fpr1.write(|w| w.fpif13().set_bit()); // Bit is cleared by writing 1
    }
  });

  // Toggle LED
  free(|cs| {
    if let Some(ref mut gpioc) = G_GPIOC.borrow(cs).borrow_mut().deref_mut() {
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