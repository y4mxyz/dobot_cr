#[derive(Debug)]
pub enum Error {

    CommunictionError,
    ArgumentError,
    MessageError,
    DobotError(ErrorCode),

}

#[derive(Debug)]
pub enum ErrorCode {

    CommandExecuteFailed,   // -1
    RobotStateError,        // -2
    EmergencyStop,          // -3
    PowerDown,              // -4

    ErrorId(isize),

}

impl ErrorCode {
    
    pub fn from(code: isize) -> Self {
        match code {
            -1 => Self::CommandExecuteFailed,
            -2 => Self::RobotStateError,
            -3 => Self::EmergencyStop,
            -4 => Self::PowerDown,
            _ => Self::ErrorId(code),
        }
    }

}