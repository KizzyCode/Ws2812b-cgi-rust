//! The HTTP<->Serial-Driver bridge

use crate::{error, error::Error, serial::SerialDevice, BAUDRATE, SERIALDEVICE};
use serde::Deserialize;
use std::io::{self, Read, Write};

/// A set LED message
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Update {
    /// The index of the WS2812B strip to update
    strip: usize,
    /// The index of the pixel to update
    pixel: usize,
    /// The new RGBW value
    rgbw: (u8, u8, u8, u8),
}
impl Update {
    /// Converts the message into the appropriate serial command
    pub fn into_serial_message(self) -> Result<String, Error> {
        // Validate strip and pixel range
        let strip @ 0..=3 = self.strip else {
            return Err(error!("LED strip index is too large ({})", self.strip));
        };
        let pixel @ 0..=511 = self.pixel else {
            return Err(error!("Pixel index is too large ({})", self.pixel));
        };
        let (r, g, b, 0) = self.rgbw else {
            return Err(error!("Unsupported white value in RGBW ({})", self.rgbw.3));
        };

        // Compute encode message
        let message = format!("{strip:04x}{pixel:04x}{r:02x}{g:02x}{b:02x}00\n");
        Ok(message)
    }

    pub fn _into_serial_message(self) -> Result<[u8; 6], Error> {
        // Validate strip and pixel range
        let strip @ 0..=3 = self.strip else {
            return Err(error!("LED strip index is too large ({})", self.strip));
        };
        let pixel @ 0..=255 = self.pixel else {
            return Err(error!("Pixel index is too large ({})", self.pixel));
        };
        let (r, g, b, 0) = self.rgbw else {
            return Err(error!("Unsupported white value in RGBW ({})", self.rgbw.3));
        };

        // Compute CRC and encode message
        let message = [strip as u8, pixel as u8, r, g, b, b'\n'];
        Ok(message)
    }
}

/// Reads the request body and parses it into update requests
pub fn parse_request() -> Result<Vec<Update>, Error> {
    // Limit stdin
    let mut stdin = io::stdin().bytes();
    let stdin_limited = (&mut stdin).take(16384);

    // Read the body
    let mut body = Vec::with_capacity(16384);
    for byte in stdin_limited {
        body.push(byte?);
    }

    // Ensure that the body is empty now
    if stdin.next().is_some() {
        return Err(error!("Request body is too large"));
    }

    // Parse the request
    let updates: Vec<Update> = serde_json::from_slice(&body)?;
    Ok(updates)
}

/// Forwards the update request to the serial driver
pub fn forward_request(updates: &[Update]) -> Result<(), Error> {
    // Send the update to the serial device
    let mut serial = SerialDevice::new(SERIALDEVICE, BAUDRATE)?;
    for update in updates {
        // Serialize and send the message
        let message = update.into_serial_message()?;
        serial.write_all(message.as_bytes())?;

        // Await response
        let mut response = vec![0; message.len()];
        serial.read_exact(&mut response)?;

        // Ensure that our response is equal to our request
        if response != message.as_bytes() {
            println!("expected: {:?}", message.as_bytes());
            println!("     got: {:?}", response.as_slice());
            return Err(error!("Transaction failed"));
        }
    }
    Ok(())
}
