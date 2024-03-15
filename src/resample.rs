use ffms2_sys::{
    FFMS_AudioDitherMethod, FFMS_MatrixEncoding, FFMS_MixingCoefficientType,
    FFMS_ResampleFilterType, FFMS_ResampleOptions, FFMS_SampleFormat,
};

use crate::audio::AudioChannel;

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
    const fn new(matrix_encoding: FFMS_MatrixEncoding) -> Self {
        match matrix_encoding {
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_NONE => Self::None,
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_DOBLY => Self::Dolby,
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_PRO_LOGIC_II => {
                Self::ProLogicII
            }
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_PRO_LOGIC_IIX => {
                Self::ProLogicIIX
            }
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_PRO_LOGIC_IIZ => {
                Self::ProLogicIIZ
            }
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_DOLBY_EX => {
                Self::DolbyEx
            }
            FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_DOLBY_HEADPHONE => {
                Self::DolbyHeadphone
            }
        }
    }

    const fn into_ffms2(self) -> FFMS_MatrixEncoding {
        match self {
            Self::None => FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_NONE,
            Self::Dolby => FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_DOBLY,
            Self::ProLogicII => {
                FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_PRO_LOGIC_II
            }
            Self::ProLogicIIX => {
                FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_PRO_LOGIC_IIX
            }
            Self::ProLogicIIZ => {
                FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_PRO_LOGIC_IIZ
            }
            Self::DolbyEx => {
                FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_DOLBY_EX
            }
            Self::DolbyHeadphone => {
                FFMS_MatrixEncoding::FFMS_MATRIX_ENCODING_DOLBY_HEADPHONE
            }
        }
    }
}

/// Audio sample formats.
#[derive(Clone, Copy, Debug)]
pub enum SampleFormat {
    /// One 8-bit unsigned integer per sample.
    U8,
    /// One 16-bit signed integer per sample.
    S16,
    /// One 32-bit signed integer per sample.
    S32,
    /// One 32-bit (single precision) floating point value per sample.
    Flt,
    /// One 64-bit (double precision) floating point value per sample.
    Dbl,
}

impl SampleFormat {
    pub(crate) const fn new(ffms2_sample_format: usize) -> Option<Self> {
        match ffms2_sample_format {
            e if e == FFMS_SampleFormat::FFMS_FMT_U8 as usize => {
                Some(Self::U8)
            }
            e if e == FFMS_SampleFormat::FFMS_FMT_S16 as usize => {
                Some(Self::S16)
            }
            e if e == FFMS_SampleFormat::FFMS_FMT_S32 as usize => {
                Some(Self::S32)
            }
            e if e == FFMS_SampleFormat::FFMS_FMT_FLT as usize => {
                Some(Self::Flt)
            }
            e if e == FFMS_SampleFormat::FFMS_FMT_DBL as usize => {
                Some(Self::Dbl)
            }
            _ => None,
        }
    }

    const fn into_ffms2(self) -> FFMS_SampleFormat {
        match self {
            Self::U8 => FFMS_SampleFormat::FFMS_FMT_U8,
            Self::S16 => FFMS_SampleFormat::FFMS_FMT_S16,
            Self::S32 => FFMS_SampleFormat::FFMS_FMT_S32,
            Self::Flt => FFMS_SampleFormat::FFMS_FMT_FLT,
            Self::Dbl => FFMS_SampleFormat::FFMS_FMT_DBL,
        }
    }
}

/// Resampling Filter Types.
#[derive(Clone, Copy, Debug)]
pub enum ResampleFilterType {
    /// Cubic.
    Cubic,
    /// Blackman Nuttall Windowed Sinc.
    Sinc,
    /// Kaiser Windowed Sinc.
    ///
    /// The input parameter is the beta value for Kaiser window.
    ///
    /// Must be a double float value in the interval [2,16], default value is 9.
    Kaiser(usize),
}

impl ResampleFilterType {
    const fn new(
        resample_filter_type: FFMS_ResampleFilterType,
        kaiser_beta: usize,
    ) -> Self {
        match resample_filter_type {
            FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_CUBIC => Self::Cubic,
            FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_SINC => Self::Sinc,
            FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_KAISER => {
                Self::Kaiser(kaiser_beta)
            }
        }
    }

    const fn into_ffms2(self) -> (i32, FFMS_ResampleFilterType) {
        match self {
            Self::Cubic => {
                (0, FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_CUBIC)
            }
            Self::Sinc => {
                (0, FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_SINC)
            }
            Self::Kaiser(kaiser_beta) => (
                kaiser_beta as i32,
                FFMS_ResampleFilterType::FFMS_RESAMPLE_FILTER_KAISER,
            ),
        }
    }
}

/// Audio dither method.
#[derive(Clone, Copy, Debug)]
pub enum AudioDitherMethod {
    /// Do not use dithering.
    None,
    /// Rectangular.
    Rectangular,
    /// Triangular.
    Triangular,
    /// Triangular dither with High Pass.
    TriangularHighPass,
    /// Triangular Dither with Noise Shaping.
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

    const fn into_ffms2(self) -> FFMS_AudioDitherMethod {
        match self {
            Self::None => FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_NONE,
            Self::Rectangular => FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_RECTANGULAR,
            Self::Triangular => FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_TRIANGULAR,
            Self::TriangularHighPass => FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_TRIANGULAR_HIGHPASS,
            Self::TriangularNoiseShaping => FFMS_AudioDitherMethod::FFMS_RESAMPLE_DITHER_TRIANGULAR_NOISESHAPING,
        }
    }
}

