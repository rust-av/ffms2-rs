use std::mem;
use std::ptr;
use std::str;

use thiserror::Error;

use ffms2_sys::{FFMS_ErrorInfo, FFMS_Errors};

const fn ffms2_error_to_str(error: i32) -> &'static str {
    match error {
        e if e == FFMS_Errors::FFMS_ERROR_SUCCESS as i32 => "Success.",
        e if e == FFMS_Errors::FFMS_ERROR_INDEX as i32 => "Index.",
        e if e == FFMS_Errors::FFMS_ERROR_INDEXING as i32 => "Indexing.",
        e if e == FFMS_Errors::FFMS_ERROR_POSTPROCESSING as i32 => {
            "Post-processing."
        }
        e if e == FFMS_Errors::FFMS_ERROR_SCALING as i32 => "Scaling.",
        e if e == FFMS_Errors::FFMS_ERROR_DECODING as i32 => "Decoding.",
        e if e == FFMS_Errors::FFMS_ERROR_SEEKING as i32 => "Seeking.",
        e if e == FFMS_Errors::FFMS_ERROR_PARSER as i32 => "Parser.",
        e if e == FFMS_Errors::FFMS_ERROR_TRACK as i32 => "Track.",
        e if e == FFMS_Errors::FFMS_ERROR_WAVE_WRITER as i32 => "Wave writer.",
        e if e == FFMS_Errors::FFMS_ERROR_CANCELLED as i32 => "Cancelled.",
        e if e == FFMS_Errors::FFMS_ERROR_RESAMPLING as i32 => "Resampling.",
        e if e == FFMS_Errors::FFMS_ERROR_UNKNOWN as i32 => "Unknown.",
        e if e == FFMS_Errors::FFMS_ERROR_UNSUPPORTED as i32 => "Unsupported.",
        e if e == FFMS_Errors::FFMS_ERROR_FILE_READ as i32 => "File read.",
        e if e == FFMS_Errors::FFMS_ERROR_FILE_WRITE as i32 => "File write.",
        e if e == FFMS_Errors::FFMS_ERROR_NO_FILE as i32 => "No file.",
        e if e == FFMS_Errors::FFMS_ERROR_VERSION as i32 => "Version.",
        e if e == FFMS_Errors::FFMS_ERROR_ALLOCATION_FAILED as i32 => {
            "Allocation failed."
        }
        e if e == FFMS_Errors::FFMS_ERROR_INVALID_ARGUMENT as i32 => {
            "Invalid argument."
        }
        e if e == FFMS_Errors::FFMS_ERROR_CODEC as i32 => "Codec error.",
        e if e == FFMS_Errors::FFMS_ERROR_NOT_AVAILABLE as i32 => {
            "Not available."
        }
        e if e == FFMS_Errors::FFMS_ERROR_FILE_MISMATCH as i32 => {
            "File mismatch."
        }
        e if e == FFMS_Errors::FFMS_ERROR_USER as i32 => "Caused by the user.",

        _ => "Unknown.",
    }
}

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
            ffms2_error_to_str(internal_error.error.ErrorType),
            ffms2_error_to_str(internal_error.error.SubType),
            str::from_utf8(&internal_error.buffer).unwrap_or("Unknown error")
        );
        Self::FFMS2(error)
    }
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
