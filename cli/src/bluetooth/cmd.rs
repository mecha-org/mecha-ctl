#![deny(clippy::all)]
use anyhow::{Result,bail};
use clap::{Args, Subcommand};


pub use mecha_bluetooth_ctl::{BluetoothControl,BluetoothErrorCodes as BluetoothSDKErrorCode};
use crate::bluetooth::{BluetoothError, BluetoothErrorCodes};

#[derive(Debug, Args)]
#[command(name = "bluetooth")]
pub struct Bluetooth {
    #[command(subcommand)]
    command: BluetoothCommand,
}

#[derive(Debug, Subcommand)]
enum BluetoothCommand {
    #[command(about = "Scan for bluetooth devices")]
    Scan,

    #[command(about = "Connect to a bluetooth device")]
    Connect(BluetoothConnectArgs),

    #[command(about = "Disconnect from a bluetooth device")]
    Disconnect(BluetoothDisconnectArgs),

    //status of bluetooth
    #[command(about = "Get the status of bluetooth")]
    Status,

    //turn on bluetooth
    #[command(about = "Turn on bluetooth")]
    On,

    //turn off bluetooth
    #[command(about = "Turn off bluetooth")]
    Off,
}

#[derive(Debug, Args)]
struct BluetoothConnectArgs {
    #[arg(required = true)]
    address: String,
}

#[derive(Debug, Args)]
struct BluetoothDisconnectArgs {
    #[arg(required = true)]
    address: String,
}

impl Bluetooth {
    pub async fn execute(&self) -> Result<()> { 
        let controller = match BluetoothControl::new().await {
            Ok(controller) => controller,
            Err(err) => {
                println!("Error: {}", err);
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToDetectBluetooth,
                    "unable to detect bluetooth".to_string()
                ))
            }
        };
        match &self.command {
            BluetoothCommand::Scan => {
                println!("Scanning for bluetooth devices...");
            }
            BluetoothCommand::Connect(args) => {
                println!(
                    "Connecting to bluetooth device with address: {}",
                    args.address
                );
            }
            BluetoothCommand::Disconnect(args) => {
                println!(
                    "Disconnecting from bluetooth device with address: {}",
                    args.address
                );
            }
            BluetoothCommand::Status => match controller.bluetooth_status().await {
                Ok(status) => {
                    println!("Bluetooth status: {}", status);
                }
                Err(e) => {
                  bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToDetectBluetooth,
                    "unable to detect bluetooth".to_string()))
                }
            
            },
            
            BluetoothCommand::On => match controller.enable_bluetooth().await {
                Ok(_) => {
                    println!("Bluetooth turned on");
                }
                Err(e) => {
                    bail!(BluetoothError::new(
                        BluetoothErrorCodes::UnableToConnectBluetooth,
                        "unable to connect bluetooth".to_string()))
                    }
                },
            
            

            BluetoothCommand::Off =>  match controller.disable_bluetooth().await {
                Ok(_) => {
                    println!("Bluetooth turned on");
                }
                Err(e) => {
                    bail!(BluetoothError::new(
                        BluetoothErrorCodes::UnableToConnectBluetooth,
                        "unable to connect bluetooth".to_string()))
                    }
                },
        }
        Ok(())
    }
}
