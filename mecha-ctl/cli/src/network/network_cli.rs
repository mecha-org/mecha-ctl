#![deny(clippy::all)]

use anyhow::Result;
use clap::{Args, Subcommand};

pub use mecha_network_ctl::{Network, NetworkStatus};

pub use crete::network::{NetworkError, NetworkErrorCodes};

#[derive(Debug, Args)]
#[command(name = "network")]
pub struct Network {
    #[command(subcommand)]
    command: NetworkCommand,
}

#[derive(Debug, Subcommand)]
enum NetworkCommand {
    #[command(about = "Scan for wireless networks")]
    Scan,

    #[command(about = "Add a wireless network")]
    Add(WirelessAddArgs),

    #[command(about = "Remove a wireless network")]
    Remove(WirelessRemoveArgs),

    #[command(about = "Connect to a wireless network")]
    Connect(WirelessConnectArgs),
}

#[derive(Debug, Args)]
struct WirelessAddArgs {
    #[arg(required = true)]
    ssid: String,

    #[arg(required = true)]
    password: String,
}

#[derive(Debug, Args)]
struct WirelessRemoveArgs {
    #[arg(required = true)]
    ssid: String,
}

#[derive(Debug, Args)]
struct WirelessConnectArgs {
    #[arg(required = true)]
    ssid: String,

    #[arg(required = true)]
    password: String,
}

impl Network {
    pub async fn execute(&self) -> Result<()> {

        let mut network_module =  match Network::new().await {
            Ok(network_module) => network_module,
            Err(e) => {
                return Err(e);
            }
        };
        match &self.command {
            NetworkCommand::Scan => {
                let scan_results = match network_module.scan_wireless_network().await {
                    Ok(scan_results) => scan_results,
                    Err(e) => {
                        return Err(e);
                    }
                };


            }
            NetworkCommand::Add(args) => {
                let _ = match network_module.add_wireless_network(&args.ssid, &args.password).await {
                    Ok(add_results) => add_results,
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            NetworkCommand::Remove(args) => {
            

                //take ssid form arg and convert it into i32
                let network_id = args.ssid.parse::<i32>().unwrap();

                // use args and use remove_wireless_network
                let _ = match network_module.remove_wireless_network(&args.ssid).await {
                    Ok(remove_results) => remove_results,
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            NetworkCommand::Connect(args) => {
                let _ = match network_module.connect_wireless_network(&args.ssid, &args.password).await {
                    Ok(connect_results) => connect_results,
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
        }

        Ok(())
    }
}
