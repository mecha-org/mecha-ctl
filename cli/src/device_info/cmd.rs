use anyhow::{bail, Result};
use clap::{Args, Subcommand};

use mecha_device_info_ctl::{DeviceInfoCtl, DeviceInfoCtlError, DeviceInfoCtlErrorCodes};

#[derive(Debug, Args)]
pub struct DeviceInfo {
    #[command(subcommand)]
    command: DeviceInfoCommands,
}

#[derive(Debug, Subcommand)]
enum DeviceInfoCommands {
    Cpu(Cpu),
    Memory(Memory),
    Storage(Storage),
}

//create cpu args
#[derive(Debug, Args)]
pub struct Cpu {
    #[command(subcommand)]
    command: CpuCommands,
}

//create cpu subcommands
#[derive(Debug, Subcommand)]
enum CpuCommands {
    #[command(about = "Get cpu usage")]
    Usage,
    #[command(about = "Get cpu info")]
    Info,
}

#[derive(Debug, Args)]
pub struct Memory {
    #[command(subcommand)]
    command: MemoryCommands,
}

#[derive(Debug, Subcommand)]
enum MemoryCommands {
    #[command(about = "Get memory usage")]
    Usage,
    #[command(about = "Get memory info")]
    Info,
}

#[derive(Debug, Args)]
pub struct Storage {
    #[command(subcommand)]
    command: StorageCommands,
}

#[derive(Debug, Subcommand)]
enum StorageCommands {
    #[command(about = "Get storage usage")]
    Usage,
    #[command(about = "Get storage info")]
    Info,
}

impl DeviceInfo {
    pub async fn execute(&self) -> Result<()> {
        let device_info = DeviceInfoCtl::new();
        match &self.command {
            DeviceInfoCommands::Cpu(cpu) => {
                // Handle CPU commands

                match &cpu.command {
                    CpuCommands::Usage => {
                        println!("CPU Usage: 50%");
                    }
                    CpuCommands::Info => match device_info.get_cpu_info() {
                        Ok(info) => {
                            println!("Cpu info {:?}", info);
                        }
                        Err(e) => {
                            bail!(DeviceInfoCtlError::new(
                                DeviceInfoCtlErrorCodes::UnknownError,
                                "uable to get cpu info".to_string()
                            ))
                        }
                    },
                }
            }
            DeviceInfoCommands::Memory(memory) => {
                // Handle Memory commands
                match &memory.command {
                    MemoryCommands::Usage => {
                        println!("Memory Usage: 8GB");
                    }
                    MemoryCommands::Info => match device_info.get_memory_info() {
                        Ok(info) => {
                            println!("info : {:?}", info);
                        }
                        Err(e) => {
                            bail!(DeviceInfoCtlError::new(
                                DeviceInfoCtlErrorCodes::FailedToGetMemoryUsage,
                                e.to_string()
                            ))
                        }
                    },
                }
            }
            DeviceInfoCommands::Storage(storage) => {
                // Handle Storage commands
                match &storage.command {
                    StorageCommands::Usage => {
                        println!("Storage Usage: 500GB");
                    }
                    StorageCommands::Info => match device_info.get_disk_info() {
                        Ok(info) => {
                            println!("disk info : {:?}", info)
                        }
                        Err(e) => {
                            bail!(DeviceInfoCtlError::new(
                                DeviceInfoCtlErrorCodes::FailedToGetDiskUsage,
                                e.to_string()
                            ))
                        }
                    },
                }
            }
        }
        Ok(())
    }
}
