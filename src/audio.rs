use std::mem;

use std::ffi::c_void;
use std::ffi::CString;
use std::path::Path;

use ffms2_sys::{
    FFMS_AudioDelayModes, FFMS_AudioGapFillModes, FFMS_MatrixEncoding,
};

use crate::error::{InternalError, Result};
use crate::index::Index;
use crate::resample::ResampleOptions;

/// Audio channel layout of an audio stream.
#[derive(Clone, Copy, Debug, Default)]
pub enum AudioChannel {
    /// Unknown audio channel.
    #[default]
    Unknown,
    /// Front left.
    FrontLeft,
    /// Front right.
    FrontRight,
    /// Front center.
    FrontCenter,
    /// Low Frequency Effects.
    LowFrequency,
    /// Back left.
    BackLeft,
    /// Back right.
    BackRight,
    /// Front left-of-center.
    FrontLeftOfCenter,
    /// Front right-of-center.
    FrontRightOfCenter,
    /// Back center.
    BackCenter,
    /// Side left.
    SideLeft,
    /// Side right.
    SideRight,
    /// Top center.
    TopCenter,
    /// Top front left.
    TopFrontLeft,
    /// Top front center.
    TopFrontCenter,
    /// Top front right.
    TopFrontRight,
    /// Top back left.
    TopBackLeft,
    /// Top back center.
    TopBackCenter,
    /// Top back right.
    TopBackRight,
    /// Stereo downmix left.
    StereoLeft,
    /// Stereo downmix right.
    StereoRight,
}

impl AudioChannel {
    const fn new(audio_channel: i64) -> Self {
        use ffms2_sys::FFMS_AudioChannel::*;
        match audio_channel {
            e if e == FFMS_CH_FRONT_LEFT as i64 => Self::FrontLeft,
            e if e == FFMS_CH_FRONT_RIGHT as i64 => Self::FrontRight,
            e if e == FFMS_CH_FRONT_CENTER as i64 => Self::FrontCenter,
            e if e == FFMS_CH_LOW_FREQUENCY as i64 => Self::LowFrequency,
            e if e == FFMS_CH_BACK_LEFT as i64 => Self::BackLeft,
            e if e == FFMS_CH_BACK_RIGHT as i64 => Self::BackRight,
            e if e == FFMS_CH_FRONT_LEFT_OF_CENTER as i64 => {
                Self::FrontLeftOfCenter
            }
            e if e == FFMS_CH_FRONT_RIGHT_OF_CENTER as i64 => {
                Self::FrontRightOfCenter
            }
            e if e == FFMS_CH_BACK_CENTER as i64 => Self::BackCenter,
            e if e == FFMS_CH_SIDE_LEFT as i64 => Self::SideLeft,
            e if e == FFMS_CH_SIDE_RIGHT as i64 => Self::SideRight,
            e if e == FFMS_CH_TOP_CENTER as i64 => Self::TopCenter,
            e if e == FFMS_CH_TOP_FRONT_LEFT as i64 => Self::TopFrontLeft,
            e if e == FFMS_CH_TOP_FRONT_CENTER as i64 => Self::TopFrontCenter,
            e if e == FFMS_CH_TOP_FRONT_RIGHT as i64 => Self::TopFrontRight,
            e if e == FFMS_CH_TOP_BACK_LEFT as i64 => Self::TopBackLeft,
            e if e == FFMS_CH_TOP_BACK_CENTER as i64 => Self::TopBackCenter,
            e if e == FFMS_CH_TOP_BACK_RIGHT as i64 => Self::TopBackRight,
            e if e == FFMS_CH_STEREO_LEFT as i64 => Self::StereoLeft,
            e if e == FFMS_CH_STEREO_RIGHT as i64 => Self::StereoRight,
            _ => Self::Unknown,
        }
    }
}

/// Modes to manage audio with discontinuous timestamps.
///
/// Zero filled gaps can be added or not according to the chosen mode.
#[derive(Clone, Copy, Debug, Default)]
pub enum AudioGapFillModes {
    /// Automatic mode.
    ///
    /// Zero filled gaps are applied **only** to audio
    /// associated with containers where this kind of operation is usually
    /// necessary.
    #[default]
    Auto,
    /// Disabled mode.
    ///
    /// Never zero fill gaps.
    Disabled,
    /// Enabled mode.
    ///
    /// Always zero fill gaps.
    Enabled,
}

