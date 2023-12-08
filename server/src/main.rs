use anyhow::{bail, Result};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::{fs::File, io::BufReader};
use tracing::{info, Level};
use tracing_subscriber;

use tonic::transport::Server;

mod configs;
use crate::configs::BaseConfig;

mod services;
use crate::services::{Battery, BatteryControl, PowerSupplyServiceServer};
use crate::services::{Bluetooth, BluetoothServiceServer};
use crate::services::{NetworkManager, NetworkManagerServiceServer};

#[tokio::main]
async fn main() -> Result<()> {
    let profile_file =
        File::open("./mecha-ctl/server/Config.yml").expect("Failed to open config file");
    let reader = BufReader::new(profile_file);

    let config: BaseConfig = serde_yaml::from_reader(reader).expect("unable to rad yaml file");

    //port for grpc server
    let port = config.server.port;
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));

    let battery = Battery {
        path: config.interfaces.battery.device.as_str().to_string(),
        currnet_now: config.interfaces.battery.current.as_str().to_string(),
    };

    //power service
    let power_supply = BatteryControl {
        power_supply: battery,
    };

    //network manager service
    let network_service = NetworkManager::default();

    //bluetooth service

    println!("Mecha Edge Server listening on {}", addr);

    let subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!(
        task = "mecha_grpc_tracer",
        result = "success",
        "grpc server started"
    );
    Server::builder()
        .add_service(PowerSupplyServiceServer::new(power_supply))
        .add_service(NetworkManagerServiceServer::new(network_service))
        .add_service(BluetoothServiceServer::new(Bluetooth::default()))
        .serve(addr)
        .await?;

    Ok(())
}
