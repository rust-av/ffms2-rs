use ffms2_sys::{
    FFMS_AudioDitherMethod, FFMS_MixingCoefficientType,
    FFMS_ResampleFilterType, FFMS_ResampleOptions, FFMS_SampleFormat,
};

use crate::audio::MatrixEncoding;

#[derive(Clone, Copy, Debug)]
pub enum SampleFormat {
    U8,
    S16,
    S32,
    Flt,
    Dbl,
}

impl SampleFormat {
    const fn ffms2_sample_format(self) -> FFMS_SampleFormat {
        match self {
            Self::U8 => FFMS_SampleFormat::FFMS_FMT_U8,
            Self::S16 => FFMS_SampleFormat::FFMS_FMT_S16,
            Self::S32 => FFMS_SampleFormat::FFMS_FMT_S32,
            Self::Flt => FFMS_SampleFormat::FFMS_FMT_FLT,
            Self::Dbl => FFMS_SampleFormat::FFMS_FMT_DBL,
        }
    }

    const fn new(ffms2_sample_format: FFMS_SampleFormat) -> Self {
        match ffms2_sample_format {
            FFMS_SampleFormat::FFMS_FMT_U8 => Self::U8,
            FFMS_SampleFormat::FFMS_FMT_S16 => Self::S16,
            FFMS_SampleFormat::FFMS_FMT_S32 => Self::S32,
            FFMS_SampleFormat::FFMS_FMT_FLT => Self::Flt,
            FFMS_SampleFormat::FFMS_FMT_DBL => Self::Dbl,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ResampleFilterType {
    Cubic,
    Sinc,
    Kaiser,
}

impl ResampleFilterType {
    const fn new(resample_filter_type: FFMS_ResampleFilterType) -> Self {
        match resample_filter_type {
            FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_CUBIC => Self::Cubic,
            FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_SINC => Self::Sinc,
            FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_KAISER => {
                Self::Kaiser
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AudioDitherMethod {
    None,
    Rectangular,
    Triangular,
    TriangularHighPass,
    TriangularNoiseShaping,
}

impl AudioDitherMethod {
    const fn new(audio_dither_method: FFMS_AudioDitherMethod) -> Self {
        match audio_dither_method {
            FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_NONE => Self::None,
            FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_RECTANGULAR => Self::Rectangular,
            FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_TRIANGULAR => Self::Triangular,
            FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_TRIANGULAR_HIGHPASS => Self::TriangularHighPass,
            FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_TRIANGULAR_NOISESHAPING => Self::TriangularNoiseShaping,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MixingCoefficientType {
    Q8,
    Q15,
    Flt,
}

impl MixingCoefficientType {
    const fn new(mixing_coefficient_type: FFMS_MixingCoefficientType) -> Self {
        match mixing_coefficient_type {
            FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_Q8 => Self::Q8,
            FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_Q15 => {
                Self::Q15
            }
            FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_FLT => {
                Self::Flt
            }
        }
    }
}

#[derive(Debug)]
pub struct ResampleOptions(FFMS_ResampleOptions);

unsafe impl Send for ResampleOptions {}

impl ResampleOptions {
    pub const fn channel_layout(&self) -> usize {
        self.0.ChannelLayout as usize
    }

    pub const fn sample_format(&self) -> SampleFormat {
        SampleFormat::new(self.0.SampleFormat)
    }

    pub const fn sample_rate(&self) -> usize {
        self.0.SampleRate as usize
    }

    pub const fn mixing_coefficient_type(&self) -> MixingCoefficientType {
        MixingCoefficientType::new(self.0.MixingCoefficientType)
    }

    pub const fn center_mix_level(&self) -> f64 {
        self.0.CenterMixLevel
    }

    pub const fn surround_mix_level(&self) -> f64 {
        self.0.SurroundMixLevel
    }

    pub const fn lfe_mix_level(&self) -> f64 {
        self.0.LFEMixLevel
    }

    pub const fn normalize(&self) -> usize {
        self.0.Normalize as usize
    }

    pub const fn force_resample(&self) -> usize {
        self.0.ForceResample as usize
    }

    pub const fn resample_filter_size(&self) -> usize {
        self.0.ResampleFilterSize as usize
    }

    pub const fn resample_phase_shift(&self) -> usize {
        self.0.ResamplePhaseShift as usize
    }

    pub const fn linear_interpolation(&self) -> usize {
        self.0.LinearInterpolation as usize
    }

    pub const fn cutoff_frequency_ratio(&self) -> usize {
        self.0.CutoffFrequencyRatio as usize
    }

    pub const fn matrix_stereo_encoding(&self) -> MatrixEncoding {
        MatrixEncoding::new(self.0.MatrixedStereoEncoding)
    }

    pub const fn resample_filter_type(&self) -> ResampleFilterType {
        ResampleFilterType::new(self.0.FilterType)
    }

    pub const fn kaiser_beta(&self) -> usize {
        self.0.KaiserBeta as usize
    }

    pub const fn audio_dither_method(&self) -> AudioDitherMethod {
        AudioDitherMethod::new(self.0.DitherMethod)
    }

    pub fn set_channel_layout(&mut self, channel_layout: i64) {
        self.0.ChannelLayout = channel_layout;
    }

    pub fn set_sample_format(&mut self, sample_format: &SampleFormat) {
        self.0.SampleFormat =
            SampleFormat::ffms2_sample_format(*sample_format);
    }

    pub fn set_normalize(&mut self, normalize: bool) {
        self.0.Normalize = normalize as i32;
    }

    pub(crate) fn create_struct(resample: &FFMS_ResampleOptions) -> Self {
        ResampleOptions(*resample)
    }

    pub(crate) fn as_ptr(&self) -> *const FFMS_ResampleOptions {
        &self.0
    }
}

impl Drop for ResampleOptions {
    fn drop(&mut self) {
        let raw_resample = Box::into_raw(Box::new(self.0));
        unsafe {
            ffms2_sys::FFMS_DestroyResampleOptions(raw_resample);
        }
    }
}
