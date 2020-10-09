use core::fmt;

use atomic_refcell::AtomicRefCell;
use x86_64::instructions::port::PortWriteOnly;

pub struct Serial;

pub static PORT: AtomicRefCell<PortWriteOnly<u8>> = AtomicRefCell::new(PortWriteOnly::new(0x3f8));

static LOG_LEVEL_NAMES: [&str; 5] = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[derive(Clone, Copy)]
pub enum LogLevel {
    Error = 0,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(LOG_LEVEL_NAMES[*self as usize])
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut port = PORT.borrow_mut();
        for b in s.bytes() {
            unsafe { port.write(b) }
        }
        Ok(())
    }
}

macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!(crate::logger::Serial, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! debug {
    ($format:expr) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Debug);
    );
    ($format:expr, $($args:tt)*) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Debug, $($args)*);
    )
}

#[macro_export]
macro_rules! warn {
    ($format:expr) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Warn);
    );
    ($format:expr, $($args:tt)*) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Warn, $($args)*);
    )
}

#[macro_export]
macro_rules! info {
    ($format:expr) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Info);
    );
    ($format:expr, $($args:tt)*) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Info, $($args)*);
    )
}

#[macro_export]
macro_rules! error {
    ($format:expr) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Error);
    );
    ($format:expr, $($args:tt)*) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Error, $($args)*);
    )
}

#[macro_export]
macro_rules! trace {
    ($format:expr) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Trace);
    );
    ($format:expr, $($args:tt)*) => (
        log!(concat!("{:5} - ", $format), $crate::logger::LogLevel::Trace, $($args)*);
    )
}
