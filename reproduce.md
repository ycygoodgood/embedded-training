# 复现说明

杨辰煜　3036480359

## 引脚选择

四个信号都放在 P16 扩展口上。UART8 在 STM32H723 上只有 PE0（RX）和 PE1（TX）这一组引脚，没有别的选择；P16 是板上专门引出的 24 脚扩展口（原理图第 6 页），脚位有完整编号，接错的可能性最小。SWD 的两根线也放在同一排针，四个信号就集中在相邻的 11 / 13 / 15 / 17 脚，接线和同板自测时只在一处取线。这几个脚都没被板上其他电路占用：USB 用 PA11 / PA12，CAN1 用 PD0 / PD1，芯片自身的调试口用 PA13 / PA14（P1 排针）。

配置上，SWDIO 发请求时要输出、读 ACK 时要输入，所以用 `Flex`；SWCLK 只输出时钟，用 `Output` 并把初始电平设为低，避免上电瞬间给目标芯片多余的时钟边沿。

P11「4 路舵机接口」上也引出了 PWM 脚，但它的原理图符号把四路信号和 5V、GND 合并成 6 个 Pin，跟 4×3 排针的实物编号对不上，接错的风险更高，所以没用。

## 实测结果

同板自测，P16-11 接 P1.3（SWCLK），P16-13 接 P1.4（SWDIO）。

```text
> python host.py COM6 swd
OK DPIDR 6BA02477
> python host.py COM6 swd
OK DPIDR 6BA02477
```

两次读数一致，说明 SWCLK 确实在输出时钟、SWDIO 的输入输出切换正确（配成 `Output` 就读不回 ACK，会返回 `ERR SWD`），控制串口到目标调试端口这条链路是通的。单次读数可能碰巧成功，连读两次一致才排除偶发接线问题。读到的 0x6BA02477 也和 Cortex-M 系列 SW-DP 的常见值一致。

之后把 P16-15 和 P16-17 短接，UART 回环也通过了：

```text
> python host.py COM7 uart-loopback
OK UART loopback
```
