//! bxCAN to USART2 Example for STM32F407 using Rust
//! Reads all CAN messages and sends them via USART2 with decoded details

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::interrupt::{self, Mutex};
use cortex_m::peripheral::NVIC;
use stm32f4::stm32f407::{self, CAN1, USART2, Interrupt};
use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};
use panic_halt as _;

static CAN: Mutex<RefCell<Option<CAN1>>> = Mutex::new(RefCell::new(None));
static USART: Mutex<RefCell<Option<USART2>>> = Mutex::new(RefCell::new(None));
static TX_BUFFER: Mutex<RefCell<[u8; 64]>> = Mutex::new(RefCell::new([0; 64]));
static TX_INDEX: AtomicUsize = AtomicUsize::new(0);
static TX_LENGTH: AtomicUsize = AtomicUsize::new(0);

#[entry]
fn main() -> ! {
    let dp = stm32f407::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let rcc = &dp.RCC;
    rcc.apb1enr.modify(|_, w| w.can1en().set_bit().usart2en().set_bit());

    // Configure USART2 (115200 baud, 8N1)
    let usart2 = &dp.USART2;
    usart2.brr.modify(|_, w| {
      w.div_fraction().variant(0)
        .div_mantissa().variant(45)
    }); // Assuming PCLK1 at 42MHz, 42_000_000 / 115200 ≈ 364, divide by 8 gives 45
    usart2.cr1.modify(|_, w| w.ue().set_bit()); // enable usart2
    usart2.cr1.modify(|_, w| w.te().set_bit()); // transmit enable
    usart2.cr1.modify(|_, w| w.txeie().set_bit()); // TX interrupt

    // Configure CAN1
    let can1 = &dp.CAN1;
    can1.mcr.modify(|_, w| w.inrq().set_bit()); // Enter initialization mode
    while can1.msr.read().inak().bit_is_clear() {}
    can1.btr.modify(|_, w| w.silm().clear_bit()); // normal operation
    can1.btr.modify(|_, w| w.lbkm().clear_bit()); // loop back mode disabled
    can1.btr.modify(|_, w| w.brp().variant(3)); // t_q = BRP+1 * t_pclk
    can1.btr.modify(|_, w| w.ts1().bits(6)); // t_bs1 = t_q * TS1+1
    can1.btr.modify(|_, w| w.ts2().bits(1)); // t_bs2 = t_q * TS2+1
    can1.mcr.modify(|_, w| w.inrq().clear_bit()); // Exit initialization mode
    while can1.msr.read().inak().bit_is_set() {}
    can1.ier.modify(|_, w| w.fmpie0().set_bit()); // Enable FIFO 0 message pending interrupt

    interrupt::free(|cs| {
        CAN.borrow(cs).replace(Some(dp.CAN1));
        USART.borrow(cs).replace(Some(dp.USART2));
    });

    unsafe {
        NVIC::unmask(Interrupt::CAN1_RX0);
        NVIC::unmask(Interrupt::USART2);
    }

    loop {}
}

#[cortex_m_rt::interrupt]
fn stm32f4::stm32f407::Interrupt::CAN1_RX0() {
    interrupt::free(|cs| {
        if let Some(ref mut can1) = CAN.borrow(cs).borrow_mut().as_mut() {
            if can1.rfr[0].read().fmp().bits() > 0 {
                let rir = can1.rfr[0].read().bits();
                let id = if rir & (1 << 2) != 0 {
                    rir >> 3 // Extended ID
                } else {
                    (rir >> 21) & 0x7FF // Standard ID
                };
                let dlc = (can1.rdtr0.read().dlc().bits() & 0xF) as usize;
                let data = [
                    can1.rfr[0].read().bits() as u8,
                    (can1.rfr[0].read().bits() >> 8) as u8,
                    (can1.rfr[0].read().bits() >> 16) as u8,
                    (can1.rfr[0].read().bits() >> 24) as u8,
                    can1.rfr[0].read().bits() as u8,
                    (can1.rfr[0].read().bits() >> 8) as u8,
                    (can1.rfr[0].read().bits() >> 16) as u8,
                    (can1.rfr[0].read().bits() >> 24) as u8,
                ];
                let mut buffer = TX_BUFFER.borrow(cs).borrow_mut();
                let len = core::fmt::Write::write_fmt(&mut *buffer, format_args!(
                    "ID: {:X} DLC: {} DATA: {:?}\r\n", id, dlc, &data[..dlc]
                )).unwrap_or(0);
                TX_LENGTH.store(len, Ordering::Release);
                TX_INDEX.store(0, Ordering::Release);
                can1.rfr[0].modify(|_, w| w.rfom0().set_bit()); // Release FIFO 0
                USART.borrow(cs).borrow_mut().as_mut().unwrap().cr1.modify(|_, w| w.txeie().set_bit());
            }
        }
    });
}

#[cortex_m_rt::interrupt]
fn stm32f4::stm32f407::Interrupt::USART2() {
    interrupt::free(|cs| {
        let index = TX_INDEX.load(Ordering::Acquire);
        let length = TX_LENGTH.load(Ordering::Acquire);
        if index < length {
            let usart2 = USART.borrow(cs).borrow_mut().as_mut().unwrap();
            let buffer = TX_BUFFER.borrow(cs).borrow();
            usart2.dr.write(|w| w.dr().bits(buffer[index] as u16));
            TX_INDEX.store(index + 1, Ordering::Release);
        } else {
            USART.borrow(cs).borrow_mut().as_mut().unwrap().cr1.modify(|_, w| w.txeie().clear_bit());
        }
    });
}
