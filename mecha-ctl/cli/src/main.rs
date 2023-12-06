//add clippy
#![warn(clippy::all)]
use anyhow::{bail, Result};
use std::{fs::File, io::BufReader};

use clap::{Parser, Subcommand};

mod battery;
use battery::Battery;

mod bluetooth;
use bluetooth::Bluetooth;

mod network;
pub use network::Network;

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
    // #[command(about = "Interact with network utility")]
    // Network(Network),
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
        }


    }
    Ok(())
}
