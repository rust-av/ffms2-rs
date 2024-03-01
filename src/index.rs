use std::mem;
use std::panic;
use std::process;

use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;

use crate::error::{IndexErrorHandling, InternalError, Result};
use crate::track::TrackType;

pub struct Index {
    index: *mut ffms2_sys::FFMS_Index,
    buffer: Vec<u8>,
}

unsafe impl Send for Index {}

impl Index {
    pub fn new(IndexFile: &Path) -> Result<Self> {
        let source = CString::new(IndexFile.to_str().unwrap()).unwrap();
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

    pub fn ErrorHandling(&self) -> IndexErrorHandling {
        use ffms2_sys::FFMS_IndexErrorHandling;

        let index_error_handling =
            unsafe { ffms2_sys::FFMS_GetErrorHandling(self.index) };
        match index_error_handling {
            FFMS_IndexErrorHandling::FFMS_IEH_ABORT => {
                IndexErrorHandling::IEH_ABORT
            }
            FFMS_IndexErrorHandling::FFMS_IEH_CLEAR_TRACK => {
                IndexErrorHandling::IEH_CLEAR_TRACK
            }
            FFMS_IndexErrorHandling::FFMS_IEH_STOP_TRACK => {
                IndexErrorHandling::IEH_STOP_TRACK
            }
            FFMS_IndexErrorHandling::FFMS_IEH_IGNORE => {
                IndexErrorHandling::IEH_IGNORE
            }
        }
    }

    pub fn ReadIndexFromBuffer(Buffer: &[u8]) -> Result<Self> {
        let mut error = InternalError::new();
        let size = mem::size_of_val(Buffer);
        let index = unsafe {
            ffms2_sys::FFMS_ReadIndexFromBuffer(
                Buffer.as_ptr(),
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

    pub fn IndexBelongsToFile(&self, SourceFile: &Path) -> Result<()> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
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

    pub fn WriteIndex(&self, SourceFile: &Path) -> Result<()> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
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

    pub fn WriteIndexToBuffer(&mut self) -> Result<&Vec<u8>> {
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

    pub fn FirstTrackOfType(&self, TrackType: TrackType) -> Result<usize> {
        let mut error = InternalError::new();
        let num_tracks = unsafe {
            ffms2_sys::FFMS_GetFirstTrackOfType(
                self.index,
                TrackType::ffms2_track_type(TrackType) as i32,
                error.as_mut_ptr(),
            )
        };
        if num_tracks < 0 {
            Err(error.into())
        } else {
            Ok(num_tracks as usize)
        }
    }

    pub fn FirstIndexedTrackOfType(
        &self,
        TrackType: TrackType,
    ) -> Result<usize> {
        let mut error = InternalError::new();
        let num_tracks = unsafe {
            ffms2_sys::FFMS_GetFirstIndexedTrackOfType(
                self.index,
                TrackType::ffms2_track_type(TrackType) as i32,
                error.as_mut_ptr(),
            )
        };
        if num_tracks < 0 {
            Err(error.into())
        } else {
            Ok(num_tracks as usize)
        }
    }

    pub fn NumTracks(&self) -> usize {
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
    pub fn new(SourceFile: &Path) -> Result<Self> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
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

    pub fn CodecNameI(&self, Track: usize) -> String {
        let c_ptr = unsafe {
            ffms2_sys::FFMS_GetCodecNameI(self.indexer, Track as i32)
        };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        c_str.to_str().unwrap().to_owned()
    }

    pub fn FormatNameI(&self) -> String {
        let c_ptr = unsafe { ffms2_sys::FFMS_GetFormatNameI(self.indexer) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        c_str.to_str().unwrap().to_owned()
    }

    pub fn NumTracksI(&self) -> usize {
        unsafe { ffms2_sys::FFMS_GetNumTracksI(self.indexer) as usize }
    }

    pub fn TrackTypeI(&self, Track: usize) -> TrackType {
        let track_type = unsafe {
            ffms2_sys::FFMS_GetTrackTypeI(self.indexer, Track as i32)
        };
        TrackType::new(track_type)
    }

    pub fn CancelIndexing(&self) {
        unsafe {
            ffms2_sys::FFMS_CancelIndexing(self.indexer);
        }
    }

    pub fn DoIndexing2(
        &self,
        ErrorHandling: IndexErrorHandling,
    ) -> Result<Index> {
        let mut error = InternalError::new();
        let handling = IndexErrorHandling::to_idx_errors(ErrorHandling) as i32;
        let index = unsafe {
            ffms2_sys::FFMS_DoIndexing2(
                self.indexer,
                handling,
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

    pub fn TrackIndexSettings(&self, Track: usize, Index: usize) {
        unsafe {
            ffms2_sys::FFMS_TrackIndexSettings(
                self.indexer,
                Track as i32,
                Index as i32,
                0,
            );
        }
    }

    pub fn TrackTypeIndexSettings(&self, TrackType: TrackType, Index: usize) {
        unsafe {
            ffms2_sys::FFMS_TrackTypeIndexSettings(
                self.indexer,
                TrackType::ffms2_track_type(TrackType) as i32,
                Index as i32,
                0,
            );
        }
    }

    pub fn ProgressCallback<F>(&self, callback: F, value: &mut usize)
    where
        F: FnMut(usize, usize, Option<&mut usize>) -> usize + 'static,
    {
        type Callback =
            dyn FnMut(usize, usize, Option<&mut usize>) -> usize + 'static;

        struct CallbackData<'a> {
            callback: Box<Callback>,
            value: &'a mut usize,
        }

        unsafe extern "C" fn IndexCallback(
            Current: i64,
            Total: i64,
            ICPrivate: *mut c_void,
        ) -> i32 {
            let mut user_data = Box::from_raw(ICPrivate as *mut CallbackData);

            let closure = panic::AssertUnwindSafe(|| {
                (user_data.callback)(
                    Current as usize,
                    Total as usize,
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

        let ICPrivate = Box::new(CallbackData {
            callback: Box::new(callback),
            value,
        });

        unsafe {
            ffms2_sys::FFMS_SetProgressCallback(
                self.indexer,
                Some(IndexCallback),
                Box::into_raw(ICPrivate) as *mut c_void,
            )
        }
    }
}
