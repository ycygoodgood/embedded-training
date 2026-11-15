//! DM-MC02 USB debug probe: SWD and UART pin configuration.

use embassy_stm32::{
    Peripherals, bind_interrupts,
    can::Can,
    gpio::{Flex, Level, Output, Speed},
    peripherals,
    usart::{
        BufferedInterruptHandler, BufferedUart, Config as UartConfig, DataBits, Parity, StopBits,
    },
};
use static_cell::StaticCell;

use crate::swd::Swd;

bind_interrupts!(struct Irqs {
    UART8 => BufferedInterruptHandler<peripherals::UART8>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: SWCLK on PD14 (P16-11), SWDIO on PE14 (P16-13).
    // SWCLK only drives the clock, so a plain output starting low is enough.
    let clock = Output::new(p.PD14, Level::Low, Speed::Low);
    // SWDIO is driven out for the request and read back for the ACK, so it has
    // to switch direction. Flex leaves it as an input with a pull-up at idle.
    let data = Flex::new(p.PE14);
    let swd = Some(Swd::new(clock, data));

    // TODO 2: UART8 on PE0 (RX, P16-15) and PE1 (TX, P16-17), 115200 8N1.
    static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    let mut config = UartConfig::default();
    config.baudrate = 115_200;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;

    let uart = BufferedUart::new(
        p.UART8,
        p.PE0,
        p.PE1,
        TX_BUF.init([0; 256]),
        RX_BUF.init([0; 256]),
        Irqs,
        config,
    )
    .expect("115200 8N1");

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart: Some(uart) }, swd)
}
