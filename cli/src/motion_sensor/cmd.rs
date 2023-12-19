use anyhow::Result;
use clap::{Args, Subcommand};

pub use mecha_motion_sensor_ctl::MotionSensorControl;

use crate::output_message::{Message, StdOut, MOTION};

#[derive(Debug, Args)]
pub struct MotionSensor {
    #[command(subcommand)]
    command: MotionSensorCommands,
}

#[derive(Debug, Subcommand)]
enum MotionSensorCommands {
    #[command(about = "Get motion sensor value")]
    Value,
    #[command(about = "Get motion sensor event")]
    Event,
}

impl MotionSensor {
    pub async fn execute(&self) -> Result<()> {
        let motion_sensor = MotionSensorControl::new(
            "/sys/bus/iio/devices/iio:device0/in_accel_x_raw",
            "/sys/bus/iio/devices/iio:device0/in_accel_y_raw",
            "/sys/bus/iio/devices/iio:device0/in_accel_z_raw",
        );

        match &self.command {
            MotionSensorCommands::Value => match motion_sensor.read_motion_sensor_value() {
                Ok((x, y, z)) => {
                    println!();
                    StdOut::info(
                        &format!("Motion sensor value: x: {}, y: {}, z: {}", x, y, z),
                        Some(MOTION),
                    )
                }
                Err(err) => return Err(err.into()),
            },
            MotionSensorCommands::Event => match motion_sensor.detect_motion_sensor_event() {
                Ok(event) => StdOut::info(
                    &format!("Motion sensor event: event: {}", event),
                    Some(MOTION),
                ),
                Err(err) => return Err(err.into()),
            },
        }
        Ok(())
    }
}
