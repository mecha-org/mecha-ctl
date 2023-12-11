mod battery_ctl_service;
pub use battery_ctl_service::{BatteryControl, PowerSupplyServiceServer,Battery};

mod bluetooth_ctl_service;
pub use bluetooth_ctl_service::{Bluetooth, BluetoothServiceServer};

mod network_ctl_service;
pub use network_ctl_service::{NetworkManager, NetworkManagerServiceServer};

mod display_ctl_service;
pub use display_ctl_service::{DisplayControl,DisplayCtrlServiceServer};