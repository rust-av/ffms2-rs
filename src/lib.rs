#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
mod utility;

pub mod audio;
pub mod frame;
pub mod index;
pub mod resample;
pub mod track;
pub mod video;

use ffms2_sys::*;

use std::fmt;
use std::mem;
use std::ptr;
use std::str;

errors!(Errors, FFMS_Errors,
        (
            ERROR_SUCCESS: "Success.",
            ERROR_INDEX: "Error Index.",
            ERROR_INDEXING: "Error Indexing.",
            ERROR_POSTPROCESSING: "Error Postprocessing.",
            ERROR_SCALING: "Error Scaling.",
            ERROR_DECODING: "Erorr Decoding.",
            ERROR_SEEKING: "Error Seeking.",
            ERROR_PARSER: "Error Parser.",
            ERROR_TRACK: "Error in the track.",
            ERROR_WAVE_WRITER: "Error with the wave writer.",
            ERROR_CANCELLED: "Error cancelled.",
            ERROR_RESAMPLING: "Error resampling.",
            ERROR_UNKNOWN: "Error unknown.",
            ERROR_UNSUPPORTED: "Error unsupported.",
            ERROR_FILE_READ: "Error file read.",
            ERROR_FILE_WRITE: "Error file write.",
            ERROR_NO_FILE: "No file.",
            ERROR_VERSION: "Version error.",
            ERROR_ALLOCATION_FAILED: "Allocation failed.",
            ERROR_INVALID_ARGUMENT: "Invalid argument.",
            ERROR_CODEC: "Error with the codec.",
            ERROR_NOT_AVAILABLE: "Not available.",
            ERROR_FILE_MISMATCH: "File mismatch.",
            ERROR_USER: "Error caused by the user.",
        )
);

create_enum!(
    IndexErrorHandling,
    FFMS_IndexErrorHandling,
    idx_errors,
    (IEH_ABORT, IEH_CLEAR_TRACK, IEH_STOP_TRACK, IEH_IGNORE)
);

display!(IndexErrorHandling,
         (
           IEH_ABORT: "Index error aborting.",
           IEH_CLEAR_TRACK: "Index error clear track.",
           IEH_STOP_TRACK: "Index error stop track.",
           IEH_IGNORE: "Index error ignore.",
         )
);

create_enum!(
    SampleFormat,
    FFMS_SampleFormat,
    sample_format,
    (FMT_U8, FMT_S16, FMT_S32, FMT_FLT, FMT_DBL)
);

create_enum!(
    LogLevels,
    FFMS_LogLevels,
    log_levels,
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
    LogLevels,
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

pub struct Error {
    error: FFMS_ErrorInfo,
    buffer: [u8; 1024],
}

impl Default for Error {
    fn default() -> Self {
        let error = FFMS_ErrorInfo {
            ErrorType: 0,
            SubType: 0,
            Buffer: ptr::null_mut(),
            BufferSize: 0,
        };
        let mut ffms = Error {
            error,
            buffer: [0; 1024],
        };
        ffms.error.Buffer = ffms.buffer.as_mut_ptr() as *mut i8;
        ffms.error.BufferSize = mem::size_of_val(&ffms.buffer) as i32;
        ffms
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {}\nSubError: {}\n Cause: {}",
            Errors::from_i32(self.error.ErrorType),
            Errors::from_i32(self.error.SubType),
            str::from_utf8(&self.buffer).unwrap(),
        )
    }
}

impl Error {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut FFMS_ErrorInfo {
        &mut self.error
    }
}

pub struct Log;

impl Log {
    pub fn GetLogLevel() -> LogLevels {
        let log = unsafe { FFMS_GetLogLevel() };
        LogLevels::from_i32(log)
    }

    pub fn SetLogLevel(Level: LogLevels) {
        unsafe {
            FFMS_SetLogLevel(LogLevels::to_log_levels(&Level) as i32);
        }
    }
}

pub struct FFMS2;

impl FFMS2 {
    pub fn Init() {
        unsafe {
            FFMS_Init(0, 0);
        }
    }

    pub fn Version() -> usize {
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
