# flydigi-cd2-light

> [!CAUTION]
> 本项目是基于逆向工程的个人学习研究成果，仅供教育和研究目的使用。本项目与飞智（Flydigi）官方无关，未获得官方授权或认可。
>
> 所有产品名称、商标和注册商标均为其各自所有者的财产。使用本软件风险自负。作者不对因使用本软件造成的任何损坏或问题承担责任。

用于控制飞智手柄充电底座 2 Pro LED 点阵屏的命令行工具。

## 功能

- 通过 USB 连接将整块 LED 屏幕填充为指定颜色
- 支持多种颜色字符串格式（CSS 颜色名、十六进制、RGB 等）
- 无需启动飞智空间站（服务），程序直接向设备发送 HID 数据

## 环境要求

- Rust 工具链（最新稳定版）
- 通过 USB 连接的飞智手柄充电底座 2 Pro

## 构建

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/flydigi-cd2-light`。

## 使用

```bash
# 将 LED 填充为红色
flydigi-cd2-light fill red

# 使用十六进制颜色
flydigi-cd2-light fill "#00ff00"

# 使用 RGB 值
flydigi-cd2-light fill "rgb(0, 128, 255)"
```

### 支持的颜色格式

任何 [csscolorparser](https://docs.rs/csscolorparser) 支持的有效 CSS 颜色字符串，包括：

- 颜色名称：`red`、`blue`、`hotpink` 等
- 十六进制：`#ff0000`、`#f00`、`#ff000080`
- RGB / RGBA：`rgb(255, 0, 0)`、`rgba(255, 0, 0, 0.5)`
- HSL / HSLA：`hsl(120, 100%, 50%)`
- 等等

## 许可证

MIT
