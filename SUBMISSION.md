# HerKules L4 · DM-MC02 USB 调试器作业 · 提交材料

| 项目 | 内容 |
| --- | --- |
| 姓名 / 学号 | （填写） |
| 仓库地址 | https://github.com/ycygoodgood/embedded-training |
| 提交 commit | （推送后填写最新 commit hash） |
| 板卡 | DM-MC02（STM32H723VGT6） |
| 提交日期 | （填写） |

---

## 1. 引脚表

> 四个信号全部取自板上 **P16「扩展 IO」24 脚排针**（原理图第 6 页，连接器型号 `SGDBF-08-024P-H35`），
> 而且是相邻的 4 个奇数脚（11/13/15/17），接线时一根排线上取四根线即可。
> 依据：达妙 [CtrBoard-H7_V1.0-240124.pdf](https://github.com/dmBots/DM-MC02/blob/master/drawings/2d/schematics/CtrBoard-H7_V1.0-240124.pdf) 第 6 页。

### 1.1 SWD（本板做探针，向目标板输出 SWD）

| 信号 | MCU 引脚 | 引脚配置 | P16 脚号 | 接到哪里 | 方向 |
| --- | --- | --- | --- | --- | --- |
| SWCLK | `PD14` | GPIO 推挽输出，初始 `Level::Low`，`Speed::Low` | **11** | 目标板 SWCLK（自测时接 P1.3） | 本板 → 目标 |
| SWDIO | `PE14` | GPIO 双向 `Flex`，空闲输入 + 上拉 | **13** | 目标板 SWDIO（自测时接 P1.4） | 双向 |
| GND | — | — | 1 / 3 / 5 / 7 | 目标板 GND（同板自测可不接） | — |

### 1.2 UART

| 信号 | MCU 引脚 | 复用功能 | P16 脚号 | 接到哪里 | 方向 |
| --- | --- | --- | --- | --- | --- |
| UART8_RX | `PE0` | AF8 | **15** | 目标 TX；回环自测时与 TX 短接 | 目标 → 本板 |
| UART8_TX | `PE1` | AF8 | **17** | 目标 RX；回环自测时与 RX 短接 | 本板 → 目标 |
| GND | — | — | 1 / 3 / 5 / 7 | 目标 GND | — |

> P16 的每根信号线都串了 **22 Ω** 电阻（R96/R97/R99/R101/R103/R105/R102/R104），
> 属于 ESD 保护，对 115200 UART 和本作业的慢速 SWD（约百 kHz）没有影响。

### 1.3 串口参数与冲突核对

| 项目 | 值 | 说明 |
| --- | --- | --- |
| 波特率 / 帧格式 | 115200 / 8N1 | `Config::default()` 本就是 8N1，代码里仍显式写出 `DataBits8` / `ParityNone` / `STOP1` |
| 中断绑定 | `UART8 => BufferedInterruptHandler<peripherals::UART8>` | 注意是「缓冲」串口的中断处理类型 |
| 收发缓冲区 | TX 256 B / RX 256 B（`StaticCell`） | 满足 `'static` 要求 |
| 与 USB 冲突 | 无 | USB 用 PA11/PA12 |
| 与 CAN1 冲突 | 无 | CAN1 用 PD0/PD1 |
| 与芯片自身调试口冲突 | 无 | 未使用 PA13/PA14（P1 上的板载 SWD 口） |
| 四个信号是否互相冲突 | 无 | PD14 / PE14 / PE0 / PE1 互不相同，且都不是外设占用脚 |

### 1.4 选型理由

> 四个信号都放在 **P16** 上，原因有三：①它是板上专门留的用户扩展口，24 脚排针有完整编号，接线不易接错；
> ②UART8 在本芯片上只有 `PE0(RX)`/`PE1(TX)` 这一组可用引脚，硬件上没有别的选择；③把 SWD 的两根线也放到
> 同一个排针，四根线在同一侧相邻的奇数脚（11/13/15/17）上，自测时从 P16 一次取线即可，避免跨接线。
> SWDIO 必须能在「输出请求」和「读 ACK」之间切换方向，所以用 `Flex` 而不是 `Output`；SWCLK 只需输出，
> 用 `Output` 并让初始电平为低。
>
> 备选方案：P11「4 路舵机接口」也引出了 PWM 脚（1 脚 PA0、2 脚 PA2、3 脚 PE9、4 脚 PE13，按原理图符号编号），
> 可以作为 SWD 的另一组选择，但它的原理图符号把 4 路信号 + 5V + GND 合并成 6 个引脚，与 4×3 排针的实物编号
> 不能一一对应，容易接错，因此没有采用。

### 1.5 P16 完整脚位（来自原理图第 6 页，供核对）

| 脚 | 网络 | 脚 | 网络 |
| --- | --- | --- | --- |
| 1 | GND | 2 | VCC_IN |
| 3 | GND | 4 | VCC_5V |
| 5 | GND | 6 | VCC_5V |
| 7 | GND | 8 | VCC_3V3 |
| **9** | UART9_TX (PD15) | 10 | CAN3_H |
| **11** | UART9_RX (PD14) ← **SWCLK** | 12 | CAN3_L |
| **13** | PE14 ← **SWDIO** | 14 | 485_B1 |
| **15** | UART8_RX (PE0) ← **UART RX** | 16 | 485_A1 |
| **17** | UART8_TX (PE1) ← **UART TX** | 18 | 485_B2 |
| 19 | SPI3_MOSI (PC12) | 20 | 485_A2 |
| 21 | SPI3_MISO (PC11) | 22 | DCMI_SCL (PB08) |
| 23 | SPI3_CLK (PC10) | 24 | DCMI_SDA (PB09) |

---

## 2. 测试记录

环境：Windows 11 / PowerShell 7，Rust 工具链按 `rust-toolchain.toml` 固定版本，probe-rs（版本填写），pyserial（版本填写）。

### 2.1 编译与烧录

| # | 命令 | 实际输出（摘要） | 判定 |
| --- | --- | --- | --- |
| 1 | `cargo build --locked` | （实测填写，例如 `Finished dev profile ...`） | ☐ 通过 |
| 2 | `probe-rs list` | （实测填写，应含助教给的探针型号；烧过固件后含 `DM-MC02 CMSIS-DAP Demo`） | ☐ 通过 |
| 3 | `cargo run --locked` | （实测填写：烧录完成后终端停住，按 Ctrl+C 退出） | ☐ 通过 |

### 2.2 USB 与控制口

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 4 | `python -m serial.tools.list_ports` | （实测填写两个 COM 口） | ☐ 通过 |
| 5 | `python host.py <控制口> ping` | `OK DM-MC02` | ☐ 通过 |

### 2.3 SWD 读 DPIDR

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 6 | `python host.py <控制口> swd` | `OK DPIDR （实测填写）` | ☐ 通过 |
| 7 | `python host.py <控制口> swd`（重复一次） | `OK DPIDR （同上，要求逐字符一致）` | ☐ 通过 |
| 8 | `python host.py <控制口> ping` | `OK DM-MC02` | ☐ 通过 |

### 2.4 UART 回环

接线：断电后短接 UART8 的 TX 与 RX。

| # | 命令 | 实际输出 | 判定 |
| --- | --- | --- | --- |
| 9 | `python host.py <UART口> uart-loopback` | `OK UART loopback` | ☐ 通过 |

### 2.5 未测项目（如实填写）

| 项目 | 原因 |
| --- | --- |
| 双板 SWD 测试（两块 DM-MC02 分别供电、接 GND） | （例如：只有一块板 / 缺少杜邦线） |
| Classical CAN 与第二节点收发 | （例如：没有 CAN 对端节点） |
| CAN FD / Linux SocketCAN 选做 | （例如：环境不满足） |

---

## 3. 一条实测证据（必写，二选一）

**选项 A：用一条结果证明配置正确**

> 例：`SWD ID` 连续两次都返回 `OK DPIDR 0x...`，且数值完全一致。这说明：
> SWCLK 真的在输出时钟、SWDIO 的方向切换正确（否则 ACK 读不到，会返回 `ERR SWD`）、
> 并且控制串口 → 固件 → GPIO → 目标调试端口这条链路全程连通。
> 单次读数可能碰巧成功，连续两次一致才能排除偶发接线问题。

**选项 B：记录一次故障定位过程**

> 例：第一次上电 `SWD ID` 一直返回 `ERR SWD`。排查顺序：
> ① 用万用表确认 SWCLK 有翻转（排除引脚选错）→ 有翻转，说明 GPIO 配置生效；
> ② 检查发现 SWDIO 写成了 `Output<'static>` 而不是 `Flex` → 固件里 `read_bits` 读到恒为 1 的 ACK；
> ③ 改成 `Flex` 并保证 `Swd::new` 里执行了 `set_as_input(Pull::Up)` 后，`SWD ID` 返回正常 DPIDR。
> 结论：SWDIO 的双向能力是这条命令成功的前提。

（把你的实际内容写在这里）

---

## 4. 提交清单

- [ ] `src/wiring.rs`（两个 TODO 已补全）
- [ ] 本文件的引脚表（接插件脚号已按原理图填写）
- [ ] 本文件的测试记录（命令 + 实际输出 + 未测项目）
- [ ] 第 3 节的一条实测证据
- [ ] 仓库已 push，且没有把 `.venv/`、`target/` 提交上去
