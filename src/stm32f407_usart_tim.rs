#![no_main]
#![no_std]

use stm32f4::stm32f407::{self, interrupt, Interrupt, NVIC};
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::interrupt::{free, Mutex};
use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

static G_TIM3: Mutex<RefCell<Option<stm32f407::TIM3>>> = Mutex::new(RefCell::new(None));
static G_USART2: Mutex<RefCell<Option<stm32f407::USART2>>> = Mutex::new(RefCell::new(None));

// USART TX DATA
static TX_BUFFER: &[u8] = b"Hello world\r\n";
static TX_INDEX: AtomicUsize = AtomicUsize::new(0);

#[entry]
fn main() -> ! {
  let peripherals = stm32f407::Peripherals::take().unwrap();
  let rcc = &peripherals.RCC;
  let timer3 = peripherals.TIM3;
  let usart2 = peripherals.USART2;

  // Setup PLL
  rcc.cr.modify(|_, w| w.hseon().set_bit());
  while rcc.cr.read().hserdy().bit_is_clear() {}

  // Configure PLL: HSE as source, multiply by 336, divide by 4 for 84MHz SYSCLK
  rcc.pllcfgr.modify(|_, w| {
      w.pllsrc().set_bit() // HSE as PLL source
        .pllm().variant(8) // VCO input = HSE / 8 (1MHz)
        .plln().variant(336) // VCO output = 1MHz * 336 = 336MHz
        .pllp().div4() // PLLP = 4, SYSCLK = 336MHz / 4 = 84MHz
  });

  // Enable PLL
  rcc.cr.modify(|_, w| w.pllon().set_bit());
  while rcc.cr.read().pllrdy().bit_is_clear() {};

  // Select PLL as system clock
  rcc.cfgr.modify(|_, w| w.sw().pll());
  while !rcc.cfgr.read().sws().is_pll() {};

  // Enable peripherals clocks
  rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());
  rcc.apb1enr.modify(|_, w| w.usart2en().set_bit());

  // Setup USART2
  usart2.brr.modify(|_, w| {
    w.div_fraction().variant(0)
      .div_mantissa().variant(45)
  }); // Assuming PCLK1 at 42MHz, 42_000_000 / 115200 ≈ 364, divide by 8 gives 45
  usart2.cr1.modify(|_, w| w.ue().set_bit().te().set_bit()); // Enable USART and transmitter
  free(|cs|
    {
      G_USART2.borrow(cs).replace(Some(usart2));
    });
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
    NVIC::unmask(Interrupt::USART2);
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
    if let Some(ref mut tim3) = G_TIM3.borrow(cs).borrow_mut().as_mut()
    {
      if tim3.sr.read().uif().bit_is_set()
      {
        tim3.sr.modify(|_, w| w.uif().clear_bit()); // Clear update interrupt flag
        TX_INDEX.store(1, Ordering::Release); // Reset buffer index at 1 as we send the 0 idx here
        if let Some(ref mut usart2) = G_USART2.borrow(cs).borrow_mut().as_mut()
        {
          usart2.cr1.modify(|_, w| w.txeie().set_bit()); // Enable TXE interrupt
          usart2.dr.write(|w| w.dr().bits(TX_BUFFER[0] as u16)); // Start transmission
        }
      }
    }
  });
}

#[interrupt]
fn USART2() {
    free(|cs|
      {
        if let Some(ref mut usart2) = G_USART2.borrow(cs).borrow_mut().as_mut()
        {
          let index = TX_INDEX.load(Ordering::Acquire);
          if index < TX_BUFFER.len()
          {
            usart2.dr.write(|w| w.dr().bits(TX_BUFFER[index] as u16));
            TX_INDEX.store(index + 1, Ordering::Release);
          }
          else
          {
            usart2.cr1.modify(|_, w| w.txeie().clear_bit()); // Disable TX interrupt when done
          }
        }
    });
}
