use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::panic;
use std::process;

use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;

use ffms2_sys::{FFMS_Index, FFMS_IndexErrorHandling, FFMS_Indexer};

use crate::error::{Error, InternalError, Result};
use crate::track::TrackType;

/// Decision method adopted when a decoding error occurs during
/// a media file indexing.
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

/// An indexing information manager.
///
/// It provides a series of methods to interact with an index file.
/// Among the functionalities:
/// - Reading and writing the content of the file
/// - Extract indexing information starting from tracks given as input.
pub struct Index(*mut ffms2_sys::FFMS_Index, Vec<u8>, usize);

unsafe impl Send for Index {}

impl Index {
    /// Creates a new [`Index`] instance from a given index file saved on disk,
    /// which can be an absolute or a relative path.
    ///
    /// Note that index files can only be read by the exact `ffms2` version
    /// as they were written with. Attempting to open an index file written with
    /// a different `ffms2` version produces an index mismatch error.
    pub fn new(index_file: &Path) -> Result<Self> {
        if index_file.is_file() {
            return Err(Error::NotAFile);
        }

        let source =
            CString::new(index_file.to_str().ok_or(Error::StrConversion)?)?;
        let mut error = InternalError::new();
        let index = unsafe {
            ffms2_sys::FFMS_ReadIndex(source.as_ptr(), error.as_mut_ptr())
        };

        if index.is_null() {
            Err(error.into())
        } else {
            Ok(Self::create_index(index))
        }
    }

    /// Returns the index error passed as input to the
    /// [`do_indexing`](Indexer::do_indexing) function.
    ///
    /// This kind of error might occur during the construction of an index
    /// file made by an [`Indexer`].
    pub fn indexer_error(&self) -> IndexErrorHandling {
        let index_error_handling =
            unsafe { ffms2_sys::FFMS_GetErrorHandling(self.0) };
        IndexErrorHandling::new(index_error_handling)
    }

