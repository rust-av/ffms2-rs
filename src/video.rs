use crate::*;

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
    ColorRanges,
    FFMS_ColorRanges,
    color_ranges,
    (CR_UNSPECIFIED, CR_MPEG, CR_JPEG)
);

set_struct!(VideoProperties, video_properties, FFMS_VideoProperties);

default_struct!(
    VideoProperties,
    video_properties,
    FFMS_VideoProperties,
    (
        FPSDenominator,
        FPSNumerator,
        RFFDenominator,
        RFFNumerator,
        NumFrames,
        SARNum,
        SARDen,
        CropTop,
        CropBottom,
        CropLeft,
        CropRight,
        TopFieldFirst,
        ColorSpace,
        ColorRange,
        FirstTime,
        LastTime,
    ),
    (
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0.0, 0.0
    ),
    (
        (cfg(feature = "ffms2-2-24-0"), Rotation, 0),
        (cfg(feature = "ffms2-2-24-0"), Stereo3DType, 0),
        (cfg(feature = "ffms2-2-24-0"), Stereo3DFlags, 0),
        (cfg(feature = "ffms2-2-30-0"), LastEndTime, 0.0),
        (
            cfg(feature = "ffms2-2-30-0"),
            HasMasteringDisplayPrimaries,
            0
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayPrimariesX,
            [0.0; 3]
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayPrimariesY,
            [0.0; 3]
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayWhitePointX,
            0.0
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayWhitePointY,
            0.0
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            HasMasteringDisplayLuminance,
            0
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayMinLuminance,
            0.0
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayMaxLuminance,
            0.0
        ),
        (cfg(feature = "ffms2-2-30-0"), HasContentLightLevel, 0),
        (cfg(feature = "ffms2-2-30-0"), ContentLightLevelMax, 0),
        (cfg(feature = "ffms2-2-30-0"), ContentLightLevelAverage, 0),
        (cfg(feature = "ffms2-2-31-0"), Flip, 0)
    )
);

set_params!(
    VideoProperties,
    video_properties,
    (
        FPSDenominator,
        FPSNumerator,
        RFFDenominator,
        RFFNumerator,
        NumFrames,
        SARNum,
        SARDen,
        CropTop,
        CropBottom,
        CropLeft,
        CropRight,
        TopFieldFirst,
        ColorSpace,
        ColorRange,
        FirstTime,
        LastTime,
    ),
    (   usize, usize, usize, usize, usize, usize, usize, usize,
        usize, usize, usize, usize, usize, usize, f64, f64
    ),
    (
        FPSDenominator as i32,
        FPSNumerator as i32,
        RFFDenominator as i32,
        RFFNumerator as i32,
        NumFrames as i32,
        SARNum as i32,
        SARDen as i32,
        CropTop as i32,
        CropBottom as i32,
        CropLeft as i32,
        CropRight as i32,
        TopFieldFirst as i32,
        ColorSpace as i32,
        ColorRange as i32,
        FirstTime as f64,
        LastTime as f64,
    )
);

set_feature_params!(
    VideoProperties,
    video_properties,
    (
        (
            cfg(feature = "ffms2-2-24-0"),
            Rotation,
            usize,
            Rotation as i32
        ),
        (
            cfg(feature = "ffms2-2-24-0"),
            Stereo3DType,
            usize,
            Stereo3DType as i32
        ),
        (
            cfg(feature = "ffms2-2-24-0"),
            Stereo3DFlags,
            usize,
            Stereo3DFlags as i32
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            LastEndTime,
            f64,
            LastEndTime as f64
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            HasMasteringDisplayPrimaries,
            usize,
            HasMasteringDisplayPrimaries as i32
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayPrimariesX,
            &[f64; 3],
            *MasteringDisplayPrimariesX as [f64; 3]
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayPrimariesY,
            &[f64; 3],
            *MasteringDisplayPrimariesY as [f64; 3]
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayWhitePointX,
            f64,
            MasteringDisplayWhitePointX as f64
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayWhitePointY,
            f64,
            MasteringDisplayWhitePointY as f64
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            HasMasteringDisplayLuminance,
            usize,
            HasMasteringDisplayLuminance as i32
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayMinLuminance,
            f64,
            MasteringDisplayMinLuminance as f64
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            MasteringDisplayMaxLuminance,
            f64,
            MasteringDisplayMaxLuminance as f64
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            HasContentLightLevel,
            usize,
            HasContentLightLevel as i32
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            ContentLightLevelMax,
            usize,
            ContentLightLevelMax as u32
        ),
        (
            cfg(feature = "ffms2-2-30-0"),
            ContentLightLevelAverage,
            usize,
            ContentLightLevelAverage as u32
        ),
        (
            cfg(feature = "ffms2-2-31-0"),
            Flip,
            usize,
            Flip as i32
        )
    )
);
