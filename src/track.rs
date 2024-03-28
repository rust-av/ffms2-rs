use std::borrow::Cow;
use std::path::Path;

use std::ffi::CString;

use ffms2_sys::FFMS_TrackType;

use crate::error::{Error, InternalError, Result};

use crate::audio::AudioSource;
use crate::frame::FrameInfo;
use crate::index::Index;
use crate::video::VideoSource;

/// Track type.
///
/// It defines the datatype contained in a multimedia stream.
#[derive(Clone, Copy, Debug)]
pub enum TrackType {
    /// Unknown datatype.
    Unknown,
    /// A video stream.
    Video,
    /// An audio stream.
    Audio,
    /// A stream of bytes.
    ///
    /// Not supported by ffms2.
    Data,
    /// A subtitle stream.
    ///
    /// Not supported by ffms2,
    Subtitle,
    /// Attachment stream.
    ///
    /// Not supported by ffms2.
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

/// Basic time unit of a track.
///
/// This information are only meaningful for video tracks.
///
/// The rational number obtained dividing the `numerator` and `denominator`
/// fields of this structure might occasionally be equal to`1/framerate`
/// for some CFR video tracks.
/// However, this similarity has no relation whatsoever with the video framerate
/// and it is not related at all with any framerate concept.
pub struct TrackTimebase {
    /// Timebase numerator.
    pub numerator: usize,
    /// Timebase denominator.
    pub denominator: usize,
}

/// A track contains all information associated to a given multimedia stream.
///
/// It does not contain any index information.
pub struct Track(*mut ffms2_sys::FFMS_Track);

unsafe impl Send for Track {}

impl Track {
    /// Builds a `[Track]` from the given `[Index]` and track number.
    pub fn from_index(index: &Index, track_number: usize) -> Result<Self> {
        if track_number > index.tracks_count() - 1 {
            return Err(Error::WrongTrack);
        }

        let track = unsafe {
            ffms2_sys::FFMS_GetTrackFromIndex(
                index.as_mut_ptr(),
                track_number as i32,
            )
        };
        Self::evaluate_track(track)
    }

    /// Builds a new `[Track]` from a video source.
    ///
    /// The `[TrackType]` is `Video`.
    pub fn from_video(video_source: &mut VideoSource) -> Result<Self> {
        let track = unsafe {
            ffms2_sys::FFMS_GetTrackFromVideo(video_source.as_mut_ptr())
        };
        Self::evaluate_track(track)
    }

    /// Builds a new `[Track]` from an audio source.
    ///
    /// The `[TrackType]` is `Audio`.
    pub fn from_audio(audio_source: &mut AudioSource) -> Result<Self> {
        let track = unsafe {
            ffms2_sys::FFMS_GetTrackFromAudio(audio_source.as_mut_ptr())
        };
        Self::evaluate_track(track)
    }

    /// Writes `Matroska` v2 timecodes for the track in a file.
    ///
    /// Only meaningful for video tracks.
    pub fn write_timecodes(&self, timecode_file: &Path) -> Result<()> {
        if timecode_file.is_file() {
            return Err(Error::NotAFile);
        }

        let source =
            CString::new(timecode_file.to_str().ok_or(Error::StrConversion)?)?;

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

    /// Returns the information on the indexed frame passed as input.
    pub fn frame_info(&self, frame: usize) -> FrameInfo {
        let res_frame =
            unsafe { ffms2_sys::FFMS_GetFrameInfo(self.0, frame as i32) };
        let ref_frame = unsafe { *res_frame };
        FrameInfo::new(ref_frame)
    }

    /// Returns the track timebase information.
    ///
    /// Only meaningful for video tracks.
    pub fn time_base(&self) -> TrackTimebase {
        let res_track = unsafe { ffms2_sys::FFMS_GetTimeBase(self.0) };
        let ref_track = unsafe { &*res_track };

        TrackTimebase {
            numerator: ref_track.Num as usize,
            denominator: ref_track.Den as usize,
        }
    }

    /// Returns the track type.
    pub fn track_type(&self) -> TrackType {
        let track_type = unsafe { ffms2_sys::FFMS_GetTrackType(self.0) };
        TrackType::new(track_type)
    }

    /// Returns the number of frames present in the track.
    ///
    /// This value is:
    /// - the number of video frames for a video track
    /// - the number of packets for an audio track
    ///
    /// An error indicates the track has not been indexed.
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
