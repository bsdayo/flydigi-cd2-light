use clap::{Parser, Subcommand};
use rusb::DeviceHandle;

mod error;
mod usb;

pub use error::Error;

const TARGET_VID: u16 = 0x37D7;
const TARGET_PID: u16 = 0x6001;
pub const LED_COUNT: usize = 162;

#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<csscolorparser::Color> for Color {
    fn from(value: csscolorparser::Color) -> Self {
        Self {
            r: value.r as u8,
            g: value.g as u8,
            b: value.b as u8,
        }
    }
}

#[derive(Parser)]
#[command(name = "flydigi-cd2-light")]
#[command(about = "Control Flydigi Controller Charging Station 2 Pro LED")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fill the entire LED screen with a color
    Fill {
        /// Color string (e.g. "red", "#ff0000", "rgb(255,0,0)")
        color: String,
    },
}

fn find_device() -> Option<DeviceHandle<rusb::GlobalContext>> {
    let devices = rusb::devices().ok()?;
    for device in devices.iter() {
        let desc = device.device_descriptor().ok()?;
        if desc.vendor_id() == TARGET_VID && desc.product_id() == TARGET_PID {
            let handle = device.open().ok()?;
            return Some(handle);
        }
    }
    None
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fill { color } => {
            let c = match csscolorparser::parse(&color) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to parse color '{}': {}", color, e);
                    std::process::exit(1);
                }
            };

            let handle = match find_device() {
                Some(h) => h,
                None => {
                    eprintln!("Device not found");
                    std::process::exit(1);
                }
            };

            if let Err(e) = handle.claim_interface(0) {
                eprintln!("Failed to claim interface: {}", e);
                std::process::exit(1);
            }

            let colors = vec![c.into(); LED_COUNT];
            let frames = vec![colors];
            if let Err(e) = usb::send_led_data(&handle, &frames) {
                eprintln!("Failed to send LED data: {}", e);
                std::process::exit(1);
            }
        }
    }
}
