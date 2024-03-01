use std::ffi::CString;
use std::path::Path;

use ffms2_sys::{FFMS_ColorRanges, FFMS_SeekMode, FFMS_VideoProperties};

use crate::error::{InternalError, Result};
use crate::frame::Resizers;
use crate::index::Index;

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

create_struct!(
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
        Rotation,
        Stereo3DType,
        Stereo3DFlags,
        LastEndTime,
        HasMasteringDisplayPrimaries,
        MasteringDisplayPrimariesX,
        MasteringDisplayPrimariesY,
        MasteringDisplayWhitePointX,
        MasteringDisplayWhitePointY,
        HasMasteringDisplayLuminance,
        MasteringDisplayMinLuminance,
        MasteringDisplayMaxLuminance,
        HasContentLightLevel,
        ContentLightLevelMax,
        ContentLightLevelAverage,
        Flip
    ),
    (
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0, 0.0, 0, 0, 0, 0.0, 0,
        [0.0; 3], [0.0; 3], 0.0, 0.0, 0, 0.0, 0.0, 0, 0, 0, 0
    )
);

pub struct VideoSource {
    video_source: *mut ffms2_sys::FFMS_VideoSource,
}

unsafe impl Send for VideoSource {}

impl VideoSource {
    pub fn new(
        SourceFile: &Path,
        Track: usize,
        Index: &Index,
        Threads: usize,
        SeekMode: SeekMode,
    ) -> Result<Self> {
        let source = CString::new(SourceFile.to_str().unwrap()).unwrap();
        let mut error = InternalError::new();
        let seek = SeekMode::to_seek_mode(SeekMode) as i32;
        let video_source = unsafe {
            ffms2_sys::FFMS_CreateVideoSource(
                source.as_ptr(),
                Track as i32,
                Index.as_mut_ptr(),
                Threads as i32,
                seek,
                error.as_mut_ptr(),
            )
        };

        if video_source.is_null() {
            Err(error.into())
        } else {
            Ok(VideoSource { video_source })
        }
    }

    pub fn GetVideoProperties(&self) -> VideoProperties {
        let video_prop =
            unsafe { ffms2_sys::FFMS_GetVideoProperties(self.video_source) };
        let ref_video = unsafe { &*video_prop };

        VideoProperties {
            video_properties: *ref_video,
        }
    }

    pub fn SetInputFormatV(
        &self,
        ColorSpace: usize,
        ColorRange: ColorRanges,
        PixelFormat: usize,
    ) -> Result<()> {
        let mut error = InternalError::new();
        let colorange = ColorRanges::to_color_ranges(ColorRange) as i32;
        let err = unsafe {
            ffms2_sys::FFMS_SetInputFormatV(
                self.video_source,
                ColorSpace as i32,
                colorange,
                PixelFormat as i32,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub fn ResetInputFormatV(&self) {
        unsafe {
            ffms2_sys::FFMS_ResetInputFormatV(self.video_source);
        }
    }

    pub fn SetOutputFormatV2(
        &self,
        TargetFormats: &mut Vec<i32>,
        Width: usize,
        Height: usize,
        Resizer: Resizers,
    ) -> Result<()> {
        let mut error = InternalError::new();
        let resize = Resizers::to_resizers(Resizer) as i32;
        TargetFormats.push(-1);
        let err = unsafe {
            ffms2_sys::FFMS_SetOutputFormatV2(
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
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub fn ResetOutputFormatV(&self) {
        unsafe {
            ffms2_sys::FFMS_ResetOutputFormatV(self.video_source);
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffms2_sys::FFMS_VideoSource {
        self.video_source
    }
}

impl Drop for VideoSource {
    fn drop(&mut self) {
        unsafe {
            ffms2_sys::FFMS_DestroyVideoSource(self.video_source);
        }
    }
}
