use crate::*;

simple_enum!(
    ResampleFilterType,
    (
        RESAMPLE_FILTER_CUBIC,
        RESAMPLE_FILTER_SINC,
        RESAMPLE_FILTER_KAISER,
    )
);

simple_enum!(
    AudioDitherMethod,
    (
        RESAMPLE_DITHER_NONE,
        RESAMPLE_DITHER_RECTANGULAR,
        RESAMPLE_DITHER_TRIANGULAR,
        RESAMPLE_DITHER_TRIANGULAR_HIGHPASS,
        RESAMPLE_DITHER_TRIANGULAR_NOISESHAPING,
    )
);

simple_enum!(
    MixingCoefficientType,
    (
        MIXING_COEFFICIENT_Q8,
        MIXING_COEFFICIENT_Q15,
        MIXING_COEFFICIENT_FLT,
    )
);

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
        FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_Q8,
        0.0,
        0.0,
        0.0,
        0,
        0,
        0,
        0,
        0,
        0.0,
        FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_NONE,
        FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_CUBIC,
        0,
        FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_NONE
    )
);

impl ResampleOptions {
    pub(crate) fn create_struct(resample: &FFMS_ResampleOptions) -> Self {
        ResampleOptions {
            resample: *resample,
        }
    }

    pub(crate) fn as_ptr(&self) -> *const FFMS_ResampleOptions {
        &self.resample
    }

    pub fn set_channel_layout(&mut self, channel_layout: i64) {
        self.resample.ChannelLayout = channel_layout;
    }

    pub fn set_sample_format(&mut self, sample_format: &SampleFormat) {
        self.resample.SampleFormat =
            SampleFormat::to_sample_format(sample_format);
    }

    pub fn normalize(&mut self, normalize: bool) {
        self.resample.Normalize = normalize as i32;
    }
}

impl Drop for ResampleOptions {
    fn drop(&mut self) {
        let raw_resample = Box::into_raw(Box::new(self.resample));
        unsafe {
            FFMS_DestroyResampleOptions(raw_resample);
        }
    }
}
