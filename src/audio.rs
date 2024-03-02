use std::mem;

use std::ffi::c_void;
use std::ffi::CString;
use std::path::Path;

use ffms2_sys::FFMS_AudioProperties;

use crate::error::{InternalError, Result};
use crate::index::Index;
use crate::resample::ResampleOptions;

#[derive(Clone, Copy, Debug)]
pub enum AudioChannel {
    FrontLeft,
    FrontRight,
    FrontCenter,
    LowFrequency,
    BackLeft,
    BackRight,
    FrontLeftOfCenter,
    FrontRightOfCenter,
    BackCenter,
    SideLeft,
    SideRight,
    TopCenter,
    TopFrontLeft,
    TopFrontCenter,
    TopFrontRight,
    TopBackLeft,
    TopBackCenter,
    TopBackRight,
    StereoLeft,
    StereoRight,
}

#[derive(Clone, Copy, Debug)]
pub enum AudioDelay {
    NoShift,
    TimeZero,
    FirstVideoTrack,
}

#[derive(Clone, Copy, Debug)]
pub enum MatrixEncoding {
    None,
    Dolby,
    ProLogicII,
    ProLogicIIX,
    ProLogicIIZ,
    DolbyEx,
    DolbyHeadphone,
}

#[derive(Debug)]
pub struct AudioProperties(FFMS_AudioProperties);

impl AudioProperties {
    pub const fn sample_format(&self) -> usize {
        self.0.SampleFormat as usize
    }

    pub const fn sample_rate(&self) -> usize {
        self.0.SampleRate as usize
    }

    pub const fn bits_per_sample(&self) -> usize {
        self.0.BitsPerSample as usize
    }

    pub const fn channels(&self) -> usize {
        self.0.Channels as usize
    }

    pub const fn channel_layout(&self) -> usize {
        self.0.ChannelLayout as usize
    }

    pub const fn samples_number(&self) -> usize {
        self.0.NumSamples as usize
    }

    pub const fn first_time(&self) -> usize {
        self.0.FirstTime as usize
    }

    pub const fn last_time(&self) -> f64 {
        self.0.LastTime
    }

    pub const fn last_end_time(&self) -> f64 {
        self.0.LastEndTime
    }
}

pub struct AudioSource {
    audio_source: *mut ffms2_sys::FFMS_AudioSource,
}

unsafe impl Send for AudioSource {}

impl AudioSource {
    pub fn new(
        SourceFile: &Path,
        Track: usize,
        Index: &Index,
        DelayMode: isize,
    ) -> Result<Self> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let audio_source = unsafe {
            ffms2_sys::FFMS_CreateAudioSource(
                source.as_ptr(),
                Track as i32,
                Index.as_mut_ptr(),
                DelayMode as i32,
                error.as_mut_ptr(),
            )
        };

        if audio_source.is_null() {
            Err(error.into())
        } else {
            Ok(AudioSource { audio_source })
        }
    }

    pub fn GetAudioProperties(&self) -> AudioProperties {
        let audio_prop =
            unsafe { ffms2_sys::FFMS_GetAudioProperties(self.audio_source) };
        let ref_audio = unsafe { &*audio_prop };

        AudioProperties(*ref_audio)
    }

    pub fn GetAudio<T>(&self, Start: usize, Count: usize) -> Result<Vec<T>> {
        let mut error = InternalError::new();
        let audio_prop = self.GetAudioProperties();
        let num_samples = audio_prop.0.NumSamples;

        if Start as i64 > (num_samples - 1) || Count as i64 > (num_samples - 1)
        {
            panic!("Requesting samples beyond the stream end");
        }

        let num_channels = audio_prop.0.Channels;
        let num_elements = Count * num_channels as usize;

        let Buf: Vec<T> = Vec::with_capacity(num_elements);
        let mut Buf = mem::ManuallyDrop::new(Buf);
        let buf_ptr = Buf.as_mut_ptr();

        let err = unsafe {
            ffms2_sys::FFMS_GetAudio(
                self.audio_source,
                buf_ptr as *mut c_void,
                Start as i64,
                Count as i64,
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

    pub fn CreateResampleOptions(&self) -> ResampleOptions {
        let res_opt = unsafe {
            ffms2_sys::FFMS_CreateResampleOptions(self.audio_source)
        };
        let ref_res = unsafe { &*res_opt };

        ResampleOptions::create_struct(ref_res)
    }

    pub fn SetOutputFormatA(&self, options: &ResampleOptions) -> Result<()> {
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
