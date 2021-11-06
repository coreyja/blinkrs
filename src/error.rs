use std::fmt;
use thiserror::Error;

/// An Error from the USB Protocol
#[derive(Debug, Error)]
pub enum BlinkError {
  /// Could not find something.... Maybe a USB interface?
  /// TODO: Make this doc better
  #[error("not found")]
  NotFound,
  /// Could not list USB devices, wrapbs an rusb::Error
  #[error("device list error")]
  DeviceListError(#[from] rusb::Error),
}