impl AudioGapFillModes {
    const fn ffms2_audio_gap_fill_modes(self) -> FFMS_AudioGapFillModes {
        match self {
            Self::Auto => FFMS_AudioGapFillModes::FFMS_GAP_FILL_AUTO,
            Self::Disabled => FFMS_AudioGapFillModes::FFMS_GAP_FILL_DISABLED,
            Self::Enabled => FFMS_AudioGapFillModes::FFMS_GAP_FILL_ENABLED,
        }
    }
}

/// Modes to manage audio delay.
///
/// A possible use case could be how to treat the first audio sample in the
/// file which does not have a timestamp of zero.
#[derive(Clone, Copy, Debug, Default)]
pub enum AudioDelay {
    /// No shift mode.
    ///
    /// No adjustment is made. The first decodable audio sample becomes
    /// the first sample in the output.
    NoShift,
    /// Time zero mode.
    ///
    /// Audio samples are created (with silence) or discarded, so that
    /// the 0-index sample in the decoded audio starts at time zero.
    TimeZero,
    /// First video track mode.
    ///
    /// Audio samples are created (with silence) or discarded,
    /// so that the 0-index sample 0 in the decoded audio starts at the same
    /// time as the 0-index frame of the first video track.
    ///
    /// Same as `TimeZero` mode if the first video frame of the first video
    /// track starts at time zero.
    ///
    /// This mode is the default one.
    #[default]
    FirstVideoTrack,
    /// Index track mode.
    ///
    /// Same as `NoShift`, but it acts on the audio samples of the video
    /// track passed as input.
    IndexTrack(usize),
}

impl AudioDelay {
    const fn ffms2_audio_delay(self) -> i32 {
        match self {
            Self::NoShift => FFMS_AudioDelayModes::FFMS_DELAY_NO_SHIFT as i32,
            Self::TimeZero => {
                FFMS_AudioDelayModes::FFMS_DELAY_TIME_ZERO as i32
            }
            Self::FirstVideoTrack => {
                FFMS_AudioDelayModes::FFMS_DELAY_FIRST_VIDEO_TRACK as i32
            }
            Self::IndexTrack(val) => val as i32,
        }
    }
}

/// Surround Sound Matrix Encoding.
///
/// Matrix encoding is an audio technique which transforms N-channel signals to
/// M-channel signals, where N > M, enabling the same audio content to be
/// played on different systems.
#[derive(Clone, Copy, Debug, Default)]
pub enum MatrixEncoding {
    #[default]
    /// No matrix encoding.
    None,
    /// Dolby.
    Dolby,
    /// Dolby Surround Pro Logic II.
    ProLogicII,
    /// Dolby Surround Pro Logic IIX.
    ProLogicIIX,
    /// Dolby Surround Pro Logic IIZ.
    ProLogicIIZ,
    /// Dolby Digital Ex.
    DolbyEx,
    /// Dolby Headphone.
    DolbyHeadphone,
}

impl MatrixEncoding {
    pub(crate) const fn new(matrix_encoding: FFMS_MatrixEncoding) -> Self {
        use ffms2_sys::FFMS_MatrixEncoding::*;
        match matrix_encoding {
            FFMS_MATRIX_ENCODING_NONE => Self::None,
            FFMS_MATRIX_ENCODING_DOBLY => Self::Dolby,
            FFMS_MATRIX_ENCODING_PRO_LOGIC_II => Self::ProLogicII,
            FFMS_MATRIX_ENCODING_PRO_LOGIC_IIX => Self::ProLogicIIX,
            FFMS_MATRIX_ENCODING_PRO_LOGIC_IIZ => Self::ProLogicIIZ,
            FFMS_MATRIX_ENCODING_DOLBY_EX => Self::DolbyEx,
            FFMS_MATRIX_ENCODING_DOLBY_HEADPHONE => Self::DolbyHeadphone,
        }
    }
}

/// Audio properties.
#[derive(Debug)]
pub struct AudioProperties {
    pub sample_format: usize,
    pub sample_rate: usize,
    pub bits_per_sample: usize,
    pub channels_count: usize,
    pub channel_layout: AudioChannel,
    pub samples_count: usize,
    pub first_time: usize,
    pub last_time: f64,
    pub last_end_time: f64,
}

/// Audio source manager.
///
/// Among its functionalities:
/// - Opening an audio source
/// - Retrieving audio samples data
/// - Setting the output data format
pub struct AudioSource {
    audio_source: *mut ffms2_sys::FFMS_AudioSource,
}

