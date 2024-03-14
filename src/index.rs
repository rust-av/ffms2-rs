use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::panic;
use std::process;

use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;

use ffms2_sys::FFMS_IndexErrorHandling;

use crate::error::{Error, InternalError, Result};
use crate::track::TrackType;

/// Decision mode when a decoding error is encountered during indexing.
#[derive(Clone, Copy, Debug)]
pub enum IndexErrorHandling {
    /// Abort the indexing operation and raise an error.
    Abort,
    /// Clear all indexing entries for a track and return a blank track.
    ClearTrack,
    /// Stop the indexing operation, but keep all previous indexing entries.
    ///
    /// It returns a track stopped where the error had occurred.
    StopTrack,
    /// Ignore decoding error.
    Ignore,
}

impl IndexErrorHandling {
    const fn ffms2_index_error_handling(self) -> FFMS_IndexErrorHandling {
        match self {
            Self::Abort => FFMS_IndexErrorHandling::FFMS_IEH_ABORT,
            Self::ClearTrack => FFMS_IndexErrorHandling::FFMS_IEH_CLEAR_TRACK,
            Self::StopTrack => FFMS_IndexErrorHandling::FFMS_IEH_STOP_TRACK,
            Self::Ignore => FFMS_IndexErrorHandling::FFMS_IEH_IGNORE,
        }
    }

    const fn new(index_error_handling: FFMS_IndexErrorHandling) -> Self {
        match index_error_handling {
            FFMS_IndexErrorHandling::FFMS_IEH_ABORT => Self::Abort,
            FFMS_IndexErrorHandling::FFMS_IEH_CLEAR_TRACK => Self::ClearTrack,
            FFMS_IndexErrorHandling::FFMS_IEH_STOP_TRACK => Self::StopTrack,
            FFMS_IndexErrorHandling::FFMS_IEH_IGNORE => Self::Ignore,
        }
    }
}

impl fmt::Display for IndexErrorHandling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Abort => "Abort indexing operation.",
            Self::ClearTrack => {
                "Return a blank track because a decoding error occurred during indexing."
            }
            Self::StopTrack => {
                "Stop track indexing because a decoding error occurred."
            }
            Self::Ignore => "Ignore decoding error.",
        };
        s.fmt(f)
    }
}

pub struct Index(*mut ffms2_sys::FFMS_Index, Vec<u8>);

unsafe impl Send for Index {}

impl Index {
    /// Creates a new `[Index]` from the filepath passed as input.
    pub fn new(index_file: &Path) -> Result<Self> {
        let source =
            CString::new(index_file.to_str().ok_or(Error::StrConversion)?)?;
        let mut error = InternalError::new();
        let index = unsafe {
            ffms2_sys::FFMS_ReadIndex(source.as_ptr(), error.as_mut_ptr())
        };

        if index.is_null() {
            Err(error.into())
        } else {
            Ok(Index(index, Vec::new()))
        }
    }

    pub fn error_handling(&self) -> IndexErrorHandling {
        let index_error_handling =
            unsafe { ffms2_sys::FFMS_GetErrorHandling(self.0) };
        IndexErrorHandling::new(index_error_handling)
    }

    pub fn from_buffer(buffer: &[u8]) -> Result<Self> {
        let mut error = InternalError::new();
        let size = mem::size_of_val(buffer);
        let index = unsafe {
            ffms2_sys::FFMS_ReadIndexFromBuffer(
                buffer.as_ptr(),
                size,
                error.as_mut_ptr(),
            )
        };

        if index.is_null() {
            Err(error.into())
        } else {
            Ok(Index(index, Vec::new()))
        }
    }

