# 扩展任务：5 Mbit/s CAN FD 与 TDC

杨辰煜　3036480359

## 实现方式

在 `src/board.rs` 的 `can1()` 里把 FDCAN1 配成 CAN FD + BRS：仲裁段 1 Mbit/s、数据段 5 Mbit/s，并启用 TDC。在 `src/protocol.rs` 增加两条命令，给 FD 帧一个可实际调用的收发路径。基础作业的 SWD 和 UART 没有改动。

## 位时序

FDCAN 内核时钟 80 MHz（PLL2Q，见 `board.rs` 的 RCC 配置），所以 1 tq = 12.5 ns。

仲裁段 1 Mbit/s：

| 项 | 值 |
| --- | --- |
| 每比特 | 1000 ns = 80 tq |
| prescaler | 1 |
| TSEG1 / TSEG2 | 59 / 20 |
| SJW | 20 |
| 校验 | 1 + 59 + 20 = 80 tq → 80 MHz / 80 = 1 Mbit/s |
| 采样点 | (1 + 59) / 80 = 75% |

数据段 5 Mbit/s：

| 项 | 值 |
| --- | --- |
| 每比特 | 200 ns = 16 tq |
| prescaler | 1 |
| TSEG1 / TSEG2 | 11 / 4 |
| SJW | 3 |
| 校验 | 1 + 11 + 4 = 16 tq → 80 MHz / 16 = 5 Mbit/s |
| 采样点 | (1 + 11) / 16 = 75% |

SJW 取 3 而不是 4，是因为 crate 的注释要求 SJW 必须小于 DTSEG2；3 tq = 37.5 ns 对两端的晶振误差够用。

## TDC

之所以要开：数据段一个 bit 只有 200 ns，而收发器 TX→RX 的环路延迟 `t_LOOP` 在 100~200 ns 量级。发送端核对自己发出的位时，若按位时序把采样定在 75%（150 ns），可能早于自己这个位回到 RX 的时刻，读到的是前一位，判成位错误。TDC 让发送端用实测延迟来定位第二采样点 SSP。

参数（TDCR，单位是数据段 tq = 12.5 ns）：

| 参数 | 值 | 依据 |
| --- | --- | --- |
| TDCO | 2 tq = 25 ns | SSP = 实测延迟 + 偏移，留一点余量避开跳变沿 |
| TDCF | 12 tq = 150 ns | 延迟测量的上限，超过按 12 tq 计，保证 SSP 不超过 14 tq（175 ns），仍在 16 tq 的比特内 |

关系：`SSP = min(实测延迟, TDCF) + TDCO`。

代码上有一处绕不过去的地方：embassy-stm32 0.6 的 `DataBitTiming` 虽然有 `transceiver_delay_compensation` 字段，但 crate 里没有把它写进寄存器（`can/fd/config.rs` 中负责 TDC 的函数被注释掉，标着 TODO），而且 `set_data_bit_timing()` 是整字写 DBTP，会把 TDC 位清掉。所以 `board.rs` 里的 `enable_tdc()` 在驱动启动后直接写 `DBTP.TDC` 和 `TDCR`：这两个寄存器受写保护，函数先把 `CCCR.INIT`、`CCCR.CCE` 置 1，写完再清 0，流程和 crate 内部的 `enter_init_mode` / `leave_init_mode` 一致。

为了能访问寄存器，`Cargo.toml` 给 embassy-stm32 加了 `unstable-pac` feature。依赖版本没有变化，`Cargo.lock` 不需要更新（`cargo build --locked` 通过即可证明）。

## FD 帧处理

- 长度：FDCAN 的 DLC 可表示 0~8、12、16、20、24、32、48、64 字节，代码用 `FdData::is_valid_len` 校验，非法长度返回 `ERR FRAME`
- 帧类型：`Header::new_fd(id, len, rtr = false, brs = true)` 构造带 BRS 的 FD 帧，收发走 `write_fd` / `read_fd`
- 命令：
  - `CAN FD TX <id> <len>`：发一帧固定测试帧，载荷第 i 个字节 = i，标准帧 ID 由命令给出
  - `CAN FD RX`：读一帧，回 `OK FD RX <id> <len> <hex>`；单条回复限制 63 字节，所以只打印前 16 个字节，长度单独给出
- 缓冲：载荷放在 `FdData` 内部的 64 字节数组，FDCAN 的 message RAM 由 crate 配置

## 编译

命令：`cargo build --locked`
工具链：`rust-toolchain.toml` 固定的 1.99.1，目标 `thumbv7em-none-eabihf`

```
   Compiling embassy-stm32 v0.6.0
   Compiling dm-mc02-cmsis-dap v0.1.0 (C:\Users\15037\Documents\embedded-training)
    Finished `dev` profile [optimized + debuginfo] target(s) in 26.88s
```

无 warning。

## 上板时要匹配的对端

- 对端必须是 CAN FD 节点，收发器支持 5 Mbit/s 数据段（板载收发器按 MCP2542FD 一类设计）
- 对端仲裁段 1 Mbit/s、数据段 5 Mbit/s、采样点 75%；建议对端也开 TDC
- 总线两端 120 Ω 端接；5 Mbit/s 时线长控制在十几米以内

## 预期行为与验证状态

预期：`CAN FD TX 7FF 64` 返回 `OK QUEUED FD`；对端收到 ID 0x7FF、64 字节、内容 00..3F 的 FD 帧；`CAN FD RX` 能打出收到的 FD 帧 ID、长度和前 16 个字节。

**尚未上板验证。** 以上结论来自手册和 crate 源码推导，硬件行为需要接 CAN FD 对端后确认；TDCO、TDCF 也需要按实测的环路延迟再校准。
