mod battery_ctl_service;
pub use battery_ctl_service::{BatteryControl, PowerSupplyServiceServer,Battery};

mod bluetooth_manager;
pub use bluetooth_manager::{Bluetooth, BluetoothServiceServer};

mod network_manager_service;
pub use network_manager_service::{NetworkManager, NetworkManagerServiceServer};