# 测试记录

杨辰煜　3036480359

| 项目 | 内容 |
| --- | --- |
| 仓库地址 | https://github.com/ycygoodgood/embedded-training |

环境：Windows 11 / PowerShell 7，Rust 工具链按 `rust-toolchain.toml` 固定，probe-rs 0.32.0，pyserial（待实验室安装）。

## 1. 编译（已完成）

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 1 | `cargo build --locked` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 23.36s` | 通过 |
| 2 | `cargo build --locked --offline` | `Finished \`dev\` profile [optimized + debuginfo] target(s) in 0.29s` | 通过 |

第 2 条证明依赖已全部缓存，在无网络的实验室环境也能直接编译。

## 2. 烧录（未测）

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 3 | `probe-rs list` | 未测 | — |
| 4 | `cargo run --locked` | 未测 | — |

## 3. USB 与控制口（未测）

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 5 | `python -m serial.tools.list_ports` | 未测 | — |
| 6 | `python host.py <控制口> ping` | 未测 | — |

## 4. SWD 读 DPIDR（未测）

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 7 | `python host.py <控制口> swd` | 未测 | — |
| 8 | `python host.py <控制口> swd`（重复一次） | 未测 | — |
| 9 | `python host.py <控制口> ping` | 未测 | — |

## 5. UART 回环（未测）

接线：断电后短接 P16-15（RX）与 P16-17（TX）。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 10 | `python host.py <UART口> uart-loopback` | 未测 | — |

## 6. 未测原因

手上只有笔记本电脑，没有 DM-MC02 板卡、外置调试探针（DAPLink）和杜邦线，无法完成首次烧录，因此第 2~5 节全部无法进行。板卡与探针需在实验室使用。

## 7. 补测计划

拿到板卡和探针后按以下顺序执行，并把输出回填上表：

1. 探针 SWDIO / SWCLK / GND 接板子 P1（P1.3 = SWCLK，P1.4 = SWDIO），`probe-rs list` 确认识别
2. `cargo run --locked` 烧录，完成后 Ctrl+C 退出并拔掉探针
3. USB-C 接电脑，`python -m serial.tools.list_ports` 看到两个串口，用 `PING` 辨认控制口
4. 断电接 P16-11 → P1.3、P16-13 → P1.4，`SWD ID` 连读两次并比对
5. 断电短接 P16-15 / P16-17，跑 `uart-loopback`
6. 未做的选做项（双板 SWD、CAN、CAN FD）在表中如实标注
