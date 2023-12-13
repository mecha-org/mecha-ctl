use anyhow::{bail, Result};
use clap::{Args, Subcommand};

use mecha_cpu_governor_ctl::{
    CpuFrequency, CpuGovernanceCtl, CpuGovernanceCtlError, CpuGovernanceCtlErrorCodes,
};

#[derive(Debug, Args)]
pub struct CpuGoverner {
    #[command(subcommand)]
    command: CpuGovernerCommands,
}

#[derive(Debug, Subcommand)]
enum CpuGovernerCommands {
    #[command(about = "Get cpu frequency")]
    GetFrequency,
    #[command(about = "set cpu frequency")]
    SetFrequency(CpuFrequencyControl),
}

#[derive(Debug, Args)]
pub struct CpuFrequencyControl {
    #[arg(required = true, short = 'f')]
    frequency: u64,
}

impl CpuGoverner {
    pub async fn execute(&self) -> Result<()> {
        let cpu_governer_control = CpuGovernanceCtl::new();
        match &self.command {
            CpuGovernerCommands::GetFrequency => match cpu_governer_control.get_cpu_frequency() {
                Ok(frequency) => {
                    println!("Current frequency is {}", frequency);
                }
                Err(e) => {
                    bail!(CpuGovernanceCtlError::new(
                        CpuGovernanceCtlErrorCodes::FailedToGetCpuFrequency,
                        format!("Error getting cpu frequency: {}", e)
                    ),)
                }
            },
            CpuGovernerCommands::SetFrequency(frequency) => {
                match cpu_governer_control.set_cpu_frequency(CpuFrequency::Freq1200000) {
                    Ok(_) => {
                        println!("Set frequency to {}", frequency.frequency);
                    }
                    Err(e) => {
                        bail!(CpuGovernanceCtlError::new(
                            CpuGovernanceCtlErrorCodes::FailedToSetCpuFrequency,
                            format!("Error setting cpu frequency: {}", e)
                        ),);
                    }
                }
            }
        }
        Ok(())
    }
}
