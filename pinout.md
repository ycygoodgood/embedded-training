# 引脚表

杨辰煜　3036480359

| 信号 | MCU 引脚 | 引脚配置 | P16 脚号 | 接线 |
| --- | --- | --- | --- | --- |
| SWCLK | PD14 | 推挽输出，初始电平低 | 11 | 目标 SWCLK；自测接 P1.3 |
| SWDIO | PE14 | 双向 Flex，空闲输入上拉 | 13 | 目标 SWDIO；自测接 P1.4 |
| UART8_RX | PE0 | AF8 | 15 | 目标 TX；自测与 17 脚短接 |
| UART8_TX | PE1 | AF8 | 17 | 目标 RX；自测与 15 脚短接 |
| GND | — | — | 1 / 3 / 5 / 7 | 目标 GND |

UART 参数：115200 baud，8 数据位，无校验，1 停止位。

## P16 扩展口脚位

| 脚 | 网络 | 脚 | 网络 |
| --- | --- | --- | --- |
| 1 | GND | 2 | VCC_IN |
| 3 | GND | 4 | VCC_5V |
| 5 | GND | 6 | VCC_5V |
| 7 | GND | 8 | VCC_3V3 |
| 9 | UART9_TX (PD15) | 10 | CAN3_H |
| **11** | UART9_RX (PD14)　**SWCLK** | 12 | CAN3_L |
| **13** | PE14　**SWDIO** | 14 | RS485_B1 |
| **15** | UART8_RX (PE0)　**UART_RX** | 16 | RS485_A1 |
| **17** | UART8_TX (PE1)　**UART_TX** | 18 | RS485_B2 |
| 19 | SPI3_MOSI (PC12) | 20 | RS485_A2 |
| 21 | SPI3_MISO (PC11) | 22 | DCMI_SCL (PB08) |
| 23 | SPI3_CLK (PC10) | 24 | DCMI_SDA (PB09) |
