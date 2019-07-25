use crate::*;

create_enum!(
    ResampleFilterType,
    FFMS_ResampleFilterType,
    resample_filter_type,
    (
        RESAMPLE_FILTER_CUBIC,
        RESAMPLE_FILTER_SINC,
        RESAMPLE_FILTER_KAISER,
    )
);

create_enum!(
    AudioDitherMethod,
    FFMS_AudioDitherMethod,
    audio_dither_method,
    (
        RESAMPLE_DITHER_NONE,
        RESAMPLE_DITHER_RECTANGULAR,
        RESAMPLE_DITHER_TRIANGULAR,
        RESAMPLE_DITHER_TRIANGULAR_HIGHPASS,
        RESAMPLE_DITHER_TRIANGULAR_NOISESHAPING,
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
        usize,
        &SampleFormat,
        usize,
        &MixingCoefficientType,
        f32,
        f32,
        f32,
        usize,
        usize,
        usize,
        usize,
        usize,
        f32,
        &MatrixEncoding,
        &ResampleFilterType,
        usize,
        &AudioDitherMethod
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
    ),
    (
        ChannelLayout as i64,
        SampleFormat::to_sample_format(SampleFormat),
        SampleRate as i32,
        MixingCoefficientType::to_mixing_coefficient_type(MixingCoefficientType),
        CenterMixLevel as f64,
        SurroundMixLevel as f64,
        LFEMixLevel as f64,
        Normalize as i32,
        ForceResample as i32,
        ResampleFilterSize as i32,
        ResamplePhaseShift as i32,
        LinearInterpolation as i32,
        CutoffFrequencyRatio as f64,
        MatrixEncoding::to_matrix_encoding(MatrixedStereoEncoding),
        ResampleFilterType::to_resample_filter_type(FilterType),
        KaiserBeta as i32,
        AudioDitherMethod::to_audio_dither_method(DitherMethod)
    )
);
