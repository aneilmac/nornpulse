
pub struct FlightRecorder { }

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

pub static FLIGHT_RECORDER : FlightRecorder = FlightRecorder { };
