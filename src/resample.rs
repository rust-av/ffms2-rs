use ffms2_sys::{FFMS_ResampleOptions, FFMS_SampleFormat};

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
}

#[derive(Clone, Copy, Debug)]
pub enum ResampleFilterType {
    Cubic,
    Sinc,
    Kaiser,
}

#[derive(Clone, Copy, Debug)]
pub enum AudioDitherMethod {
    None,
    Rectangular,
    Triangular,
    TriangularHighPass,
    TriangularNoiseShaping,
}

#[derive(Clone, Copy, Debug)]
pub enum MixingCoefficientType {
    Q8,
    Q15,
    Flt,
}

create_struct!(
    ResampleOptions,
    resample,
    FFMS_ResampleOptions,
    (
        ChannelLayout,
        SampleFormat,
        SampleRate,
        MixingCoefficientType,
        CenterMixLevel,
        SurroundMixLevel,
        LFEMixLevel,
        Normalize,
        ForceResample,
        ResampleFilterSize,
        ResamplePhaseShift,
        LinearInterpolation,
        CutoffFrequencyRatio,
        MatrixedStereoEncoding,
        FilterType,
        KaiserBeta,
        DitherMethod
    ),
    (
        0,
        FFMS_SampleFormat::FFMS_FMT_U8,
        0,
        ffms2_sys::FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_Q8,
        0.0,
        0.0,
        0.0,
        0,
        0,
        0,
        0,
        0,
        0.0,
        ffms2_sys::FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_NONE,
        ffms2_sys::FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_CUBIC,
        0,
        ffms2_sys::FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_NONE
    )
);

impl ResampleOptions {
    pub fn set_channel_layout(&mut self, channel_layout: i64) {
        self.resample.ChannelLayout = channel_layout;
    }

    pub fn set_sample_format(&mut self, sample_format: &SampleFormat) {
        self.resample.SampleFormat =
            SampleFormat::ffms2_sample_format(*sample_format);
    }

    pub fn normalize(&mut self, normalize: bool) {
        self.resample.Normalize = normalize as i32;
    }

    pub(crate) fn create_struct(resample: &FFMS_ResampleOptions) -> Self {
        ResampleOptions {
            resample: *resample,
        }
    }

    pub(crate) fn as_ptr(&self) -> *const FFMS_ResampleOptions {
        &self.resample
    }
}

impl Drop for ResampleOptions {
    fn drop(&mut self) {
        let raw_resample = Box::into_raw(Box::new(self.resample));
        unsafe {
            ffms2_sys::FFMS_DestroyResampleOptions(raw_resample);
        }
    }
}
