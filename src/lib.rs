#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod audio;
pub mod frame;
pub mod video;
pub mod resample;
pub mod track;

mod utility;

use ffms2_sys::*;

use std::fmt;

errors!(Error, FFMS_Errors, ffms_errors,
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
            ERROR_USER: "Error.",
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

pub struct FFMSError {
    error: FFMS_ErrorInfo,
}

impl FFMSError {
    pub fn new(error_type: i32, sub_type: i32, buffer: &Vec<u8>) -> Self {
        let error = FFMS_ErrorInfo {
            ErrorType: error_type,
            SubType: sub_type,
            BufferSize: buffer.len() as i32,
            Buffer: buffer.as_ptr() as *mut i8,
        };
        FFMSError { error }
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
