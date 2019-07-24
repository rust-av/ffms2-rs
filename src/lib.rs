#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utility;

use ffms2_sys::*;

use std::fmt;
use std::ptr;

errors!(Error, FFMS_Errors, ffms_errors,
        (
            ERROR_SUCCESS: "Success.",
            ERROR_INDEX: "Error Index.",
            ERROR_INDEXING: "Error Indexing.",
            ERROR_POSTPROCESSING: "Error Postprocessing.",
            ERROR_SCALING: "Error Scaling.",
            ERROR_DECODING: "Erorr Decoding.",
            ERROR_SEEKING: "Error Seeking.",
            ERROR_PARSER: "Error Parser.",
            ERROR_TRACK: "Error in the track.",
            ERROR_WAVE_WRITER: "Error with the wave writer.",
            ERROR_CANCELLED: "Error cancelled.",
            ERROR_RESAMPLING: "Error resampling.",
            ERROR_UNKNOWN: "Error unknown.",
            ERROR_UNSUPPORTED: "Error unsupported.",
            ERROR_FILE_READ: "Error file read.",
            ERROR_FILE_WRITE: "Error file write.",
            ERROR_NO_FILE: "No file.",
            ERROR_VERSION: "Version error.",
            ERROR_ALLOCATION_FAILED: "Allocation failed.",
            ERROR_INVALID_ARGUMENT: "Invalid argument.",
            ERROR_CODEC: "Error with the codec.",
            ERROR_NOT_AVAILABLE: "Not available.",
            ERROR_FILE_MISMATCH: "File mismatch.",
            ERROR_USER: "Error.",
        )
);

errors!(IndexErrorHandling, FFMS_IndexErrorHandling, ffms_idx_errors,
       (
           IEH_ABORT: "Index error aborting.",
           IEH_CLEAR_TRACK: "Index error clear track.",
           IEH_STOP_TRACK: "Index error stop track.",
           IEH_IGNORE: "Index error ignore.",
       )
);

create_enum!(
    SeekMode,
    FFMS_SeekMode,
    seek_mode,
    (
        SEEK_LINEAR_NO_RW,
        SEEK_LINEAR,
        SEEK_NORMAL,
        SEEK_UNSAFE,
        SEEK_AGGRESSIVE,
    )
);

create_enum!(
    TrackType,
    FFMS_TrackType,
    track_type,
    (
        TYPE_UNKNOWN,
        TYPE_VIDEO,
        TYPE_AUDIO,
        TYPE_DATA,
        TYPE_SUBTITLE,
        TYPE_ATTACHMENT,
    )
);

create_enum!(
    SampleFormat,
    FFMS_SampleFormat,
    sample_format,
    (FMT_U8, FMT_S16, FMT_S32, FMT_FLT, FMT_DBL)
);

create_enum!(
    AudioChannel,
    FFMS_AudioChannel,
    audio_channel,
    (
        CH_FRONT_LEFT,
        CH_FRONT_RIGHT,
        CH_FRONT_CENTER,
        CH_LOW_FREQUENCY,
        CH_BACK_LEFT,
        CH_BACK_RIGHT,
        CH_FRONT_LEFT_OF_CENTER,
        CH_FRONT_RIGHT_OF_CENTER,
        CH_BACK_CENTER,
        CH_SIDE_LEFT,
        CH_SIDE_RIGHT,
        CH_TOP_CENTER,
        CH_TOP_FRONT_LEFT,
        CH_TOP_FRONT_CENTER,
        CH_TOP_FRONT_RIGHT,
        CH_TOP_BACK_LEFT,
        CH_TOP_BACK_CENTER,
        CH_TOP_BACK_RIGHT,
        CH_STEREO_LEFT,
        CH_STEREO_RIGHT,
    )
);

create_enum!(
    Resizers,
    FFMS_Resizers,
    resizers,
    (
        RESIZER_FAST_BILINEAR,
        RESIZER_BILINEAR,
        RESIZER_BICUBIC,
        RESIZER_X,
        RESIZER_POINT,
        RESIZER_AREA,
        RESIZER_BICUBLIN,
        RESIZER_GAUSS,
        RESIZER_SINC,
        RESIZER_LANCZOS,
        RESIZER_SPLINE,
    )
);

create_enum!(
    AudioDelay,
    FFMS_AudioDelayModes,
    audio_delay_modes,
    (DELAY_NO_SHIFT, DELAY_TIME_ZERO, DELAY_FIRST_VIDEO_TRACK)
);

create_enum!(
    ChromaLocations,
    FFMS_ChromaLocations,
    chroma_locations,
    (
        LOC_UNSPECIFIED,
        LOC_LEFT,
        LOC_CENTER,
        LOC_TOPLEFT,
        LOC_TOP,
        LOC_BOTTOMLEFT,
        LOC_BOTTOM,
    )
);

create_enum!(
    ColorRanges,
    FFMS_ColorRanges,
    color_ranges,
    (CR_UNSPECIFIED, CR_MPEG, CR_JPEG)
);

create_enum!(
    Stereo3DType,
    FFMS_Stereo3DType,
    stereo3d_type,
    (
        S3D_TYPE_2D,
        S3D_TYPE_SIDEBYSIDE,
        S3D_TYPE_TOPBOTTOM,
        S3D_TYPE_FRAMESEQUENCE,
        S3D_TYPE_CHECKERBOARD,
        S3D_TYPE_SIDEBYSIDE_QUINCUNX,
        S3D_TYPE_LINES,
        S3D_TYPE_COLUMNS,
    )
);

create_enum!(
    Stereo3DFlags,
    FFMS_Stereo3DFlags,
    stereo3d_flags,
    (S3D_FLAGS_INVERT)
);

create_enum!(
    MixingCoefficientType,
    FFMS_MixingCoefficientType,
    mixing_coefficient_type,
    (
        MIXING_COEFFICIENT_Q8,
        MIXING_COEFFICIENT_Q15,
        MIXING_COEFFICIENT_FLT,
    )
);

create_enum!(
    MatrixEncoding,
    FFMS_MatrixEncoding,
    matrix_encoding,
    (
        MATRIX_ENCODING_NONE,
        MATRIX_ENCODING_DOBLY,
        MATRIX_ENCODING_PRO_LOGIC_II,
        MATRIX_ENCODING_PRO_LOGIC_IIX,
        MATRIX_ENCODING_PRO_LOGIC_IIZ,
        MATRIX_ENCODING_DOLBY_EX,
        MATRIX_ENCODING_DOLBY_HEADPHONE,
    )
);

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

create_enum!(
    LogLevels,
    FFMS_LogLevels,
    log_levels,
    (
        LOG_QUIET,
        LOG_PANIC,
        LOG_FATAL,
        LOG_ERROR,
        LOG_WARNING,
        LOG_INFO,
        LOG_VERBOSE,
        LOG_DEBUG,
        LOG_TRACE,
    )
);

pub struct FFMSError {
    error: FFMS_ErrorInfo,
}

impl FFMSError {
    pub fn new(error_type: i32, sub_type: i32, buffer: &Vec<u8>) -> Self {
        let error = FFMS_ErrorInfo {
            ErrorType: error_type,
            SubType: sub_type,
            BufferSize: buffer.len() as i32,
            Buffer: buffer.as_ptr() as *mut i8,
        };
        FFMSError { error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ffms_errors() {
        assert_eq!(2 + 2, 4);
    }
}
