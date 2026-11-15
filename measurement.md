# 测试记录

杨辰煜　3036480359

| 项目 | 内容 |
| --- | --- |
| 仓库地址 | https://github.com/ycygoodgood/embedded-training |

## 1. 编译

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 1 | `cargo build --locked` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 23.36s` | 通过 |
| 2 | `cargo build --locked --offline` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 0.29s` | 通过 |

## 2. USB 与控制口

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 3 | `python -m serial.tools.list_ports` | `COM6` / `COM7` | 通过 |
| 4 | `python host.py COM6 ping` | `OK DM-MC02` | 通过 |

## 3. SWD 读 DPIDR

接线：P16-11 → P1.3（SWCLK），P16-13 → P1.4（SWDIO）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 5 | `python host.py COM6 swd` | `OK DPIDR 6BA02477` | 通过 |
| 6 | `python host.py COM6 swd` | `OK DPIDR 6BA02477` | 通过 |
| 7 | `python host.py COM6 ping` | `OK DM-MC02` | 通过 |

## 4. UART 回环

接线：断电后短接 P16-15（RX）与 P16-17（TX）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 8 | `python host.py COM7 uart-loopback` | `OK UART loopback` | 通过 |

