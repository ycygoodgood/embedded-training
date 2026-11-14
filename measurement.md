# 测试记录

杨辰煜　3036480359

| 项目 | 内容 |
| --- | --- |
| 仓库地址 | https://github.com/ycygoodgood/embedded-training |

环境：Windows 11 / PowerShell 7，Rust 工具链按 `rust-toolchain.toml` 固定，probe-rs 0.32.0，pyserial 已安装。

## 1. 编译

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 1 | `cargo build --locked` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 23.36s` | 通过 |
| 2 | `cargo build --locked --offline` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 0.29s` | 通过 |

## 2. 烧录

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 3 | `probe-rs list` | 识别到外置 DAPLink 探针（型号与 VID:PID 未保存） | 通过 |
| 4 | `cargo run --locked` | 编译后由 probe-rs 完成擦写，随后终端停住无输出，Ctrl+C 退出 | 通过 |

## 3. USB 与控制口

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 5 | `python -m serial.tools.list_ports` | 出现两个新增串口：控制口 COM3、UART 口 COM4 | 通过 |
| 6 | `python host.py COM3 ping` | `OK DM-MC02` | 通过 |

## 4. SWD 读 DPIDR

接线：P16-11 → P1.3（SWCLK），P16-13 → P1.4（SWDIO）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 7 | `python host.py COM3 swd` | `OK DPIDR 6B……`（首字节 6B，后 6 位未保存） | 通过 |
| 8 | 同上，重复一次 | 与第 7 条一致（两次比对相同） | 通过 |
| 9 | `python host.py COM3 ping` | `OK DM-MC02` | 通过 |

## 5. UART 回环

接线：断电后短接 P16-15（RX）与 P16-17（TX）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 10 | `python host.py COM4 uart-loopback` | `OK UART loopback` | 通过 |

## 6. 未测项目

| 项目 | 原因 |
| --- | --- |
| 双板 SWD 测试（两块 DM-MC02 分别供电、接 GND） | 只使用一块板做同板自测 |
| Classical CAN 与第二节点收发 | 没有 CAN 对端节点 |
| CAN FD / Linux SocketCAN 选做 | 未开展 |

## 7. 说明

本节记录各项测试的通过与否。控制口为 COM3、UART 口为 COM4；DPIDR 只记下首字节 6B（固件按 `"{id:08X}"` 输出，不带 `0x` 前缀），后 6 位与探针型号、VID:PID 在执行时未保存。

若需补齐，可按下列步骤复测：

1. `probe-rs list` 记录探针型号与 VID:PID
2. `python -m serial.tools.list_ports` 确认两个 COM 口号，`PING` 辨认控制口
3. 按第 4 节接线后连读两次 `SWD ID`，记录完整 8 位 DPIDR 并比对
