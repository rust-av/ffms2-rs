use std::path::Path;

use std::ffi::CString;

use ffms2_sys::FFMS_TrackType;

use crate::error::{Error, InternalError, Result};

use crate::audio::AudioSource;
use crate::frame::FrameInfo;
use crate::index::Index;
use crate::video::VideoSource;

#[derive(Clone, Copy, Debug)]
pub enum TrackType {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl TrackType {
    pub(crate) const fn ffms2_track_type(self) -> FFMS_TrackType {
        match self {
            Self::Unknown => FFMS_TrackType::FFMS_TYPE_UNKNOWN,
            Self::Video => FFMS_TrackType::FFMS_TYPE_VIDEO,
            Self::Audio => FFMS_TrackType::FFMS_TYPE_AUDIO,
            Self::Data => FFMS_TrackType::FFMS_TYPE_DATA,
            Self::Subtitle => FFMS_TrackType::FFMS_TYPE_SUBTITLE,
            Self::Attachment => FFMS_TrackType::FFMS_TYPE_ATTACHMENT,
        }
    }

    pub(crate) const fn new(track_type: i32) -> Self {
        match track_type {
            e if e == FFMS_TrackType::FFMS_TYPE_UNKNOWN as i32 => {
                Self::Unknown
            }
            e if e == FFMS_TrackType::FFMS_TYPE_VIDEO as i32 => Self::Video,
            e if e == FFMS_TrackType::FFMS_TYPE_AUDIO as i32 => Self::Audio,
            e if e == FFMS_TrackType::FFMS_TYPE_DATA as i32 => Self::Data,
            e if e == FFMS_TrackType::FFMS_TYPE_SUBTITLE as i32 => {
                Self::Subtitle
            }
            e if e == FFMS_TrackType::FFMS_TYPE_ATTACHMENT as i32 => {
                Self::Attachment
            }
            _ => Self::Unknown,
        }
    }
}

pub struct TrackTimebase {
    pub numerator: usize,
    pub denominator: usize,
}

pub struct Track(*mut ffms2_sys::FFMS_Track);

unsafe impl Send for Track {}

impl Track {
    pub fn track_from_index(index: &Index, track: usize) -> Result<Self> {
        let track = unsafe {
            ffms2_sys::FFMS_GetTrackFromIndex(index.as_mut_ptr(), track as i32)
        };
        Self::evaluate_track(track)
    }

    pub fn track_from_video(video_source: &mut VideoSource) -> Result<Self> {
        let track = unsafe {
            ffms2_sys::FFMS_GetTrackFromVideo(video_source.as_mut_ptr())
        };
        Self::evaluate_track(track)
    }

    pub fn track_from_audio(audio_source: &mut AudioSource) -> Result<Self> {
        let track = unsafe {
            ffms2_sys::FFMS_GetTrackFromAudio(audio_source.as_mut_ptr())
        };
        Self::evaluate_track(track)
    }

    pub fn write_timecodes(&self, timecode_file: &Path) -> Result<()> {
        let source = CString::new(timecode_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_WriteTimecodes(
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

    pub fn frame_info(&self, frame: usize) -> FrameInfo {
        let res_frame =
            unsafe { ffms2_sys::FFMS_GetFrameInfo(self.0, frame as i32) };
        let ref_frame = unsafe { &*res_frame };
        FrameInfo::new(ref_frame)
    }

    pub fn time_base(&self) -> TrackTimebase {
        let res_track = unsafe { ffms2_sys::FFMS_GetTimeBase(self.0) };
        let ref_track = unsafe { &*res_track };

        TrackTimebase {
            numerator: ref_track.Num as usize,
            denominator: ref_track.Den as usize,
        }
    }

    pub fn track_type(&self) -> TrackType {
        let track_type = unsafe { ffms2_sys::FFMS_GetTrackType(self.0) };
        TrackType::new(track_type)
    }

    pub fn frames_count(&self) -> Result<usize> {
        let num_frames = unsafe { ffms2_sys::FFMS_GetNumFrames(self.0) };
        if num_frames < 0 {
            Err(Error::Frames)
        } else {
            Ok(num_frames as usize)
        }
    }

    fn evaluate_track(track: *mut ffms2_sys::FFMS_Track) -> Result<Self> {
        let num_frames = unsafe { ffms2_sys::FFMS_GetNumFrames(track) };
        if num_frames < 0 {
            Err(Error::Track)
        } else {
            Ok(Self(track))
        }
    }
}
