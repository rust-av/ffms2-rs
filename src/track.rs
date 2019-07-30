use crate::audio::*;
use crate::frame::*;
use crate::index::*;
use crate::video::*;
use crate::*;

use std::ffi::CString;
use std::mem;
use std::path::PathBuf;

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
    (usize, usize),
    (0, 0),
    (Num as i64, Den as i64)
);

macro_rules! track_error {
    ($track:expr, $num_frames:ident) => {
        let $num_frames = unsafe { FFMS_GetNumFrames($track) };
        if $num_frames != 0 {
            panic!("Error creating the track");
        }
    };
}

pub struct Track {
    track: *mut FFMS_Track,
}

impl Track {
    pub fn TrackFromIndex(index: &Index, Track: usize) -> Self {
        let track = unsafe {
            FFMS_GetTrackFromIndex(index.as_mut_ptr(), Track as i32)
        };
        track_error!(track, num_frames);
        Track { track }
    }

    pub fn TrackFromVideo(V: &mut VideoSource) -> Self {
        let track = unsafe { FFMS_GetTrackFromVideo(V.as_mut_ptr()) };
        track_error!(track, num_frames);
        Track { track }
    }

    pub fn TrackFromAudio(A: &mut AudioSource) -> Self {
        let track = unsafe { FFMS_GetTrackFromAudio(A.as_mut_ptr()) };
        track_error!(track, num_frames);
        Track { track }
    }

    pub fn WriteTimecodes(&self, TimecodeFile: &PathBuf) -> Result<(), Error> {
        let source = CString::new(TimecodeFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let err = unsafe {
            FFMS_WriteTimecodes(
                self.track,
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

    pub fn FrameInfo(&self, Frame: usize) -> FrameInfo {
        let res_frame = unsafe { FFMS_GetFrameInfo(self.track, Frame as i32) };
        let ref_frame = unsafe {
            mem::transmute::<*const FFMS_FrameInfo, &FFMS_FrameInfo>(res_frame)
        };
        FrameInfo::create_struct(ref_frame)
    }

    pub fn TimeBase(&self) -> TrackTimeBase {
        let res_track = unsafe { FFMS_GetTimeBase(self.track) };
        let ref_track = unsafe {
            mem::transmute::<*const FFMS_TrackTimeBase, &FFMS_TrackTimeBase>(
                res_track,
            )
        };
        TrackTimeBase {
            track_time_base: *ref_track,
        }
    }

    pub fn TrackType(&self) -> TrackType {
        let track_type = unsafe { FFMS_GetTrackType(self.track) };
        TrackType::from_i32(track_type)
    }

    pub fn NumFrames(&self) -> usize {
        track_error!(self.track, num_frames);
        num_frames as usize
    }
}
