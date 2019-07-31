use crate::frame::*;
use crate::index::*;
use crate::*;

use ffms2_sys::*;

use std::ffi::CString;
use std::path::PathBuf;

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

simple_enum!(
    Stereo3DType,
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

simple_enum!(Stereo3DFlags, (S3D_FLAGS_INVERT));

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
    (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0, 0.0),
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
    (
        usize, usize, usize, usize, usize, usize, usize, usize, usize, usize,
        usize, usize, usize, usize, f64, f64
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
        (cfg(feature = "ffms2-2-31-0"), Flip, usize, Flip as i32)
    )
);

pub struct VideoSource {
    video_source: *mut FFMS_VideoSource,
}

impl VideoSource {
    pub fn new(
        SourceFile: &PathBuf,
        Track: usize,
        Index: &Index,
        Threads: usize,
        SeekMode: SeekMode,
    ) -> Result<Self, Error> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error: Error = Default::default();
        let seek = SeekMode::to_seek_mode(&SeekMode) as i32;
        let video_source = unsafe {
            FFMS_CreateVideoSource(
                source.as_ptr(),
                Track as i32,
                Index.as_mut_ptr(),
                Threads as i32,
                seek,
                error.as_mut_ptr(),
            )
        };

        if video_source.is_null() {
            Err(error)
        } else {
            Ok(VideoSource { video_source })
        }
    }

    pub fn GetVideoProperties(&self) -> VideoProperties {
        let video_prop = unsafe { FFMS_GetVideoProperties(self.video_source) };
        let ref_video = unsafe { &*video_prop };

        VideoProperties {
            video_properties: *ref_video,
        }
    }

    #[cfg(feature = "ffms2-2-17-1")]
    pub fn SetInputFormatV(
        &self,
        ColorSpace: usize,
        ColorRange: ColorRanges,
        PixelFormat: usize,
    ) -> Result<(), Error> {
        let mut error: Error = Default::default();
        let colorange = ColorRanges::to_color_ranges(&ColorRange) as i32;
        let err = unsafe {
            FFMS_SetInputFormatV(
                self.video_source,
                ColorSpace as i32,
                colorange,
                PixelFormat as i32,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn ResetInputFormatV(&self) {
        unsafe {
            FFMS_ResetInputFormatV(self.video_source);
        }
    }

    #[cfg(feature = "ffms2-2-15-3")]
    pub fn SetOutputFormatV2(
        &self,
        TargetFormats: &mut Vec<i32>,
        Width: usize,
        Height: usize,
        Resizer: Resizers,
    ) -> Result<(), Error> {
        let mut error: Error = Default::default();
        let resize = Resizers::to_resizers(&Resizer) as i32;
        TargetFormats.push(-1);
        let err = unsafe {
            FFMS_SetOutputFormatV2(
                self.video_source,
                TargetFormats.as_ptr(),
                Width as i32,
                Height as i32,
                resize,
                error.as_mut_ptr(),
            )
        };
        TargetFormats.pop();

        if err != 0 {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn ResetOutputFormatV(&self) {
        unsafe {
            FFMS_ResetOutputFormatV(self.video_source);
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut FFMS_VideoSource {
        self.video_source
    }
}

impl Drop for VideoSource {
    fn drop(&mut self) {
        unsafe {
            FFMS_DestroyVideoSource(self.video_source);
        }
    }
}
