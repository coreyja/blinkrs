use super::constants::{FADE_COMMAND_ACTION, IMMEDIATE_COMMAND_ACTION};
use super::Color;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
/// Represents which LED to use on the m2 variant.
/// This is ignored on the m1
pub enum LedNum {
  /// Use all/both LEDS
  All,
  /// Use only LED1
  Led1,
  /// Use only LED2
  Led2,
}

impl LedNum {
  /// Get the u8 that is used in the blink1 USB protocol
  pub fn as_u8(&self) -> u8 {
    match self {
      LedNum::All => 0,
      LedNum::Led1 => 1,
      LedNum::Led2 => 2,
    }
  }
}

/// Represents a command processable by the specification outlined in the [blink1 docs](https://git.io/JenDr).
/// Fade is the only message that supports sending to a specific LED since by trial and error that
/// was the only one that worked (even though immediate is documented to work).
#[derive(Debug, Copy, Clone)]
pub enum Message {
  /// Turn off the LEDs
  Off,
  /// Fade to the specified color over the given duration. Supports using a specific LED
  Fade(Color, Duration, LedNum),
  /// Set the LED(s) to the specified color without fading
  Immediate(Color),
  /// Set the color line pattern for the given position [position is the untyped u8]
  SetLinePattern(Color, Duration, u8),
  /// Set the Led Num. Doing this command BEFORE the SetLinePattern command will make the following
  /// line patterns use only the specified led. This lasts until it is set again
  /// Does NOT apply to usages of Fade and Immediate
  SetLedNum(LedNum),
  /// Start/stop the loop on the blink1
  PlayLoop {
    /// Whether to start or stop the loop
    on: bool,
    /// The position in the loop to start playing from
    start_pos: u8,
    /// The position in the loop to stop playing at
    end_pos: u8,
    /// How many times to loop over the animation
    loop_count: u8,
  },
}

impl Message {
  /// Returns the buffer that will be written to the blink(1) usb device based on the specification
  /// outlined in the [blink1 docs](https://git.io/JenDr).
  pub fn buffer(&self) -> [u8; 8] {
    match self {
      Message::Off => Message::Immediate(Color::Three(0x00, 0x00, 0x00)).buffer(),
      Message::Fade(color, duration, ledn) => {
        let (r, g, b) = color.rgb();
        // divide by 10 and truncate into two parts
        let dms = duration.as_millis().checked_div(10).unwrap_or(0) as u16;
        let th = dms.checked_shr(8).unwrap_or(0) as u8;
        let tl = dms.checked_rem(0xff).unwrap_or(0) as u8;
        [0x01, FADE_COMMAND_ACTION, r, g, b, th, tl, ledn.as_u8()]
      }
      Message::Immediate(color) => {
        let (r, g, b) = color.rgb();
        [0x01, IMMEDIATE_COMMAND_ACTION, r, g, b, 0x00, 0x00, 0x00]
      }
      &Message::SetLinePattern(color, duration, pos) => {
        let (r, g, b) = color.rgb();

        // divide by 10 and truncate into two parts
        let dms = duration.as_millis().checked_div(10).unwrap_or(0) as u16;
        let th = dms.checked_shr(8).unwrap_or(0) as u8;
        let tl = dms.checked_rem(0xff).unwrap_or(0) as u8;

        [0x01, 80, r, g, b, th, tl, pos]
      }
      &Message::SetLedNum(ledn) => [0x01, 108, ledn.as_u8(), 0, 0, 0, 0, 0],
      &Message::PlayLoop {
        on,
        start_pos,
        end_pos,
        loop_count,
      } => {
        let on_u8 = if on { 1 } else { 0 };

        [0x01, 112, on_u8, start_pos, end_pos, loop_count, 0, 0]
      }
    }
  }
}

impl From<&str> for Message {
  fn from(input: &str) -> Self {
    Message::Immediate(Color::from(input))
  }
}

#[cfg(test)]
mod tests {
  use super::Message;

  #[test]
  fn test_red() {
    let red = Message::from("red");
    assert_eq!(red.buffer()[2..5], [0xff, 0x00, 0x00])
  }

  #[test]
  fn test_green() {
    let red = Message::from("green");
    assert_eq!(red.buffer()[2..5], [0x00, 0xff, 0x00])
  }

  #[test]
  fn test_blue() {
    let red = Message::from("blue");
    assert_eq!(red.buffer()[2..5], [0x00, 0x00, 0xff])
  }

  #[test]
  fn test_off() {
    let red = Message::from("off");
    assert_eq!(red.buffer()[2..5], [0x00, 0x00, 0x00])
  }
}
