use crate::*;
use ffms2_sys::*;

use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::panic;
use std::path::PathBuf;
use std::process;

/* External Function example */
/*pub fn UpdateProgress<T>(Current: usize, Total: usize, ICPrivate: T) -> usize {
    println!("Indexing, please wait...");
    0
}*/

pub struct Index {
    index: *mut FFMS_Index,
    buffer: Vec<u8>,
}

impl Index {
    pub fn new(IndexFile: &PathBuf) -> Result<Self, Error> {
        let source = CString::new(IndexFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let index = unsafe { FFMS_ReadIndex(source.as_ptr(), error.as_mut_ptr()) };

        if index.is_null() {
            Err(error)
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn ReadIndexFromBuffer(Buffer: &Vec<u8>) -> Result<Self, Error> {
        let mut error: Error = Default::default();
        let size = mem::size_of_val(Buffer.as_slice());
        let index = unsafe { FFMS_ReadIndexFromBuffer(Buffer.as_ptr(), size, error.as_mut_ptr()) };

        if index.is_null() {
            Err(error)
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    pub fn IndexBelongsToFile(&self, SourceFile: &PathBuf) -> Result<(), Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let err =
            unsafe { FFMS_IndexBelongsToFile(self.index, source.as_ptr(), error.as_mut_ptr()) };

        if err != 0 {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn WriteIndex(&self, SourceFile: &PathBuf) -> Result<(), Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let err = unsafe { FFMS_WriteIndex(source.as_ptr(), self.index, error.as_mut_ptr()) };

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
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            FFMS_FreeIndexBuffer(&mut self.buffer.as_mut_ptr());
            FFMS_DestroyIndex(self.index);
        }
    }
}

pub struct Indexer {
    indexer: *mut FFMS_Indexer,
}

impl Indexer {
    pub fn new(SourceFile: &PathBuf) -> Result<Self, Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let indexer = unsafe { FFMS_CreateIndexer(source.as_ptr(), error.as_mut_ptr()) };

        if indexer.is_null() {
            Err(error)
        } else {
            Ok(Indexer { indexer })
        }
    }

    #[cfg(feature = "ffms2-2-21-0")]
    pub fn DoIndexing2(&self, ErrorHandling: usize) -> Result<Index, Error> {
        let mut error: Error = Default::default();
        let index =
            unsafe { FFMS_DoIndexing2(self.indexer, ErrorHandling as i32, error.as_mut_ptr()) };

        if index.is_null() {
            Err(error)
        } else {
            Ok(Index {
                index,
                buffer: Vec::new(),
            })
        }
    }

    #[cfg(feature = "ffms2-2-21-0")]
    pub fn TrackIndexSettings(&self, Track: usize, Index: usize) {
        unsafe {
            FFMS_TrackIndexSettings(self.indexer, Track as i32, Index as i32, 0);
        }
    }

    #[cfg(feature = "ffms2-2-21-0")]
    pub fn TrackTypeIndexSettings(&self, TrackType: usize, Index: usize) {
        unsafe {
            FFMS_TrackTypeIndexSettings(self.indexer, TrackType as i32, Index as i32, 0);
        }
    }

    #[cfg(feature = "ffms2-2-21-0")]
    pub fn set_ProgressCallback<F, T>(&self, callback: F, value: T)
    where
        F: FnMut(usize, usize, T) -> usize + 'static,
    {
        struct CallbackData<T> {
            callback: Box<FnMut(usize, usize, T) -> usize + 'static>,
            value: T,
        }

        unsafe extern "C" fn IndexCallback<T>(
            Current: i64,
            Total: i64,
            ICPrivate: *mut c_void,
        ) -> i32 {
            let mut user_data = Box::from_raw(ICPrivate as *mut CallbackData<T>);

            let closure = panic::AssertUnwindSafe(|| {
                (user_data.callback)(Current as usize, Total as usize, user_data.value) as i32
            });

            let res = panic::catch_unwind(closure);

            if res.is_err() {
                process::abort();
            }

            mem::forget(ICPrivate);
            res.unwrap()
        }

        let ICPrivate = Box::new(CallbackData {
            callback: Box::new(callback),
            value: value,
        });

        unsafe {
            FFMS_SetProgressCallback(
                self.indexer,
                Some(IndexCallback::<T>),
                Box::into_raw(ICPrivate) as *mut c_void,
            )
        }
    }
}

impl Drop for Indexer {
    fn drop(&mut self) {
        unsafe {
            FFMS_CancelIndexing(self.indexer);
        }
    }
}
