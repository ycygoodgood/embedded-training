use core::num::{NonZeroU8, NonZeroU16};

use embassy_stm32::{
    Config, Peri, Peripherals, bind_interrupts,
    can::{
        Can, CanConfigurator, IT0InterruptHandler, IT1InterruptHandler,
        config::{DataBitTiming, FrameTransmissionConfig, NominalBitTiming},
    },
    pac, peripherals,
    rcc::{
        Hse, HseMode, Hsi48Config, Pll, PllDiv, PllMul, PllPreDiv, PllSource,
        mux::{Fdcansel, Usbsel},
    },
    time::Hertz,
    usb::{self, Driver},
};
use static_cell::StaticCell;

pub type Usb = Driver<'static, peripherals::USB_OTG_HS>;

bind_interrupts!(struct UsbIrqs {
    OTG_HS => usb::InterruptHandler<peripherals::USB_OTG_HS>;
});

bind_interrupts!(struct CanIrqs {
    FDCAN1_IT0 => IT0InterruptHandler<peripherals::FDCAN1>;
    FDCAN1_IT1 => IT1InterruptHandler<peripherals::FDCAN1>;
});

/// The clock tree is provided. Keep SYSCLK at its default HSI setting.
pub fn init() -> Peripherals {
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(24_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll2 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV12,
        mul: PllMul::MUL240,
        divp: Some(PllDiv::DIV2),
        divq: Some(PllDiv::DIV6),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.mux.fdcansel = Fdcansel::PLL2_Q;
    config.rcc.hsi48 = Some(Hsi48Config {
        sync_from_usb: true,
    });
    config.rcc.mux.usbsel = Usbsel::HSI48;
    embassy_stm32::init(config)
}

pub fn usb(
    otg: embassy_stm32::Peri<'static, peripherals::USB_OTG_HS>,
    dp: embassy_stm32::Peri<'static, peripherals::PA12>,
    dm: embassy_stm32::Peri<'static, peripherals::PA11>,
) -> Usb {
    static OUT: StaticCell<[u8; 512]> = StaticCell::new();
    let mut config = usb::Config::default();
    config.vbus_detection = false;
    Driver::new_fs(otg, UsbIrqs, dp, dm, OUT.init([0; 512]), config)
}

/// CAN FD setup: 1 Mbit/s arbitration phase, 5 Mbit/s data phase with BRS, TDC on.
pub fn can1(
    fdcan: Peri<'static, peripherals::FDCAN1>,
    rx: Peri<'static, peripherals::PD0>,
    tx: Peri<'static, peripherals::PD1>,
) -> Can<'static> {
    let mut can = CanConfigurator::new(fdcan, rx, tx, CanIrqs);

    // Arbitration phase: 80 MHz / 80 tq = 1 Mbit/s, sample point (1+59)/80 = 75%.
    let nominal = NominalBitTiming {
        prescaler: NonZeroU16::new(1).unwrap(),
        seg1: NonZeroU8::new(59).unwrap(),
        seg2: NonZeroU8::new(20).unwrap(),
        sync_jump_width: NonZeroU8::new(20).unwrap(),
    };

    // Data phase: 80 MHz / 16 tq = 5 Mbit/s, sample point (1+11)/16 = 75%.
    let data = DataBitTiming {
        transceiver_delay_compensation: true,
        prescaler: NonZeroU16::new(1).unwrap(),
        seg1: NonZeroU8::new(11).unwrap(),
        seg2: NonZeroU8::new(4).unwrap(),
        sync_jump_width: NonZeroU8::new(3).unwrap(),
    };

    let config = can
        .config()
        .set_nominal_bit_timing(nominal)
        .set_data_bit_timing(data)
        .set_frame_transmit(FrameTransmissionConfig::AllowFdCanAndBRS);
    can.set_config(config);

    let can = can.into_normal_mode();
    enable_tdc();
    can
}

/// TDC offset and filter window, in data phase time quanta (1 tq = 12.5 ns at 80 MHz).
const TDC_OFFSET: u8 = 2;
const TDC_FILTER: u8 = 12;

/// embassy-stm32 0.6 never writes the TDC fields: `DataBitTiming` carries a
/// `transceiver_delay_compensation` flag but the register write is still a TODO in the
/// crate, and `set_data_bit_timing` overwrites DBTP without the TDC bit. So DBTP.TDC and
/// TDCR are programmed here, after the driver has applied its configuration. Both are
/// write protected, which is why the peripheral is briefly put back into config mode.
fn enable_tdc() {
    let cccr = pac::FDCAN1.cccr();
    cccr.modify(|w| w.set_init(true));
    while !cccr.read().init() {}
    cccr.modify(|w| w.set_cce(true));

    pac::FDCAN1.dbtp().modify(|w| w.set_tdc(true));
    pac::FDCAN1.tdcr().modify(|w| {
        w.set_tdco(TDC_OFFSET);
        w.set_tdcf(TDC_FILTER);
    });

    cccr.modify(|w| w.set_cce(false));
    cccr.modify(|w| w.set_init(false));
    while cccr.read().init() {}
}