    pub fn belongs_to_file(&self, source_file: &Path) -> Result<()> {
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_IndexBelongsToFile(
                self.0,
                source.as_ptr(),
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub fn write_to_file(&self, source_file: &Path) -> Result<()> {
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;

        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_WriteIndex(
                source.as_ptr(),
                self.0,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub fn write_to_buffer(&mut self) -> Result<&Vec<u8>> {
        let mut error = InternalError::new();
        let mut size = 0;
        let err = unsafe {
            ffms2_sys::FFMS_WriteIndexToBuffer(
                &mut self.1.as_mut_ptr(),
                &mut size,
                self.0,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(&self.1)
        }
    }

    pub fn first_track_of_type(&self, track_type: TrackType) -> Result<usize> {
        let mut error = InternalError::new();
        let num_tracks = unsafe {
            ffms2_sys::FFMS_GetFirstTrackOfType(
                self.0,
                TrackType::ffms2_track_type(track_type) as i32,
                error.as_mut_ptr(),
            )
        };
        if num_tracks < 0 {
            Err(error.into())
        } else {
            Ok(num_tracks as usize)
        }
    }

    pub fn first_indexed_track_of_type(
        &self,
        track_type: TrackType,
    ) -> Result<usize> {
        let mut error = InternalError::new();
        let num_tracks = unsafe {
            ffms2_sys::FFMS_GetFirstIndexedTrackOfType(
                self.0,
                TrackType::ffms2_track_type(track_type) as i32,
                error.as_mut_ptr(),
            )
        };
        if num_tracks < 0 {
            Err(error.into())
        } else {
            Ok(num_tracks as usize)
        }
    }

    /// Returns the number of indexed tracks.
    pub fn tracks_count(&self) -> usize {
        unsafe { ffms2_sys::FFMS_GetNumTracks(self.0) as usize }
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut ffms2_sys::FFMS_Index {
        self.0
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            if !self.1.is_empty() {
                ffms2_sys::FFMS_FreeIndexBuffer(&mut self.1.as_mut_ptr());
            }
            ffms2_sys::FFMS_DestroyIndex(self.0);
        }
    }
}

pub struct Indexer(*mut ffms2_sys::FFMS_Indexer);

unsafe impl Send for Indexer {}

impl Indexer {
    pub fn new(source_file: &Path) -> Result<Self> {
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;

        let mut error = InternalError::new();
        let indexer = unsafe {
            ffms2_sys::FFMS_CreateIndexer(source.as_ptr(), error.as_mut_ptr())
        };

        if indexer.is_null() {
            Err(error.into())
        } else {
            Ok(Indexer(indexer))
        }
    }

    pub fn with_demuxer_options(
        source_file: &Path,
        demuxer_options: HashMap<String, String>,
    ) -> Result<Self> {
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;
        let number_options = demuxer_options.len();

        let mut demuxer_options_cstring = Vec::new();
        for (key, value) in demuxer_options.into_iter() {
            demuxer_options_cstring
                .push((CString::new(key)?, CString::new(value)?));
        }

        let demuxer_keys_values = demuxer_options_cstring
            .iter()
            .map(|(key, value)| ffms2_sys::FFMS_KeyValuePair {
                Key: key.as_ptr(),
                Value: value.as_ptr(),
            })
            .collect::<Vec<ffms2_sys::FFMS_KeyValuePair>>();

        let mut error = InternalError::new();
        let indexer = unsafe {
            ffms2_sys::FFMS_CreateIndexer2(
                source.as_ptr(),
                demuxer_keys_values.as_ptr(),
                number_options as i32,
                error.as_mut_ptr(),
            )
        };

        if indexer.is_null() {
            Err(error.into())
        } else {
            Ok(Indexer(indexer))
        }
    }

    pub fn codec_name(&self, track: usize) -> Result<String> {
        let c_ptr =
            unsafe { ffms2_sys::FFMS_GetCodecNameI(self.0, track as i32) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        Ok(c_str.into_string()?)
    }

    pub fn format_name(&self) -> Result<String> {
        let c_ptr = unsafe { ffms2_sys::FFMS_GetFormatNameI(self.0) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        Ok(c_str.into_string()?)
    }

    pub fn tracks_count(&self) -> usize {
        unsafe { ffms2_sys::FFMS_GetNumTracksI(self.0) as usize }
    }

    pub fn track_type(&self, track: usize) -> TrackType {
        let track_type =
            unsafe { ffms2_sys::FFMS_GetTrackTypeI(self.0, track as i32) };
        TrackType::new(track_type)
    }

    /// Cancels the indexing process.
    pub fn cancel(&self) {
        unsafe {
            ffms2_sys::FFMS_CancelIndexing(self.0);
        }
    }

    pub fn do_indexing2(
        &self,
        error_handling: IndexErrorHandling,
    ) -> Result<Index> {
        let mut error = InternalError::new();
        let index = unsafe {
            ffms2_sys::FFMS_DoIndexing2(
                self.0,
                IndexErrorHandling::ffms2_index_error_handling(error_handling)
                    as i32,
                error.as_mut_ptr(),
            )
        };

        if index.is_null() {
            Err(error.into())
        } else {
            Ok(Index(index, Vec::new()))
        }
    }

    pub fn track_index_settings(&self, track: usize, index: usize) {
        unsafe {
            ffms2_sys::FFMS_TrackIndexSettings(
                self.0,
                track as i32,
                index as i32,
                0,
            );
        }
    }

    pub fn track_type_index_settings(
        &self,
        track_type: TrackType,
        index: usize,
    ) {
        unsafe {
            ffms2_sys::FFMS_TrackTypeIndexSettings(
                self.0,
                TrackType::ffms2_track_type(track_type) as i32,
                index as i32,
                0,
            );
        }
    }

    /// Sets a callback function for indexing progress updates.
    ///
    /// A progress callback is called regularly during indexing to report
    /// progress. It also offers the chance to interrupt indexing.
    ///
    /// Callback function's arguments are:
    /// - current: the current progress percentage
    /// - total: the maximum progress percentage
    /// - fixed percentage: an optional value which can be passed to the
    ///   function in order to modify the progress process.
    ///
    /// The indexing progress is usually calculated
    /// dividing `current` by `total`.
    pub fn progress_callback<F>(&self, callback: F, value: &mut usize)
    where
        F: FnMut(usize, usize, Option<&mut usize>) -> usize + 'static,
    {
        type Callback =
            dyn FnMut(usize, usize, Option<&mut usize>) -> usize + 'static;

        struct CallbackData<'a> {
            callback: Box<Callback>,
            value: &'a mut usize,
        }

        unsafe extern "C" fn index_callback(
            current: i64,
            total: i64,
            ic_private: *mut c_void,
        ) -> i32 {
            let mut user_data = Box::from_raw(ic_private as *mut CallbackData);

            let closure = panic::AssertUnwindSafe(|| {
                (user_data.callback)(
                    current as usize,
                    total as usize,
                    Some(user_data.value),
                ) as i32
            });

            let res = match panic::catch_unwind(closure) {
                Ok(res) => res,
                Err(_) => process::abort(),
            };

            Box::leak(user_data);

            res
        }

        let ic_private = Box::new(CallbackData {
            callback: Box::new(callback),
            value,
        });

        unsafe {
            ffms2_sys::FFMS_SetProgressCallback(
                self.0,
                Some(index_callback),
                Box::into_raw(ic_private) as *mut c_void,
            )
        }
    }
}
