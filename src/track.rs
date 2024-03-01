use std::path::Path;

use std::ffi::CString;

use ffms2_sys::{
    FFMS_GetFrameInfo, FFMS_GetNumFrames, FFMS_GetTimeBase,
    FFMS_GetTrackFromAudio, FFMS_GetTrackFromIndex, FFMS_GetTrackFromVideo,
    FFMS_GetTrackType, FFMS_Track, FFMS_TrackTimeBase, FFMS_TrackType,
    FFMS_WriteTimecodes,
};

use crate::error::{Error, InternalError, Result};

use crate::audio::*;
use crate::frame::*;
use crate::index::*;
use crate::video::*;

create_enum!(
    TrackType,
    FFMS_TrackType,
    track_type,
    (
        TYPE_UNKNOWN,
        TYPE_VIDEO,
        TYPE_AUDIO,
        TYPE_DATA,
        TYPE_SUBTITLE,
        TYPE_ATTACHMENT,
    )
);

from_i32!(
    TrackType,
    FFMS_TrackType,
    (
        TYPE_UNKNOWN,
        TYPE_VIDEO,
        TYPE_AUDIO,
        TYPE_DATA,
        TYPE_SUBTITLE,
        TYPE_ATTACHMENT,
    )
);

create_struct!(
    TrackTimeBase,
    track_time_base,
    FFMS_TrackTimeBase,
    (Num, Den),
    (0, 0)
);

pub struct Track {
    track: *mut FFMS_Track,
}

unsafe impl Send for Track {}

impl Track {
    pub fn TrackFromIndex(index: &Index, Track: usize) -> Result<Self> {
        let track = unsafe {
            FFMS_GetTrackFromIndex(index.as_mut_ptr(), Track as i32)
        };
        Self::evaluate_track(track)
    }

    pub fn TrackFromVideo(V: &mut VideoSource) -> Result<Self> {
        let track = unsafe { FFMS_GetTrackFromVideo(V.as_mut_ptr()) };
        Self::evaluate_track(track)
    }

    pub fn TrackFromAudio(A: &mut AudioSource) -> Result<Self> {
        let track = unsafe { FFMS_GetTrackFromAudio(A.as_mut_ptr()) };
        Self::evaluate_track(track)
    }

    pub fn WriteTimecodes(&self, TimecodeFile: &Path) -> Result<()> {
        let source = CString::new(TimecodeFile.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let err = unsafe {
            FFMS_WriteTimecodes(
                self.track,
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

    pub fn FrameInfo(&self, Frame: usize) -> FrameInfo {
        let res_frame = unsafe { FFMS_GetFrameInfo(self.track, Frame as i32) };
        let ref_frame = unsafe { &*res_frame };
        FrameInfo::create_struct(ref_frame)
    }

    pub fn TimeBase(&self) -> TrackTimeBase {
        let res_track = unsafe { FFMS_GetTimeBase(self.track) };
        let ref_track = unsafe { &*res_track };
        TrackTimeBase {
            track_time_base: *ref_track,
        }
    }

    pub fn TrackType(&self) -> TrackType {
        let track_type = unsafe { FFMS_GetTrackType(self.track) };
        TrackType::from_i32(track_type)
    }

    pub fn NumFrames(&self) -> Result<usize> {
        let num_frames = unsafe { FFMS_GetNumFrames(self.track) };
        if num_frames < 0 {
            Err(Error::Frames)
        } else {
            Ok(num_frames as usize)
        }
    }

    fn evaluate_track(track: *mut FFMS_Track) -> Result<Self> {
        let num_frames = unsafe { FFMS_GetNumFrames(track) };
        if num_frames < 0 {
            Err(Error::Track)
        } else {
            Ok(Self { track })
        }
    }
}
