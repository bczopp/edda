//! Minimal logging (Phase 11.1.1). ERROR and WARN only; on host logs to stderr.
//! On ESP32, wire to Serial for debugging (e.g. defmt or custom backend).

use log::{Level, LevelFilter, Log, Metadata, Record};

struct MinimalLogger;

impl Log for MinimalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Warn
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("[{}] {}: {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

/// Initialize minimal logging (ERROR, WARN only). Call once at startup.
/// On host: logs to stderr. On ESP32: replace with Serial/defmt backend.
pub fn init_minimal_logging() -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(MinimalLogger))?;
    log::set_max_level(LevelFilter::Warn);
    Ok(())
}
