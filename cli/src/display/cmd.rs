use anyhow::{bail, Result};
use clap::{Args, Subcommand};
pub use mecha_display_ctl::DisplayControl;
use tracing_subscriber::field::display;

use crate::display::{DisplayError, DisplayErrorCodes};

#[derive(Debug, Args)]
pub struct Display {
    #[command(subcommand)]
    command: DisplayCommands,
}

#[derive(Debug, Subcommand)]
enum DisplayCommands {
    #[command(about = "Get Display brightness")]
    GetBrightness,
    #[command(about = "Set Display brightness")]
    SetBrightness { brightness: u8 },
}

impl Display {
    pub async fn execute(&self) -> Result<()> {
        // Use match to handle errors when creating a new DisplayControl instance
        let display = match DisplayControl::new("path/to/display") {
            Ok(display) => display,
            Err(err) => {
                println!("Error: {}", err);
                bail!(DisplayError::new(
                    DisplayErrorCodes::Unknown,
                    "unable to get display".to_string()
                ))
            }
        };

        match &self.command {
            DisplayCommands::GetBrightness => {
                let brightness = match display.get_display_brightness() {
                    Ok(brightness) => brightness,
                    Err(err) => {
                        println!("Error: {}", err);
                        bail!(DisplayError::new(
                            DisplayErrorCodes::UnableToGetBrightness,
                            "unable to get display brightness".to_string()
                        ))
                    }
                };

                println!("Current brightness: {}", brightness);
                Ok(())
            }
            DisplayCommands::SetBrightness { brightness } => {
                match display.set_display_brightness(*brightness) {
                    Ok(_) => {
                        println!("Brightness set to {}", brightness);
                        Ok(())
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        bail!(DisplayError::new(
                            DisplayErrorCodes::UnableToSetBrightness,
                            "unable to set display".to_string()
                        ))
                    }
                }
            }
        }
    }
}
