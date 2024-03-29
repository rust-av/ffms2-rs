use std::borrow;
use std::mem;
use std::ptr;
use std::str;

use thiserror::Error;

use ffms2_sys::{FFMS_ErrorInfo, FFMS_Errors::*};

// Main ffms2 error types.
const fn ffms2_main_types_error(error: i32) -> &'static str {
    match error {
        // No error.
        e if e == FFMS_ERROR_SUCCESS as i32 => "Success.",

        // Index file handling.
        e if e == FFMS_ERROR_INDEX as i32 => "Index File Handling.",
        // Indexing.
        e if e == FFMS_ERROR_INDEXING as i32 => "Indexing.",
        // Video post-processing (libpostproc).
        e if e == FFMS_ERROR_POSTPROCESSING as i32 => "Video Post-Processing.",
        // Image scaling (libswscale).
        e if e == FFMS_ERROR_SCALING as i32 => "Image Scaling.",
        // Audio/Video decoding.
        e if e == FFMS_ERROR_DECODING as i32 => "Audio/Video Decoding.",
        // Seeking.
        e if e == FFMS_ERROR_SEEKING as i32 => "Seeking.",
        // File parsing.
        e if e == FFMS_ERROR_PARSER as i32 => "File Parsing.",
        // Track handling.
        e if e == FFMS_ERROR_TRACK as i32 => "Track Handling.",
        // WAVE64 file writer.
        e if e == FFMS_ERROR_WAVE_WRITER as i32 => "Wave writer.",
        // Operation aborted.
        e if e == FFMS_ERROR_CANCELLED as i32 => "Operation Aborted.",
        // Audio resampling.
        e if e == FFMS_ERROR_RESAMPLING as i32 => "Audio Resampling.",
        _ => "Unknown.",
    }
}

// ffms2 subtypes error.
const fn ffms2_subtypes_error(error: i32) -> &'static str {
    match error {
        // Unknown error.
        e if e == FFMS_ERROR_UNKNOWN as i32 => "Unknown.",
        // Format or operation is not supported with this binary.
        e if e == FFMS_ERROR_UNSUPPORTED as i32 => {
            "Format or Operation Unsupported."
        }
        // Cannot read from file.
        e if e == FFMS_ERROR_FILE_READ as i32 => "Impossible to read file.",
        // Cannot write to file.
        e if e == FFMS_ERROR_FILE_WRITE as i32 => "Impossible to write file.",
        // No such file or directory.
        e if e == FFMS_ERROR_NO_FILE as i32 => "No file.",
        // Wrong version.
        e if e == FFMS_ERROR_VERSION as i32 => "Wrong Version.",
        // Out of memory.
        e if e == FFMS_ERROR_ALLOCATION_FAILED as i32 => {
            "Allocation failed, Out of Memory."
        }
        // Invalid argument.
        e if e == FFMS_ERROR_INVALID_ARGUMENT as i32 => "Invalid argument.",
        // Decoder error.
        e if e == FFMS_ERROR_CODEC as i32 => "Failed Decoder Operation.",
        // Requested mode or operation unavailable in this binary.
        e if e == FFMS_ERROR_NOT_AVAILABLE as i32 => {
            "Not available Mode or Operation."
        }
        // Provided index does not match the file
        e if e == FFMS_ERROR_FILE_MISMATCH as i32 => "Index Mismatch File.",
        // Error caused by a user.
        e if e == FFMS_ERROR_USER as i32 => "User Error.",
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

/// All error types.
#[derive(Debug, Error)]
pub enum Error {
    /// A `ffms2` API error.
    #[error("ffms2 API error.")]
    FFMS2(borrow::Cow<'static, str>),
    /// I/O error.
    #[error("Failed to perform an I/O operation.")]
    InputOutput(#[from] std::io::Error),
    /// The path is not a file.
    #[error("The path is not a file.")]
    NotAFile,
    /// Failure in retrieving the track.
    #[error("Failed to get the track.")]
    Track,
    /// Wrong track number.
    #[error("Wrong track number.")]
    WrongTrack,
    /// Unknown track type.
    #[error("Unknown track type.")]
    UnknownTrackType,
    /// Wrong audio sample range.
    #[error("Wrong audio sample range.")]
    WrongSampleRange,
    /// Unknown audio sample format.
    #[error("Unknown audio sample format.")]
    UknownSampleFormat,
    /// Unknown audio channel.
    #[error("Unknown audio channel.")]
    UknownAudioChannel,
    /// Failure in getting frames.
    #[error("Failed to get frames.")]
    Frames,
    /// Wrong frame number.
    #[error("Wrong frame number.")]
    WrongFrame,
    /// Wrong timestamp.
    #[error("Wrong timestamp.")]
    WrongTimestamp,
    /// Failure in converting an operating system string into a [`&str`].
    #[error("str convervion error.")]
    StrConversion,
    /// Failure in creating a [`CString`](std::ffi::CString).
    #[error("Failed to create a CString")]
    CString(#[from] std::ffi::NulError),
    /// Failure in converting a [`CString`](std::ffi::CString) into a [`String`].
    #[error("Failed to convert a CString into a String")]
    StringConversion(#[from] std::ffi::IntoStringError),
}

impl From<InternalError> for Error {
    fn from(internal_error: InternalError) -> Self {
        let error = format!(
            "Error: {}\nSubError: {}\n Cause: {}",
            ffms2_main_types_error(internal_error.error.ErrorType),
            ffms2_subtypes_error(internal_error.error.SubType),
            str::from_utf8(&internal_error.buffer).unwrap_or("Unknown error")
        );
        Self::FFMS2(borrow::Cow::Owned(error))
    }
}

/// A specialized [`Result`] type.
pub type Result<T> = ::std::result::Result<T, Error>;
