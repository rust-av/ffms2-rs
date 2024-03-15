use std::mem;

use std::borrow::Cow;
use std::path::Path;

use std::ffi::c_void;
use std::ffi::CString;

use ffms2_sys::{
    FFMS_AudioChannel, FFMS_AudioDelayModes, FFMS_AudioGapFillModes,
};

use crate::error::{Error, InternalError, Result};
use crate::index::Index;
use crate::resample::{ResampleOptions, SampleFormat};

/// Audio channel layout of an audio stream.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioChannel {
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
    const AUDIO_CHANNELS: [(FFMS_AudioChannel, AudioChannel); 20] = [
        // First 4 bits block from right
        (FFMS_AudioChannel::FFMS_CH_FRONT_LEFT, Self::FrontLeft),
        (FFMS_AudioChannel::FFMS_CH_FRONT_RIGHT, Self::FrontRight),
        (FFMS_AudioChannel::FFMS_CH_FRONT_CENTER, Self::FrontCenter),
        (FFMS_AudioChannel::FFMS_CH_LOW_FREQUENCY, Self::LowFrequency),
        // Second 4 bits block from right
        (FFMS_AudioChannel::FFMS_CH_BACK_LEFT, Self::BackLeft),
        (FFMS_AudioChannel::FFMS_CH_BACK_RIGHT, Self::BackRight),
        (
            FFMS_AudioChannel::FFMS_CH_FRONT_LEFT_OF_CENTER,
            Self::FrontLeftOfCenter,
        ),
        (
            FFMS_AudioChannel::FFMS_CH_FRONT_RIGHT_OF_CENTER,
            Self::FrontRightOfCenter,
        ),
        // Third 4 bits block from right
        (FFMS_AudioChannel::FFMS_CH_BACK_CENTER, Self::BackCenter),
        (FFMS_AudioChannel::FFMS_CH_SIDE_LEFT, Self::SideLeft),
        (FFMS_AudioChannel::FFMS_CH_SIDE_RIGHT, Self::SideRight),
        (FFMS_AudioChannel::FFMS_CH_TOP_CENTER, Self::TopCenter),
        // Fourth 4 bits block from right
        (
            FFMS_AudioChannel::FFMS_CH_TOP_FRONT_LEFT,
            Self::TopFrontLeft,
        ),
        (
            FFMS_AudioChannel::FFMS_CH_TOP_FRONT_CENTER,
            Self::TopFrontCenter,
        ),
        (
            FFMS_AudioChannel::FFMS_CH_TOP_FRONT_RIGHT,
            Self::TopFrontRight,
        ),
        (FFMS_AudioChannel::FFMS_CH_TOP_BACK_LEFT, Self::TopBackLeft),
        // Fifth 4 bits block from right
        (
            FFMS_AudioChannel::FFMS_CH_TOP_BACK_CENTER,
            Self::TopBackCenter,
        ),
        (
            FFMS_AudioChannel::FFMS_CH_TOP_BACK_RIGHT,
            Self::TopBackRight,
        ),
        // Eight 4 bits from right
        (FFMS_AudioChannel::FFMS_CH_STEREO_LEFT, Self::StereoLeft),
        (FFMS_AudioChannel::FFMS_CH_STEREO_RIGHT, Self::StereoRight),
    ];

    pub(crate) fn channel_map(audio_channel: i64) -> Option<Vec<Self>> {
        let channels_map = Self::AUDIO_CHANNELS
            .iter()
            .flat_map(|(ffms2_channel, channel)| {
                if audio_channel & *ffms2_channel as i64 == 1 {
                    Some(*channel)
                } else {
                    None
                }
            })
            .collect::<Vec<Self>>();

        channels_map.is_empty().then_some(channels_map)
    }

    pub(crate) fn into_ffms2(channels_map: &[Self]) -> i64 {
        channels_map.iter().fold(0, |acc, audio_channel| {
            if let Some(channel) = Self::AUDIO_CHANNELS
                .iter()
                .find(|(_, channel)| *audio_channel == *channel)
            {
                acc | channel.0 as i64
            } else {
                acc
            }
        })
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

/// Audio properties.
#[derive(Debug)]
pub struct AudioProperties {
    /// Audio sample format.
    ///
    /// If `None`, no sample format has been found.
    pub sample_format: Option<SampleFormat>,
    /// Audio sample rate (samples/second).
    pub sample_rate: usize,
    /// The number of bits per audio sample.
    ///
    /// It represents the number of bits actually used to code each sample,
    /// not the number of bits used to store each sample,
    /// and may hence be different from what `[SampleFormat]` would imply.
    ///
    /// Figuring out which bytes are significant and which are not is left to
    /// a developer.
    pub bits_per_sample: usize,
    /// The number of audio channels.
    pub channels_count: usize,
    /// Audio stream channel layout.
    ///
    /// If `None`, no channel layout has been found.
    pub channel_layout: Option<Vec<AudioChannel>>,
    /// Audio stream number of samples.
    pub samples_count: usize,
    /// Audio stream first timestamp in milliseconds.
    ///
    /// Useful to know if the audio stream has a delay, or for quickly
    /// determining its length in seconds.
    pub first_time: f64,
    /// Audio stream last timestamp in milliseconds.
    ///
    /// Useful to know if the audio stream has a delay, or for quickly
    /// determining its length in seconds.
    pub last_time: f64,
    /// Audio stream last packet end time in milliseconds.
    pub last_end_time: f64,
}

/// Audio source manager.
///
/// Among its functionalities:
/// - Opening an audio source
/// - Retrieving audio samples data
/// - Setting the output data format
pub struct AudioSource(*mut ffms2_sys::FFMS_AudioSource);

unsafe impl Send for AudioSource {}

impl AudioSource {
    /// Creates a new `[AudioSource]` instance.
    pub fn new(
        source_file: &Path,
        track: usize,
        index: &Index,
        delay_mode: AudioDelay,
    ) -> Result<Self> {
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;
        let mut error = InternalError::new();
        let audio_source = unsafe {
            ffms2_sys::FFMS_CreateAudioSource(
                source.as_ptr(),
                track as i32,
                index.as_mut_ptr(),
                delay_mode.ffms2_audio_delay(),
                error.as_mut_ptr(),
            )
        };

        if audio_source.is_null() {
            Err(error.into())
        } else {
            Ok(AudioSource(audio_source))
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
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;
        let mut error = InternalError::new();
        let audio_source = unsafe {
            ffms2_sys::FFMS_CreateAudioSource2(
                source.as_ptr(),
                track as i32,
                index.as_mut_ptr(),
                delay_mode.ffms2_audio_delay(),
                fill_gaps.ffms2_audio_gap_fill_modes() as i32,
                drc_scale,
                error.as_mut_ptr(),
            )
        };

        if audio_source.is_null() {
            Err(error.into())
        } else {
            Ok(AudioSource(audio_source))
        }
    }

    /// Returns the `[AudioProperties]` structure.
    pub fn audio_properties(&self) -> AudioProperties {
        let audio_prop = unsafe { ffms2_sys::FFMS_GetAudioProperties(self.0) };
        let audio_ref = unsafe { &*audio_prop };

        AudioProperties {
            sample_format: SampleFormat::new(audio_ref.SampleFormat as usize),
            sample_rate: audio_ref.SampleRate as usize,
            bits_per_sample: audio_ref.BitsPerSample as usize,
            channels_count: audio_ref.Channels as usize,
            channel_layout: AudioChannel::channel_map(audio_ref.ChannelLayout),
            samples_count: audio_ref.NumSamples as usize,
            first_time: audio_ref.FirstTime,
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
            return Err(Error::FFMS2(Cow::Borrowed(
                "Requesting samples beyond the stream end",
            )));
        }

        let num_elements = samples_count * audio_prop.channels_count;

        let buf: Vec<T> = Vec::with_capacity(num_elements);
        let mut buf = mem::ManuallyDrop::new(buf);
        let buf_ptr = buf.as_mut_ptr();

        let err = unsafe {
            ffms2_sys::FFMS_GetAudio(
                self.0,
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

    /// Returns the `[ResampleOptions]` structure.
    pub fn resample_options(&self) -> ResampleOptions {
        let res_opt = unsafe { ffms2_sys::FFMS_CreateResampleOptions(self.0) };
        let ref_res = unsafe { *res_opt };

        ResampleOptions::new(ref_res)
    }

    /// Sets audio samples output format.
    pub fn output_format(&self, options: &ResampleOptions) -> Result<()> {
        let channel_layout = match &options.channel_layout {
            Some(channel_layout) => channel_layout,
            None => {
                return Err(Error::FFMS2(Cow::Borrowed(
                    "Unknown audio channel.",
                )))
            }
        };

        let sample_format = match options.sample_format {
            Some(sample_format) => sample_format,
            None => {
                return Err(Error::FFMS2(Cow::Borrowed(
                    "Unknown sample format.",
                )))
            }
        };

        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_SetOutputFormatA(
                self.0,
                &options.ffms2_resample(channel_layout, sample_format),
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
        self.0
    }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        unsafe {
            ffms2_sys::FFMS_DestroyAudioSource(self.0);
        }
    }
}
