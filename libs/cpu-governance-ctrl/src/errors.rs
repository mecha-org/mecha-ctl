#[derive(Debug)]
pub enum CpuCtlErrorCodes {
    FailedToSetCpuGovernor,
    FailedToSetCpuGovernorPath,
    FailedToGetCpuGovernor,
    FailedToGetCpuFrequency,
    FailedToSetCpuFrequency,
    FailedToSetCpuFrequencyPath,
    FailedToOpenFile,
    FailedToWriteToFile,
    FailedToReadFile,
    UnknownError,
}

//impl std::fmt::Display  for CpuCtlErrorCodes
impl std::fmt::Display for CpuCtlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CpuCtlErrorCodes::FailedToSetCpuGovernor => write!(f, "FailedToSetCpuGovernor"),
            CpuCtlErrorCodes::FailedToSetCpuGovernorPath => {
                write!(f, "FailedToSetCpuGovernorPath")
            }
            CpuCtlErrorCodes::FailedToGetCpuGovernor => write!(f, "FailedToGetCpuGovernor"),
            CpuCtlErrorCodes::FailedToGetCpuFrequency => write!(f, "FailedToGetCpuFrequency"),
            CpuCtlErrorCodes::FailedToSetCpuFrequency => write!(f, "FailedToSetCpuFrequency"),
            CpuCtlErrorCodes::FailedToSetCpuFrequencyPath => {
                write!(f, "FailedToSetCpuFrequencyPath")
            }
            CpuCtlErrorCodes::FailedToOpenFile => write!(f, "FailedToOpenFile"),
            CpuCtlErrorCodes::FailedToWriteToFile => write!(f, "FailedToWriteToFile"),
            CpuCtlErrorCodes::FailedToReadFile => write!(f, "FailedToReadFile"),
            CpuCtlErrorCodes::UnknownError => write!(f, "UnknownError"),
        }
    }
}

#[derive(Debug)]
pub struct CpuCtlError {
    pub code: CpuCtlErrorCodes,
    pub message: String,
}

//impl std::fmt::Display for CpuCtlError
impl std::fmt::Display for CpuCtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl CpuCtlError {
    pub fn new(code: CpuCtlErrorCodes, message: String) -> Self {
        CpuCtlError { code, message }
    }
}
