#[macro_use]
mod utility;

pub mod audio;
pub mod error;
pub mod frame;
pub mod index;
pub mod resample;
pub mod track;
pub mod video;

use ffms2_sys::*;

create_enum!(
    SampleFormat,
    FFMS_SampleFormat,
    sample_format,
    (FMT_U8, FMT_S16, FMT_S32, FMT_FLT, FMT_DBL)
);

create_enum!(
    LogLevel,
    FFMS_LogLevels,
    log_level,
    (
        LOG_QUIET,
        LOG_PANIC,
        LOG_FATAL,
        LOG_ERROR,
        LOG_WARNING,
        LOG_INFO,
        LOG_VERBOSE,
        LOG_DEBUG,
        LOG_TRACE,
    )
);

from_i32!(
    LogLevel,
    FFMS_LogLevels,
    (
        LOG_QUIET,
        LOG_PANIC,
        LOG_FATAL,
        LOG_ERROR,
        LOG_WARNING,
        LOG_INFO,
        LOG_VERBOSE,
        LOG_DEBUG,
        LOG_TRACE,
    )
);

/// Log level settings.
pub struct Log;

impl Log {
    /// Returns the current log level.
    pub fn log_level() -> LogLevel {
        let log = unsafe { FFMS_GetLogLevel() };
        LogLevel::from_i32(log)
    }

    /// Sets a log level.
    pub fn set_log_level(level: LogLevel) {
        unsafe {
            FFMS_SetLogLevel(LogLevel::to_log_level(level) as i32);
        }
    }
}

/// FFMS2 initializer.
pub struct FFMS2;

impl FFMS2 {
    /// Initializes FFMS2.
    pub fn init() {
        unsafe {
            FFMS_Init(0, 0);
        }
    }

    /// Returns FFMS2 version.
    pub fn version() -> usize {
        unsafe { FFMS_GetVersion() as usize }
    }
}

impl Drop for FFMS2 {
    fn drop(&mut self) {
        unsafe {
            FFMS_Deinit();
        }
    }
}
