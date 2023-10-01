//! A serial device

use crate::error::Error;
use std::{
    ffi::{c_char, CStr, CString},
    io::{self, Read, Write},
};

extern "C" {
    // const char* ws2812bcgi_serial_open(int64_t* fd, const uint8_t* path, uint64_t bauds)
    fn ws2812bcgi_serial_open(fd: *mut i64, path: *const u8, bauds: u64) -> *const c_char;

    // const char* ws2812bcgi_serial_read_buf(uint8_t* buf, size_t* pos, size_t capacity, int64_t fd)
    fn ws2812bcgi_serial_read_buf(buf: *mut u8, pos: *mut usize, capacity: usize, fd: i64) -> *const c_char;

    // const char* ws2812bcgi_serial_write_buf(int64_t fd, const uint8_t* buf, size_t* pos, size_t capacity)
    fn ws2812bcgi_serial_write_buf(fd: i64, buf: *const u8, pos: *mut usize, capacity: usize) -> *const c_char;

    // const char* ws2812bcgi_serial_flush(int64_t fd)
    fn ws2812bcgi_serial_flush(fd: i64) -> *const c_char;

    // const char* ws2812bcgi_serial_close(int64_t fd)
    fn ws2812bcgi_serial_close(fd: i64) -> *const c_char;
}

/// Performs an FFI call
macro_rules! ffi {
    ($fn:ident $(, $arg:expr)*) => {{
        // Call function
        let result = $fn($($arg),*);
        if !result.is_null() {
            // Get the error information
            let os_err = std::io::Error::last_os_error();
            let desc = CStr::from_ptr(result).to_string_lossy();

            // Create the I/O error
            let full_desc = format!("{desc} ({os_err})");
            let err = std::io::Error::new(os_err.kind(), full_desc);
            Err(err)
        } else {
            // Return ok status
            Ok(())
        }
    }};
}

/// A serial device
pub struct SerialDevice {
    /// The underlying file descriptor
    fd: i64,
}
impl SerialDevice {
    /// Opens a serial device
    pub fn new(path: &str, baudrate: u64) -> Result<Self, Error> {
        // Prepare the path
        let path = CString::new(path)?;
        let path_ptr = path.as_bytes_with_nul().as_ptr();

        // Open the file
        let mut fd = -1;
        unsafe { ffi!(ws2812bcgi_serial_open, &mut fd, path_ptr, baudrate) }?;
        Ok(Self { fd })
    }
}
impl Read for SerialDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut pos = 0;
        unsafe { ffi!(ws2812bcgi_serial_read_buf, buf.as_mut_ptr(), &mut pos, buf.len(), self.fd) }?;
        Ok(pos)
    }
}
impl Write for SerialDevice {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut pos = 0;
        unsafe { ffi!(ws2812bcgi_serial_write_buf, self.fd, buf.as_ptr(), &mut pos, buf.len()) }?;
        Ok(pos)
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe { ffi!(ws2812bcgi_serial_flush, self.fd) }
    }
}
impl Drop for SerialDevice {
    fn drop(&mut self) {
        let _ = unsafe { ffi!(ws2812bcgi_serial_close, self.fd) };
    }
}