unsafe impl Send for AudioSource {}

impl AudioSource {
    /// Creates a new `[AudioSource]` instance.
    pub fn new(
        source_file: &Path,
        track: usize,
        index: &Index,
        delay_mode: AudioDelay,
    ) -> Result<Self> {
        let source = CString::new(source_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let audio_source = unsafe {
            ffms2_sys::FFMS_CreateAudioSource(
                source.as_ptr(),
                track as i32,
                index.as_mut_ptr(),
                delay_mode.ffms2_audio_delay() as i32,
                error.as_mut_ptr(),
            )
        };

        if audio_source.is_null() {
            Err(error.into())
        } else {
            Ok(AudioSource { audio_source })
        }
    }

    /// Creates a new `[AudioSource]` instance also considering the mode to fill
    /// audio gaps and the Dynamic Range Compression, which balances the range
    /// between the loudest and quietest sounds.
    pub fn audio_source_2(
        source_file: &Path,
        track: usize,
        index: &Index,
        delay_mode: AudioDelay,
        fill_gaps: AudioGapFillModes,
        drc_scale: f64,
    ) -> Result<Self> {
        let source = CString::new(source_file.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let audio_source = unsafe {
            ffms2_sys::FFMS_CreateAudioSource2(
                source.as_ptr(),
                track as i32,
                index.as_mut_ptr(),
                delay_mode.ffms2_audio_delay() as i32,
                fill_gaps.ffms2_audio_gap_fill_modes() as i32,
                drc_scale,
                error.as_mut_ptr(),
            )
        };

        if audio_source.is_null() {
            Err(error.into())
        } else {
            Ok(AudioSource { audio_source })
        }
    }

    /// Returns the `[AudioProperties]`.
    pub fn audio_properties(&self) -> AudioProperties {
        let audio_prop =
            unsafe { ffms2_sys::FFMS_GetAudioProperties(self.audio_source) };
        let audio_ref = unsafe { &*audio_prop };

        AudioProperties {
            sample_format: audio_ref.SampleFormat as usize,
            sample_rate: audio_ref.SampleRate as usize,
            bits_per_sample: audio_ref.BitsPerSample as usize,
            channels_count: audio_ref.Channels as usize,
            channel_layout: AudioChannel::new(audio_ref.ChannelLayout),
            samples_count: audio_ref.NumSamples as usize,
            first_time: audio_ref.FirstTime as usize,
            last_time: audio_ref.LastTime,
            last_end_time: audio_ref.LastEndTime,
        }
    }

    /// Returns audio data.
    pub fn audio<T>(
        &self,
        sample_start: usize,
        samples_count: usize,
    ) -> Result<Vec<T>> {
        let mut error = InternalError::new();
        let audio_prop = self.audio_properties();

        if sample_start > (audio_prop.samples_count - 1)
            || samples_count > (audio_prop.samples_count - 1)
        {
            panic!("Requesting samples beyond the stream end");
        }

        let num_elements = samples_count * audio_prop.channels_count;

        let buf: Vec<T> = Vec::with_capacity(num_elements);
        let mut buf = mem::ManuallyDrop::new(buf);
        let buf_ptr = buf.as_mut_ptr();

        let err = unsafe {
            ffms2_sys::FFMS_GetAudio(
                self.audio_source,
                buf_ptr as *mut c_void,
                sample_start as i64,
                samples_count as i64,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            let audio_vec = unsafe {
                Vec::from_raw_parts(buf_ptr, num_elements, num_elements)
            };

            Ok(audio_vec)
        }
    }

    /// Returns the `[ResampleOptions]`.
    pub fn create_resample_options(&self) -> ResampleOptions {
        let res_opt = unsafe {
            ffms2_sys::FFMS_CreateResampleOptions(self.audio_source)
        };
        let ref_res = unsafe { &*res_opt };

        ResampleOptions::create_struct(ref_res)
    }

    /// Sets audio samples output format.
    pub fn output_format(&self, options: &ResampleOptions) -> Result<()> {
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_SetOutputFormatA(
                self.audio_source,
                options.as_ptr(),
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffms2_sys::FFMS_AudioSource {
        self.audio_source
    }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        unsafe {
            ffms2_sys::FFMS_DestroyAudioSource(self.audio_source);
        }
    }
}
