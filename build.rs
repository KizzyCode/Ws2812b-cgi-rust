use cc::Build;
use std::env::consts::FAMILY;

/// Select the platform specific serial implementation
fn select_serial_impl() -> &'static str {
    match FAMILY {
        "unix" => "src/serial/unix.c",
        family => panic!("Unsupported target OS family: {family}"),
    }
}

fn main() {
    // Build and link the serial implementation
    let serial_impl = select_serial_impl();
    Build::new().file(serial_impl).extra_warnings(true).warnings_into_errors(true).compile("serial");
}
