# flydigi-cd2-light

> [!CAUTION]
> This project is an unofficial reverse-engineering effort for educational and research purposes only. It is not affiliated with, endorsed by, or sponsored by Flydigi.
>
> All product names, trademarks, and registered trademarks are the property of their respective owners. Use this software at your own risk. The authors assume no liability for any damage or issues arising from its use.

A CLI tool to control the LED dot matrix screen on the Flydigi Controller Charging Station 2 Pro.

## Features

- Fill the entire LED screen with a solid color via USB
- Support for multiple color string formats (CSS color names, hex, RGB, etc.)

## Requirements

- Rust toolchain (latest stable)
- A Flydigi Controller Charging Station 2 Pro connected via USB

## Build

```bash
cargo build --release
```

The compiled binary will be available at `target/release/flydigi-cd2-light`.

## Usage

```bash
# Fill LEDs with red
flydigi-cd2-light fill red

# Fill LEDs with a hex color
flydigi-cd2-light fill "#00ff00"

# Fill LEDs with an RGB value
flydigi-cd2-light fill "rgb(0, 128, 255)"
```

### Supported Color Formats

Any valid CSS color string accepted by [csscolorparser](https://docs.rs/csscolorparser), including:

- Named colors: `red`, `blue`, `hotpink`, ...
- Hex: `#ff0000`, `#f00`, `#ff000080`
- RGB / RGBA: `rgb(255, 0, 0)`, `rgba(255, 0, 0, 0.5)`
- HSL / HSLA: `hsl(120, 100%, 50%)`
- And more

## License

MIT