/// Channel Mixing Matrix Coefficient Types.
#[derive(Clone, Copy, Debug)]
pub enum MixingCoefficientType {
    /// 8 bit.
    Q8,
    /// 16-bit, 8.8 fixed-point.
    Q15,
    /// 32-bit, 17.15 fixed-point.
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

    const fn into_ffms2(self) -> FFMS_MixingCoefficientType {
        match self {
            Self::Q8 => FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_Q8,
            Self::Q15 => {
                FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_Q15
            }
            Self::Flt => {
                FFMS_MixingCoefficientType::FFMS_MIXING_COEFFICIENT_FLT
            }
        }
    }
}

#[derive(Debug)]
pub struct ResampleOptions {
    /// Audio stream channel layout.
    ///
    /// If `None`, no channel layout has been found.
    pub channel_layout: Option<Vec<AudioChannel>>,
    /// Audio stream sample format.
    ///
    /// If `None`, no sample format has been found.
    pub sample_format: Option<SampleFormat>,
    /// Audio stream sample rate.
    pub sample_rate: usize,
    /// Channel mixing coefficient types.
    pub mixing_coefficient_type: MixingCoefficientType,
    /// Center Mix Level.
    ///
    /// It is a value expressed in dB, and must be in the interval [-32,32].
    pub center_mix_level: f64,
    /// Surround Mix Level.
    ///
    /// It is a value expressed in dB, and must be in the interval [-32,32].
    pub surround_mix_level: f64,
    /// Low Frequency Effects (LFE) mix into non LFE level.
    ///
    /// It is used when there is a LFE input, but no LFE output.
    ///
    /// It is a value expressed in dB, and must be in the interval [-32,32].
    pub lfe_mix_level: f64,
    /// Mix level normalization.
    pub normalize: usize,
    /// Force resampling.
    pub force_resample: bool,
    /// Length of each FIR filter in the resampling filterbank relative to the
    /// cutoff frequency.
    pub filter_size: usize,
    /// Binary logarithm of the number of entries in the
    /// resampling polyphase filterbank.
    pub phase_shift: usize,
    /// If `true`, then the resampling FIR filter will be linearly interpolated.
    pub linear_interpolation: bool,
    /// Resampling cutoff frequency.
    pub cutoff_frequency_ratio: f64,
    /// Matrixed stereo encoding.
    pub matrix_stereo_encoding: MatrixEncoding,
    /// Resampling filter type.
    pub filter_type: ResampleFilterType,
    /// Audio dither method.
    pub audio_dither_method: AudioDitherMethod,
}

unsafe impl Send for ResampleOptions {}

impl ResampleOptions {
    pub(crate) fn new(resample: FFMS_ResampleOptions) -> Self {
        Self {
            channel_layout: AudioChannel::channel_map(resample.ChannelLayout),
            sample_format: SampleFormat::new(resample.SampleFormat as usize),
            sample_rate: resample.SampleRate as usize,
            mixing_coefficient_type: MixingCoefficientType::new(
                resample.MixingCoefficientType,
            ),
            center_mix_level: resample.CenterMixLevel,
            surround_mix_level: resample.SurroundMixLevel,
            lfe_mix_level: resample.LFEMixLevel,
            normalize: resample.Normalize as usize,
            force_resample: resample.ForceResample > 0,
            filter_size: resample.ResampleFilterSize as usize,
            phase_shift: resample.ResamplePhaseShift as usize,
            linear_interpolation: resample.LinearInterpolation > 0,
            cutoff_frequency_ratio: resample.CutoffFrequencyRatio,
            matrix_stereo_encoding: MatrixEncoding::new(
                resample.MatrixedStereoEncoding,
            ),
            filter_type: ResampleFilterType::new(
                resample.FilterType,
                resample.KaiserBeta as usize,
            ),
            audio_dither_method: AudioDitherMethod::new(resample.DitherMethod),
        }
    }

    pub(crate) fn ffms2_resample(
        &self,
        channel_layout: &[AudioChannel],
        sample_format: SampleFormat,
    ) -> FFMS_ResampleOptions {
        let (kaiser_beta, filter_type) = self.filter_type.into_ffms2();
        FFMS_ResampleOptions {
            ChannelLayout: AudioChannel::into_ffms2(channel_layout),
            SampleFormat: sample_format.into_ffms2(),
            SampleRate: self.sample_rate as i32,
            MixingCoefficientType: self.mixing_coefficient_type.into_ffms2(),
            CenterMixLevel: self.center_mix_level,
            SurroundMixLevel: self.surround_mix_level,
            LFEMixLevel: self.lfe_mix_level,
            Normalize: self.normalize as i32,
            ForceResample: if self.force_resample { 1 } else { 0 },
            ResampleFilterSize: self.filter_size as i32,
            ResamplePhaseShift: self.phase_shift as i32,
            LinearInterpolation: if self.linear_interpolation { 1 } else { 0 },
            CutoffFrequencyRatio: self.cutoff_frequency_ratio,
            MatrixedStereoEncoding: self.matrix_stereo_encoding.into_ffms2(),
            KaiserBeta: kaiser_beta,
            FilterType: filter_type,
            DitherMethod: self.audio_dither_method.into_ffms2(),
        }
    }
}