    /// Creates a new [`Index`] instance from a memory bytes buffer.
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
            Ok(Self::create_index(index))
        }
    }

    /// Makes an heuristic, quite reliable, to guess whether an index belongs
    /// to a given source file.
    ///
    /// This method is useful to determine if the index object is actually
    /// relevant, since the only two ways to pair up an index file to a source
    /// file are:
    ///
    /// - Trust whoever provided the two files
    /// - Comparing the two filenames, which is usually a not very reliable
    ///   operation.
    pub fn belongs_to_file(&self, source_file: &Path) -> Result<bool> {
        if source_file.is_file() {
            return Err(Error::NotAFile);
        }

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
            Ok(true)
        }
    }

    /// Writes the indexing information to a given index file, which can
    /// be an absolute or a relative path.
    ///
    /// If the index file already exists, it will be truncated and overwritten.
    ///
    /// Saving indexing information in a file avoids re-indexing
    /// a media file every time. This operation can be particularly time-saving
    /// with very large files or with those files containing a lot of
    /// audio tracks, since both of them can take quite a lot to index.
    pub fn write_to_file(&self, source_file: &Path) -> Result<()> {
        if source_file.is_file() {
            return Err(Error::NotAFile);
        }

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

    /// Writes the indexing information to a memory bytes buffer.
    pub fn write_to_buffer(&mut self) -> Result<&[u8]> {
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

    /// Finds the first track of the type passed as input and
    /// returns its track number.
    pub fn first_track_type(&self, track_type: TrackType) -> Result<usize> {
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

    /// Finds the first indexed track of the type passed as input and
    /// returns its track number.
    ///
    /// This method ignores the tracks which have not been indexed.
    pub fn first_indexed_track_type(
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

    /// Returns the number of tracks contained in a given index file.
    #[inline(always)]
    pub fn tracks_count(&self) -> usize {
        self.2
    }

    #[inline(always)]
    pub(crate) fn as_mut_ptr(&self) -> *mut FFMS_Index {
        self.0
    }

    #[inline(always)]
    fn create_index(index: *mut FFMS_Index) -> Self {
        let track_count =
            unsafe { ffms2_sys::FFMS_GetNumTracks(index) as usize };
        Self(index, Vec::new(), track_count)
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

/// An indexer manager.
///
/// Before opening a media file, it is **necessary** to index its content.
///
/// An indexer ensures that key frame positions, timecode data, and
/// other information are known in advance, so that a frame-accurate seeking
/// can be done in an easy way.
///
/// An indexer also provides a series of methods to query for information about
/// a media file.
pub struct Indexer(*mut ffms2_sys::FFMS_Indexer, usize);

unsafe impl Send for Indexer {}

impl Indexer {
    /// Creates a new [`Indexer`] from a given media file, which can be an
    /// absolute or a relative path.
    ///
    /// By default, all video tracks present in a media file are indexed,
    /// while all audio tracks are not.
    pub fn new(source_file: &Path) -> Result<Self> {
        if source_file.is_file() {
            return Err(Error::NotAFile);
        }

        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;

        let mut error = InternalError::new();
        let indexer = unsafe {
            ffms2_sys::FFMS_CreateIndexer(source.as_ptr(), error.as_mut_ptr())
        };

        if indexer.is_null() {
            Err(error.into())
        } else {
            Ok(Self::create_indexer(indexer))
        }
    }

    /// Creates a new [`Indexer`] from a given media file. The indexing process
    /// can be controlled through the demuxing options described as a
    /// key-value pairs.
    ///
    /// The media file path can be absolute or relative.
    pub fn with_demuxer_options(
        source_file: &Path,
        demuxer_options: HashMap<String, String>,
    ) -> Result<Self> {
        if source_file.is_file() {
            return Err(Error::NotAFile);
        }

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
            Ok(Self::create_indexer(indexer))
        }
    }

    /// Returns the human-readable codec name of the given track number
    /// contained in the indexed media file.
    ///
    /// Returns an error when a wrong track number is passed.
    pub fn codec_name(&self, track: usize) -> Result<String> {
        if track > self.tracks_count() - 1 {
            return Err(Error::WrongTrack);
        }

        let c_ptr =
            unsafe { ffms2_sys::FFMS_GetCodecNameI(self.0, track as i32) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        Ok(c_str.into_string()?)
    }

    /// Returns the human-readable container format name contained in the
    /// indexed media file.
    pub fn format_name(&self) -> Result<String> {
        let c_ptr = unsafe { ffms2_sys::FFMS_GetFormatNameI(self.0) };
        let c_str = unsafe { CString::from_raw(c_ptr as *mut i8) };
        Ok(c_str.into_string()?)
    }

    /// Returns the total number of tracks contained in the indexed media file.
    ///
    /// Differently from [`Index.tracks_count`](Index::tracks_count),
    /// it does not require indexing the entire media file first.
    pub fn tracks_count(&self) -> usize {
        self.1
    }

    /// Returns the track type associated with the input track number.
    ///
    /// Differently from [`Index.first_track_type`](Index::first_track_type),
    /// it does not require indexing the entire media file first.
    ///
    /// If an indexed file has already been created, it is recommended to use
    /// the [`Index.first_track_type`](Index::first_track_type) method,
    /// since the indexer is destructed when a new [`Index`]
    /// instance is created.
    ///
    /// Returns an error when a wrong track number is passed.
    pub fn track_type(&self, track: usize) -> Result<TrackType> {
        if track > self.tracks_count() - 1 {
            return Err(Error::WrongTrack);
        }

        let track_type =
            unsafe { ffms2_sys::FFMS_GetTrackTypeI(self.0, track as i32) };
        Ok(TrackType::new(track_type))
    }

    /// Stops the indexing process and destroys the indexer.
    ///
    /// This method should be used when there is no longer any interest in
    /// further additional tracks than the ones which have already been indexed.
    pub fn cancel(&self) {
        unsafe {
            ffms2_sys::FFMS_CancelIndexing(self.0);
        }
    }

    /// Runs the actual indexing process.
    ///
    /// This function destroys the [`Indexer`] and frees the allocated memory,
    /// even when the indexing process fails for any reason.
    pub fn do_indexing(
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
            Ok(Index::create_index(index))
        }
    }

    /// Enables the indexing process for the given track number.
    ///
    /// If an audio track is passed as input, it enables dumping the decoded
    /// audio during the indexing process.
    ///
    /// Returns an error when a wrong track number is passed.
    pub fn enable_track(&mut self, track: usize) -> Result<()> {
        if track > self.tracks_count() - 1 {
            return Err(Error::WrongTrack);
        }

        unsafe {
            ffms2_sys::FFMS_TrackIndexSettings(self.0, track as i32, 1, 0);
        }
        Ok(())
    }

    /// Disables the indexing process for the given track number.
    ///
    /// If an audio track is passed as input, it disables dumping the decoded
    /// audio during the indexing process.
    ///
    /// Returns an error when a wrong track number is passed.
    pub fn disable_track(&mut self, track: usize) -> Result<()> {
        if track > self.tracks_count() - 1 {
            return Err(Error::WrongTrack);
        }

        unsafe {
            ffms2_sys::FFMS_TrackIndexSettings(self.0, track as i32, 0, 0);
        }
        Ok(())
    }

    /// Enables the indexing process for the given track type.
    ///
    /// If an audio track type is passed as input, it enables dumping the
    /// decoded audio during the indexing process.
    ///
    /// Returns an error when a [`TrackType.Unknown`](TrackType::Unknown)
    /// track type is passed.
    pub fn enable_track_type(&mut self, track_type: TrackType) -> Result<()> {
        if matches!(track_type, TrackType::Unknown) {
            return Err(Error::UnknownTrackType);
        }

        unsafe {
            ffms2_sys::FFMS_TrackTypeIndexSettings(
                self.0,
                TrackType::ffms2_track_type(track_type) as i32,
                1,
                0,
            );
        }

        Ok(())
    }

    /// Disables the indexing process for the given track type.
    ///
    /// If an audio track type is passed as input, it disables dumping the
    /// decoded audio during the indexing process.
    ///
    /// Returns an error when a [`TrackType.Unknown`](TrackType::Unknown)
    /// track type is passed.
    pub fn disable_track_type(&mut self, track_type: TrackType) -> Result<()> {
        if matches!(track_type, TrackType::Unknown) {
            return Err(Error::UnknownTrackType);
        }

        unsafe {
            ffms2_sys::FFMS_TrackTypeIndexSettings(
                self.0,
                TrackType::ffms2_track_type(track_type) as i32,
                0,
                0,
            );
        }

        Ok(())
    }

    /// Sets a callback function for indexing progress updates.
    ///
    /// A progress callback is called regularly during indexing to report
    /// progress. It also offers the chance to interrupt indexing.
    ///
    /// Callback function's arguments are:
    /// - **current**: the current progress percentage
    /// - **total**: the maximum progress percentage
    /// - **fixed percentage**: an optional value which can be passed to the
    ///   function in order to modify the progress process.
    ///
    /// The indexing progress is usually calculated dividing
    /// `current` by `total`.
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
                Err(e) => {
                    println!("Aborting the process because of the ffms2 callback error: {:?}", e);
                    process::abort()
                }
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

    #[inline(always)]
    fn create_indexer(indexer: *mut FFMS_Indexer) -> Self {
        let track_count =
            unsafe { ffms2_sys::FFMS_GetNumTracksI(indexer) as usize };
        Self(indexer, track_count)
    }
}
