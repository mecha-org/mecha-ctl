use anyhow::{bail, Result};
use clap::{Args, Subcommand};

pub use mecha_battery_ctl::{Battery as Power, PowerSupplyInfo};

use crate::battery::{BatteryError, BatteryErrorCodes};

#[derive(Debug, Args)]
pub struct Battery {
    #[command(subcommand)]
    command: BatteryCommands,
}

#[derive(Debug, Subcommand)]
enum BatteryCommands {
    #[command(about = "Get battery info")]
    Info,
}

impl Battery {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            BatteryCommands::Info => {
                let battery = Power {
                    path: "hello".to_string(),
                    currnet_now: "hello".to_string(),
                };

                let _ = match battery.info() {
                    Ok(power) => power,
                    Err(err) => {
                        println!("Error: {}", err);
                        bail!(BatteryError::new(
                            BatteryErrorCodes::UnableToDetectBattery,
                            "unable to get battery info".to_string()
                        ))
                    }
                };

                Ok(())
            }
        }
    }
}
