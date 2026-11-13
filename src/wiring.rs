//! DM-MC02 USB 调试器 —— 作业里需要补全的两个 TODO（本文件已补全，可直接编译）。
//!
//! 引脚全部取自板上的 **P16「扩展 IO」24 脚排针**（型号 SGDBF-08-024P-H35），
//! 四根线在同一个排针的相邻脚上，接线最省事：
//!
//!   P16 引脚 | MCU 引脚 | 信号    | 配置
//!   ---------|----------|---------|------------------------------------
//!    11      | PD14     | SWCLK   | GPIO 推挽输出，初始低（板上串 22R）
//!    13      | PE14     | SWDIO   | GPIO 双向 Flex，空闲输入上拉（串 22R）
//!    15      | PE0      | UART8_RX| AF8，接目标 TX
//!    17      | PE1      | UART8_TX| AF8，发往目标 RX
//!
//! P16 其余引脚（供接线参考）：1/3/5/7 = GND，2 = VCC_IN，4/6 = VCC_5V，
//! 8 = VCC_3V3，9/11 = UART9(PD15/PD14)，10/12 = CAN3，14~20 = RS485，
//! 19/21/23 = SPI3(PC12/PC11/PC10)，22/24 = DCMI(PB08/PB09)。
//!
//! 换引脚只改下面标了「改这里」的四行，其它逻辑不用动。

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

// TODO 2 的第一步：把 UART8 的中断绑到「缓冲串口」的中断处理函数上。
// 注意类型是 BufferedInterruptHandler，不是普通 UART 的 InterruptHandler。
bind_interrupts!(struct Irqs {
    UART8 => BufferedInterruptHandler<peripherals::UART8>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // ── TODO 1：SWD 的两根线 ────────────────────────────────────────────────
    // SWCLK 只需要输出时钟：用 Output，初始电平 Low，速度 Low（慢一点更抗干扰）。
    // 初始为低是因为 Swd::new 会先把时钟拉低，避免上电瞬间给目标一个多余边沿。
    let clock = Output::new(p.PD14, Level::Low, Speed::Low); // 改这里：SWCLK → P16-11
    // SWDIO 既要发送请求（输出），又要读 ACK 和数据（输入），所以必须用 Flex。
    // 用 Output 会读不到 ACK，SWD ID 会一直返回 ERR SWD。
    let data = Flex::new(p.PE14); // 改这里：SWDIO → P16-13

    let swd = Some(Swd::new(clock, data));

    // ── TODO 2：UART8，115200 8N1 ───────────────────────────────────────────
    // 收发缓冲区要求生命周期是 'static，所以放进 static，再用 StaticCell 初始化。
    // 256 字节足够容纳一次 USB 包突发（PACKET = 64），也不会占太多 SRAM。
    static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    // 默认就是 8 数据位、无校验、1 停止位；这里只显式写出来，避免被默认值坑。
    let mut config = UartConfig::default();
    config.baudrate = 115_200;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;

    // 参数顺序最容易记错：外设, RX, TX, TX 缓冲, RX 缓冲, 中断绑定, config
    // （缓冲的先后和引脚的先后一致，别按名字猜。）
    let uart = BufferedUart::new(
        p.UART8,
        p.PE0, // 改这里：UART8_RX → P16-15（接目标板 TX）
        p.PE1, // 改这里：UART8_TX → P16-17（发往目标板 RX）
        TX_BUF.init([0; 256]),
        RX_BUF.init([0; 256]),
        Irqs,
        config,
    )
    .expect("115200 8N1 是合法配置，这里不应该失败");

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart: Some(uart) }, swd)
}
