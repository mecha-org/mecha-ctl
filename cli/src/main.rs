//add clippy
#![warn(clippy::all)]
use anyhow::Result;

use clap::Parser;

mod battery;
use battery::Battery;

mod bluetooth;
use bluetooth::Bluetooth;

mod network;
pub use network::Network;

mod display;
pub use display::Display;

mod led;
pub use led::Led;

#[derive(Debug, Parser)]
#[command(name = "mecha")]
#[command(about = "A fictional Mecha CLI", long_about = None)]
struct MechaCli {
    #[command(subcommand)]
    command: Mecha,
}

#[derive(Debug, Parser)]
enum Mecha {
    #[command(about = "Device battery utility")]
    Battery(Battery),
    #[command(about = "Device bluetooth utility")]
    Bluetooth(Bluetooth),
    #[command(about = "Interact with network utility")]
    Network(Network),
    #[command(about = "Device Display utility")]
    Display(Display),
    #[command(about = "Device led utility")]
    Led(Led),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = MechaCli::parse();
    match cli.command {
        Mecha::Battery(battery) => match battery.execute().await {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        },

        Mecha::Bluetooth(bluetooth) => match bluetooth.execute().await {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        },

        Mecha::Network(network) => match network.execute().await {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        },

        Mecha::Display(display) => match display.execute().await {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        },

        Mecha::Led(led) => match led.execute().await {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        },
    }
    Ok(())
}
