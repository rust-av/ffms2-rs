#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod audio;
pub mod frame;
pub mod video;
pub mod resample;
pub mod index;
pub mod track;

mod utility;

use ffms2_sys::*;

use std::fmt;
use std::str;
use std::mem;
use std::ptr;

errors!(Errors, FFMS_Errors, ffms_errors,
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

errors!(IndexErrorHandling, FFMS_IndexErrorHandling, ffms_idx_errors,
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
        let mut ffms = Error{ error, buffer: [0; 1024] };
        ffms.error.Buffer = ffms.buffer.as_mut_ptr() as *mut i8;
        ffms.error.BufferSize = mem::size_of_val(&ffms.buffer) as i32;
        ffms
    }
}

impl fmt::Display for Error {
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
    pub fn GetLogLevel() -> i32 {
        unsafe { FFMS_GetLogLevel() }
    }

    pub fn SetLogLevel(Level: i32) {
        unsafe { FFMS_SetLogLevel(Level); }
    }
}

pub struct FFMS2;

impl FFMS2 {
    pub fn new() {
        unsafe{ FFMS_Init(0, 0); }
    }

    pub fn GetVersion() -> usize {
        unsafe { FFMS_GetVersion() as usize }
    }
}

impl Drop for FFMS2 {
    fn drop(&mut self) {
        unsafe{ FFMS_Deinit(); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ffms_errors() {
        assert_eq!(2 + 2, 4);
    }
}
