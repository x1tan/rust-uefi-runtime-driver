use core::fmt;
use core::fmt::Write;

use atomic_refcell::AtomicRefCell;
use x86_64::instructions::port::PortWriteOnly;

#[derive(Default)]
pub struct Logger;

pub struct Serial;

pub static PORT: AtomicRefCell<PortWriteOnly<u8>> = AtomicRefCell::new(PortWriteOnly::new(0x3f8));
pub static mut LOGGER: Option<Logger> = None;

impl Logger {
    pub fn initialize() {
        unsafe {
            // Construct the logger.
            let logger = {
                LOGGER = Some(Logger::default());
                LOGGER.as_ref().unwrap()
            };

            // Set the logger.
            log::set_logger(logger).unwrap(); // Can only fail if already initialized.
        }

        // Log everything.
        log::set_max_level(log::LevelFilter::Debug);
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let _ = writeln!(Serial, "{:5} {}", record.level(), *record.args());
    }

    fn flush(&self) {
        // This simple logger does not buffer output.
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
