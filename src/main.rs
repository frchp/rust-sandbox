#![no_main]
#![no_std]

use stm32f4::stm32f407::{self, interrupt, Interrupt, NVIC};
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::interrupt::{free, Mutex};
use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

static G_TIM3_INTERRUPT_OCCURRED: AtomicBool = AtomicBool::new(false);
static G_TIM3: Mutex<RefCell<Option<stm32f407::TIM3>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
  let peripherals = stm32f407::Peripherals::take().unwrap();
  let gpiod = peripherals.GPIOD;
  let rcc = &peripherals.RCC;
  let timer3 = peripherals.TIM3;

  // Enable peripherals clocks
  rcc.ahb1enr.modify(|_, w| w.gpioden().set_bit());
  rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());

  // Setup LED PD12
  gpiod.otyper.modify(|_, w| w.ot12().push_pull());
  gpiod.moder.modify(|_, w| w.moder12().output());
  // Setup TIMER3 with APB clock default (/1, 16MHz @ rst)
  timer3.psc.write(|w| w.psc().variant(1599)); // Prescaler: 16MHz / (1599 + 1) = 10kHz (0.1ms per tick)
  timer3.arr.modify(|_, w| w.arr().variant(4999)); // Auto-reload: 5000 ticks = 500ms interval
  timer3.dier.write(|w| w.uie().set_bit()); // enable update interrupt
  timer3.egr.write(|w| w.ug().set_bit()); // update generation
  timer3.cr1.modify(|_, w| w.cen().set_bit()); // enable timer
  free(|cs|
  {
    G_TIM3.borrow(cs).replace(Some(timer3));
  });
  // Setup NVIC interrupt
  unsafe
  {
    NVIC::unmask(Interrupt::TIM3);
  };

  loop
  {
    cortex_m::asm::wfi();
    if G_TIM3_INTERRUPT_OCCURRED.swap(false, Ordering::Acquire) {
      if gpiod.odr.read().odr12().bit_is_set()
      {
        gpiod.odr.write(|w| w.odr12().clear_bit());
      }
      else
      {
        gpiod.odr.write(|w| w.odr12().set_bit());
      }
    }
  }
}

#[interrupt]
fn TIM3()
{
  // Clear interrupt
  free(|cs|
  {
    if let Some(ref mut tim3) = G_TIM3.borrow(cs).borrow_mut().as_mut()
    {
      if tim3.sr.read().uif().bit_is_set()
      {
        tim3.sr.modify(|_, w| w.uif().clear_bit()); // Clear update interrupt flag
        G_TIM3_INTERRUPT_OCCURRED.store(true, Ordering::Release);
      }
    }
  });
}