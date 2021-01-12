mod injected_calls;

pub use injected_calls::inject_calls;

use log::Log;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::os::raw::c_char;

pub struct FlightRecorder {}

impl log::Log for FlightRecorder {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub static FLIGHT_RECORDER: FlightRecorder = FlightRecorder {};

#[derive(FromPrimitive)]
#[repr(u32)]
enum FlightCategory {
    FlightRecordDestruct = 0x00,
    FatalError = 0x01,
    Shutdown = 0x10,
    PrayViolation = 0x20,
    Initializing = 0x40,
}

fn category_to_level(c: u32) -> log::Level {
    match FromPrimitive::from_u32(c) {
        Some(FlightCategory::FlightRecordDestruct) => log::Level::Debug,
        Some(FlightCategory::FatalError) => log::Level::Error,
        Some(FlightCategory::Shutdown) => log::Level::Debug,
        Some(FlightCategory::PrayViolation) => log::Level::Warn,
        Some(FlightCategory::Initializing) => log::Level::Debug,
        None => log::Level::Info,
    }
}
