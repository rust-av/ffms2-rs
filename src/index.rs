use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::panic;
use std::process;

use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;

use ffms2_sys::FFMS_IndexErrorHandling;

use crate::error::{InternalError, Result};
use crate::track::TrackType;

#[derive(Clone, Copy, Debug)]
pub enum IndexErrorHandling {
    Abort,
    ClearTrack,
    StopTrack,
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
            Self::Abort => "Index error aborting.",
            Self::ClearTrack => "Index error clear track.",
            Self::StopTrack => "Index error stop track.",
            Self::Ignore => "Index error ignore.",
        };
        s.fmt(f)
    }
}

pub struct Index {
    index: *mut ffms2_sys::FFMS_Index,
    buffer: Vec<u8>,
}

unsafe impl Send for Index {}

impl Index {
    pub fn new(index_file: &Path) -> Result<Self> {
        let source = CString::new(index_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let index = unsafe {
            ffms2_sys::FFMS_ReadIndex(source.as_ptr(), error.as_mut_ptr())
        };

        if index.is_null() {
            Err(error.into())
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn error_handling(&self) -> IndexErrorHandling {
        let index_error_handling =
            unsafe { ffms2_sys::FFMS_GetErrorHandling(self.index) };
        IndexErrorHandling::new(index_error_handling)
    }

    pub fn read_index_from_buffer(buffer: &[u8]) -> Result<Self> {
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
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn index_belongs_to_file(&self, source_file: &Path) -> Result<()> {
        let source = CString::new(source_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_IndexBelongsToFile(
                self.index,
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

    pub fn write_index(&self, source_file: &Path) -> Result<()> {
        let source = CString::new(source_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_WriteIndex(
                source.as_ptr(),
                self.index,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub fn write_index_to_buffer(&mut self) -> Result<&Vec<u8>> {
        let mut error = InternalError::new();
        let mut size = 0;
        let err = unsafe {
            ffms2_sys::FFMS_WriteIndexToBuffer(
                &mut self.buffer.as_mut_ptr(),
                &mut size,
                self.index,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(&self.buffer)
        }
    }

    pub fn first_track_of_type(&self, track_type: TrackType) -> Result<usize> {
        let mut error = InternalError::new();
        let num_tracks = unsafe {
            ffms2_sys::FFMS_GetFirstTrackOfType(
                self.index,
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
                self.index,
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

    pub fn tracks_count(&self) -> usize {
        unsafe { ffms2_sys::FFMS_GetNumTracks(self.index) as usize }
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut ffms2_sys::FFMS_Index {
        self.index
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            if !self.buffer.is_empty() {
                ffms2_sys::FFMS_FreeIndexBuffer(&mut self.buffer.as_mut_ptr());
            }
            ffms2_sys::FFMS_DestroyIndex(self.index);
        }
    }
}

pub struct Indexer {
    indexer: *mut ffms2_sys::FFMS_Indexer,
}

unsafe impl Send for Indexer {}

impl Indexer {
    pub fn new(source_file: &Path) -> Result<Self> {
        let source = CString::new(source_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let indexer = unsafe {
            ffms2_sys::FFMS_CreateIndexer(source.as_ptr(), error.as_mut_ptr())
        };

        if indexer.is_null() {
            Err(error.into())
        } else {
            Ok(Indexer { indexer })
        }
    }

    pub fn create_indexer_2(
        source_file: &Path,
        demuxer_options: HashMap<String, String>,
    ) -> Result<Self> {
        let source = CString::new(source_file.to_str().unwrap()).unwrap();
        let number_options = demuxer_options.len();
        let demuxer_options_cstring = demuxer_options
            .into_iter()
            .map(|(key, value)| {
                (CString::new(key).unwrap(), CString::new(value).unwrap())
            })
            .collect::<Vec<(CString, CString)>>();

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
            Ok(Indexer { indexer })
        }
    }

    pub fn codec_name(&self, track: usize) -> String {
        let c_ptr = unsafe {
            ffms2_sys::FFMS_GetCodecNameI(self.indexer, track as i32)
        };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        c_str.to_str().unwrap().to_owned()
    }

    pub fn format_name(&self) -> String {
        let c_ptr = unsafe { ffms2_sys::FFMS_GetFormatNameI(self.indexer) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        c_str.to_str().unwrap().to_owned()
    }

    pub fn tracks_count(&self) -> usize {
        unsafe { ffms2_sys::FFMS_GetNumTracksI(self.indexer) as usize }
    }

    pub fn track_type(&self, track: usize) -> TrackType {
        let track_type = unsafe {
            ffms2_sys::FFMS_GetTrackTypeI(self.indexer, track as i32)
        };
        TrackType::new(track_type)
    }

    pub fn cancel_indexing(&self) {
        unsafe {
            ffms2_sys::FFMS_CancelIndexing(self.indexer);
        }
    }

    pub fn do_indexing2(
        &self,
        error_handling: IndexErrorHandling,
    ) -> Result<Index> {
        let mut error = InternalError::new();
        let index = unsafe {
            ffms2_sys::FFMS_DoIndexing2(
                self.indexer,
                IndexErrorHandling::ffms2_index_error_handling(error_handling)
                    as i32,
                error.as_mut_ptr(),
            )
        };

        if index.is_null() {
            Err(error.into())
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn track_index_settings(&self, track: usize, index: usize) {
        unsafe {
            ffms2_sys::FFMS_TrackIndexSettings(
                self.indexer,
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
                self.indexer,
                TrackType::ffms2_track_type(track_type) as i32,
                index as i32,
                0,
            );
        }
    }

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

            let res = panic::catch_unwind(closure);

            if res.is_err() {
                process::abort();
            }

            Box::leak(user_data);

            res.unwrap()
        }

        let ic_private = Box::new(CallbackData {
            callback: Box::new(callback),
            value,
        });

        unsafe {
            ffms2_sys::FFMS_SetProgressCallback(
                self.indexer,
                Some(index_callback),
                Box::into_raw(ic_private) as *mut c_void,
            )
        }
    }
}
