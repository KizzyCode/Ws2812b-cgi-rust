#![doc = include_str!("../README.md")]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]

mod bridge;
mod error;
mod serial;

use crate::error::Error;
use std::{process, thread, time::Duration};

/// The baudrate to use (we can use a high baudrate here since the driver uses USB-CDC serial)
const BAUDRATE: u64 = 921600;
/// The path to the WS2812B driver
const SERIALDEVICE: &str = match option_env!("WS2812B_CGI_SERIALDEVICE") {
    Some(path) => path,
    None => "/dev/ws2812b.serial",
};
/// The timeout of a CGI process (to detect stale locks etc.)
const TIMEOUT_SECS: &str = match option_env!("WS2812B_CGI_TIMEOUT") {
    Some(timeout) => timeout,
    None => "10",
};

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    // Start the watchdog thread
    thread::spawn(|| {
        // Parse the timeout
        #[allow(clippy::expect_used)]
        let timeout: u64 = TIMEOUT_SECS.parse().expect("failed to parse timeout");
        let timeout = Duration::from_secs(timeout);

        // Sleep for timeout duration and terminate the process if it is still running
        thread::sleep(timeout);
        eprintln!("Watchdog timeout reached");
        process::abort();
    });

    /// Fallible main logic
    fn try_() -> Result<(), Error> {
        let request = bridge::parse_request()?;
        bridge::forward_request(&request)?;
        Ok(())
    }

    // Try to run the main code
    if let Err(e) = try_() {
        eprintln!("Fatal error: {e}");
        process::exit(1);
    }

    // Exit gracefully
    print!("HTTP/1.1 200 OK\r\n");
    print!("Content-Length: 0\r\n");
    print!("\r\n");
    process::exit(0);
}
