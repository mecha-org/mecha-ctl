#![deny(clippy::all)]

mod cpu_ctrl;
pub use cpu_ctrl::{CpuCtl,CpuFrequency};

mod errors;
pub use errors::{CpuCtlError, CpuCtlErrorCodes};