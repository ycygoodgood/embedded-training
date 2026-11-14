# 测试记录

杨辰煜　3036480359

| 项目 | 内容 |
| --- | --- |
| 仓库地址 | https://github.com/ycygoodgood/embedded-training |

环境：Windows 11 / PowerShell 7，Rust 工具链按 `rust-toolchain.toml` 固定，probe-rs 0.32.0，pyserial。

## 1. 编译

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 1 | `cargo build --locked` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 23.36s` | 通过 |
| 2 | `cargo build --locked --offline` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 0.29s` | 通过 |

## 2. 烧录

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 3 | `probe-rs list` | — | 通过 |
| 4 | `cargo run --locked` | — | 通过 |

## 3. USB 与控制口

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 5 | `python -m serial.tools.list_ports` | `COM3` / `COM4` | 通过 |
| 6 | `python host.py COM3 ping` | `OK DM-MC02` | 通过 |

## 4. SWD 读 DPIDR

接线：P16-11 → P1.3（SWCLK），P16-13 → P1.4（SWDIO）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 7 | `python host.py COM3 swd` | `OK DPIDR 6B......` | 通过 |
| 8 | `python host.py COM3 swd` | `OK DPIDR 6B......` | 通过 |
| 9 | `python host.py COM3 ping` | `OK DM-MC02` | 通过 |

## 5. UART 回环

接线：断电后短接 P16-15（RX）与 P16-17（TX）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 10 | `python host.py COM4 uart-loopback` | `OK UART loopback` | 通过 |

探针型号与 VID:PID、DPIDR 后 6 位未保存。
