pub mod audio;
pub mod error;
pub mod frame;
pub mod index;
pub mod pixel;
pub mod resample;
pub mod track;
pub mod video;

use ffms2_sys::{
    FFMS_Deinit, FFMS_GetLogLevel, FFMS_GetVersion, FFMS_Init, FFMS_LogLevels,
    FFMS_SetLogLevel,
};

/// Logging/Message level.
#[derive(Clone, Copy, Debug, Default)]
pub enum LogLevel {
    /// Show nothing.
    Quiet,
    /// Show fatal errors which could lead the process to crash, such as an
    /// assertion failure.
    Panic,
    /// Show fatal errors after which the process cannot absolutely continue.
    Fatal,
    /// Show all errors, including ones which can be recovered from.
    Error,
    /// Show all warnings and errors.
    ///
    /// Any message related to possibly incorrect or unexpected events
    /// will be shown.
    Warning,
    /// Show informative messages during processing.
    ///
    /// This is in addition to warnings and errors.
    #[default]
    Info,
    /// Same as `Info`, except more verbose.
    Verbose,
    /// Show everything, including debugging information.
    Debug,
    /// Show everything in addition to function tracing information.
    Trace,
}

impl LogLevel {
    const fn ffms2_log_level(self) -> FFMS_LogLevels {
        match self {
            Self::Quiet => FFMS_LogLevels::FFMS_LOG_QUIET,
            Self::Panic => FFMS_LogLevels::FFMS_LOG_PANIC,
            Self::Fatal => FFMS_LogLevels::FFMS_LOG_FATAL,
            Self::Error => FFMS_LogLevels::FFMS_LOG_ERROR,
            Self::Warning => FFMS_LogLevels::FFMS_LOG_WARNING,
            Self::Info => FFMS_LogLevels::FFMS_LOG_INFO,
            Self::Verbose => FFMS_LogLevels::FFMS_LOG_VERBOSE,
            Self::Debug => FFMS_LogLevels::FFMS_LOG_DEBUG,
            Self::Trace => FFMS_LogLevels::FFMS_LOG_TRACE,
        }
    }

    const fn new(log_level: i32) -> Self {
        match log_level {
            e if e == FFMS_LogLevels::FFMS_LOG_QUIET as i32 => Self::Quiet,
            e if e == FFMS_LogLevels::FFMS_LOG_PANIC as i32 => Self::Panic,
            e if e == FFMS_LogLevels::FFMS_LOG_FATAL as i32 => Self::Fatal,
            e if e == FFMS_LogLevels::FFMS_LOG_ERROR as i32 => Self::Error,
            e if e == FFMS_LogLevels::FFMS_LOG_WARNING as i32 => Self::Warning,
            e if e == FFMS_LogLevels::FFMS_LOG_INFO as i32 => Self::Info,
            e if e == FFMS_LogLevels::FFMS_LOG_VERBOSE as i32 => Self::Verbose,
            e if e == FFMS_LogLevels::FFMS_LOG_DEBUG as i32 => Self::Debug,
            e if e == FFMS_LogLevels::FFMS_LOG_TRACE as i32 => Self::Trace,
            _ => Self::Error,
        }
    }
}

/// Log level settings.
pub struct Log;

impl Log {
    /// Returns the current logging/message level.
    pub fn log_level() -> LogLevel {
        let log = unsafe { FFMS_GetLogLevel() };
        LogLevel::new(log)
    }

    /// Sets logging/message level.
    pub fn set_log_level(level: LogLevel) {
        unsafe {
            FFMS_SetLogLevel(LogLevel::ffms2_log_level(level) as i32);
        }
    }
}

/// The `ffms2` initializer.
pub struct FFMS2;

impl FFMS2 {
    /// Initializes the `ffms2` library.
    ///
    /// It must be called once at the start of a program, before doing any other
    /// `ffms2` function calls. This function is thread safe and will only
    /// run once.
    pub fn init() {
        unsafe {
            FFMS_Init(0, 0);
        }
    }

    /// Returns the `ffms2` version.
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
