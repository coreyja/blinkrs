use std::error::Error;
use std::fmt;

/// An Error from the USB Protocol
#[derive(Debug)]
pub enum BlinkError {
  /// Could not find something.... Maybe a USB interface?
  /// TODO: Make this doc better
  NotFound,
  /// Could not list USB devices, wrapbs an rusb::Error
  DeviceListError(rusb::Error),
}

impl From<rusb::Error> for BlinkError {
  fn from(error: rusb::Error) -> Self {
    BlinkError::DeviceListError(error)
  }
}

impl fmt::Display for BlinkError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for BlinkError {
  fn description(&self) -> &str {
    match self {
      BlinkError::NotFound => "not found",
      BlinkError::DeviceListError(_e) => "unable to find usb device",
    }
  }
}
