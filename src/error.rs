use std::mem;
use std::ptr;
use std::str;

use thiserror::Error;

use ffms2_sys::{FFMS_ErrorInfo, FFMS_Errors, FFMS_IndexErrorHandling};

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

pub(crate) struct InternalError {
    error: FFMS_ErrorInfo,
    buffer: [u8; 1024],
}

impl InternalError {
    pub(crate) fn new() -> Self {
        let error = FFMS_ErrorInfo {
            ErrorType: 0,
            SubType: 0,
            Buffer: ptr::null_mut(),
            BufferSize: 0,
        };
        let mut ffms = Self {
            error,
            buffer: [0; 1024],
        };
        ffms.error.Buffer = ffms.buffer.as_mut_ptr() as *mut i8;
        ffms.error.BufferSize = mem::size_of_val(&ffms.buffer) as i32;
        ffms
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut FFMS_ErrorInfo {
        &mut self.error
    }
}

/// FFMS2 error types
#[derive(Debug, Error)]
pub enum Error {
    /// An FFMS2 API error.
    #[error("FFSM2 API error")]
    FFMS2(String),
    /// Failure in retrieving the track.
    #[error("Impossible to retrieve the track")]
    Track,
    /// Failure in getting frames.
    #[error("Impossible to get frames")]
    Frames,
}

impl From<InternalError> for Error {
    fn from(internal_error: InternalError) -> Self {
        let error = format!(
            "Error: {}\nSubError: {}\n Cause: {}",
            Errors::from_i32(internal_error.error.ErrorType),
            Errors::from_i32(internal_error.error.SubType),
            str::from_utf8(&internal_error.buffer).unwrap_or("Unknown error")
        );
        Self::FFMS2(error)
    }
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
