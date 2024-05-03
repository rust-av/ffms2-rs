use crate::track::*;
use crate::*;

use ffms2_sys::*;

use std::default::Default;
use std::ffi::c_char;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::panic;
use std::path::Path;
use std::process;

pub struct Index {
    index: *mut FFMS_Index,
    buffer: Vec<u8>,
}

unsafe impl Send for Index {}

impl Index {
    pub fn new(IndexFile: &Path) -> Result<Self, Error> {
        let source = CString::new(IndexFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let index =
            unsafe { FFMS_ReadIndex(source.as_ptr(), error.as_mut_ptr()) };

        if index.is_null() {
            Err(error)
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn ErrorHandling(&self) -> IndexErrorHandling {
        let index_error_handling =
            unsafe { FFMS_GetErrorHandling(self.index) };
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

    pub fn ReadIndexFromBuffer(Buffer: &[u8]) -> Result<Self, Error> {
        let mut error: Error = Default::default();
        let size = mem::size_of_val(Buffer);
        let index = unsafe {
            FFMS_ReadIndexFromBuffer(Buffer.as_ptr(), size, error.as_mut_ptr())
        };

        if index.is_null() {
            Err(error)
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn IndexBelongsToFile(&self, SourceFile: &Path) -> Result<(), Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let err = unsafe {
            FFMS_IndexBelongsToFile(
                self.index,
                source.as_ptr(),
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn WriteIndex(&self, SourceFile: &Path) -> Result<(), Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let err = unsafe {
            FFMS_WriteIndex(source.as_ptr(), self.index, error.as_mut_ptr())
        };

        if err != 0 {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn WriteIndexToBuffer(&mut self) -> Result<&Vec<u8>, Error> {
        let mut error: Error = Default::default();
        let mut size = 0;
        let err = unsafe {
            FFMS_WriteIndexToBuffer(
                &mut self.buffer.as_mut_ptr(),
                &mut size,
                self.index,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error)
        } else {
            Ok(&self.buffer)
        }
    }

    pub fn FirstTrackOfType(
        &self,
        TrackType: TrackType,
    ) -> Result<usize, Error> {
        let mut error: Error = Default::default();
        let num_tracks = unsafe {
            FFMS_GetFirstTrackOfType(
                self.index,
                TrackType::to_track_type(TrackType) as i32,
                error.as_mut_ptr(),
            )
        };
        if num_tracks < 0 {
            Err(error)
        } else {
            Ok(num_tracks as usize)
        }
    }

    pub fn FirstIndexedTrackOfType(
        &self,
        TrackType: TrackType,
    ) -> Result<usize, Error> {
        let mut error: Error = Default::default();
        let num_tracks = unsafe {
            FFMS_GetFirstIndexedTrackOfType(
                self.index,
                TrackType::to_track_type(TrackType) as i32,
                error.as_mut_ptr(),
            )
        };
        if num_tracks < 0 {
            Err(error)
        } else {
            Ok(num_tracks as usize)
        }
    }

    pub fn NumTracks(&self) -> usize {
        unsafe { FFMS_GetNumTracks(self.index) as usize }
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut FFMS_Index {
        self.index
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            if !self.buffer.is_empty() {
                FFMS_FreeIndexBuffer(&mut self.buffer.as_mut_ptr());
            }
            FFMS_DestroyIndex(self.index);
        }
    }
}

pub struct Indexer {
    indexer: *mut FFMS_Indexer,
}

unsafe impl Send for Indexer {}

impl Indexer {
    pub fn new(SourceFile: &Path) -> Result<Self, Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let indexer =
            unsafe { FFMS_CreateIndexer(source.as_ptr(), error.as_mut_ptr()) };

        if indexer.is_null() {
            Err(error)
        } else {
            Ok(Indexer { indexer })
        }
    }

    pub fn CodecNameI(&self, Track: usize) -> String {
        let c_ptr = unsafe { FFMS_GetCodecNameI(self.indexer, Track as i32) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut c_char) };
        c_str.to_str().unwrap().to_owned()
    }

    pub fn FormatNameI(&self) -> String {
        let c_ptr = unsafe { FFMS_GetFormatNameI(self.indexer) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut c_char) };
        c_str.to_str().unwrap().to_owned()
    }

    pub fn NumTracksI(&self) -> usize {
        unsafe { FFMS_GetNumTracksI(self.indexer) as usize }
    }

    pub fn TrackTypeI(&self, Track: usize) -> TrackType {
        let track_type =
            unsafe { FFMS_GetTrackTypeI(self.indexer, Track as i32) };
        TrackType::from_i32(track_type)
    }

    pub fn CancelIndexing(&self) {
        unsafe {
            FFMS_CancelIndexing(self.indexer);
        }
    }

    pub fn DoIndexing2(
        &self,
        ErrorHandling: IndexErrorHandling,
    ) -> Result<Index, Error> {
        let mut error: Error = Default::default();
        let handling = IndexErrorHandling::to_idx_errors(ErrorHandling) as i32;
        let index = unsafe {
            FFMS_DoIndexing2(self.indexer, handling, error.as_mut_ptr())
        };

        if index.is_null() {
            Err(error)
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn TrackIndexSettings(&self, Track: usize, Index: usize) {
        unsafe {
            FFMS_TrackIndexSettings(
                self.indexer,
                Track as i32,
                Index as i32,
                0,
            );
        }
    }

    pub fn TrackTypeIndexSettings(&self, TrackType: TrackType, Index: usize) {
        unsafe {
            FFMS_TrackTypeIndexSettings(
                self.indexer,
                TrackType::to_track_type(TrackType) as i32,
                Index as i32,
                0,
            );
        }
    }

    pub fn ProgressCallback<F>(&self, callback: F, value: &mut usize)
    where
        F: FnMut(usize, usize, Option<&mut usize>) -> usize + 'static,
    {
        struct CallbackData<'a> {
            callback: Box<
                dyn FnMut(usize, usize, Option<&mut usize>) -> usize + 'static,
            >,
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
            FFMS_SetProgressCallback(
                self.indexer,
                Some(IndexCallback),
                Box::into_raw(ICPrivate) as *mut c_void,
            )
        }
    }
}
