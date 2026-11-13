# 测试记录

杨辰煜　3036480359

| 项目 | 内容 |
| --- | --- |
| 仓库地址 | https://github.com/ycygoodgood/embedded-training |

环境：Windows 11 / PowerShell 7，Rust 工具链按 `rust-toolchain.toml` 固定，probe-rs（版本待填），pyserial（版本待填）。

## 1. 编译与烧录

| # | 命令 | 实际输出（摘要） | 判定 |
| --- | --- | --- | --- |
| 1 | `cargo build --locked` | （填写，例如 `Finished dev profile ...`） | ☐ 通过 |
| 2 | `probe-rs list` | （填写，应含探针型号；烧过固件后含 `DM-MC02 CMSIS-DAP Demo`） | ☐ 通过 |
| 3 | `cargo run --locked` | （填写：烧录完成后终端停住，按 Ctrl+C 退出） | ☐ 通过 |

## 2. USB 与控制口

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 4 | `python -m serial.tools.list_ports` | （填写两个 COM 口） | ☐ 通过 |
| 5 | `python host.py <控制口> ping` | `OK DM-MC02` | ☐ 通过 |

## 3. SWD 读 DPIDR

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 6 | `python host.py <控制口> swd` | `OK DPIDR （填写）` | ☐ 通过 |
| 7 | `python host.py <控制口> swd`（重复一次） | `OK DPIDR （应与上一次逐字符一致）` | ☐ 通过 |
| 8 | `python host.py <控制口> ping` | `OK DM-MC02` | ☐ 通过 |

## 4. UART 回环

接线：断电后短接 P16-15（RX）与 P16-17（TX）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 9 | `python host.py <UART口> uart-loopback` | `OK UART loopback` | ☐ 通过 |

## 5. 未测项目

| 项目 | 原因 |
| --- | --- |
| 双板 SWD 测试（两块 DM-MC02 分别供电、接 GND） | （填写，例如只有一块板 / 缺少杜邦线） |
| Classical CAN 与第二节点收发 | （填写，例如没有 CAN 对端节点） |
| CAN FD / Linux SocketCAN 选做 | （填写，例如环境不满足） |
