use std::ffi::CString;
use std::path::Path;

use ffms2_sys::{FFMS_ColorRanges, FFMS_SeekMode, FFMS_VideoProperties};

use crate::error::{InternalError, Result};
use crate::frame::Resizers;
use crate::index::Index;

#[derive(Clone, Copy, Debug)]
pub enum SeekMode {
    LinearNoRW,
    Linear,
    Normal,
    Unsafe,
    Aggressive,
}

impl SeekMode {
    pub(crate) const fn ffms2_seek_mode(self) -> FFMS_SeekMode {
        match self {
            Self::LinearNoRW => FFMS_SeekMode::FFMS_SEEK_LINEAR_NO_RW,
            Self::Linear => FFMS_SeekMode::FFMS_SEEK_LINEAR,
            Self::Normal => FFMS_SeekMode::FFMS_SEEK_NORMAL,
            Self::Unsafe => FFMS_SeekMode::FFMS_SEEK_UNSAFE,
            Self::Aggressive => FFMS_SeekMode::FFMS_SEEK_AGGRESSIVE,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Stereo3DType {
    TwoDimensional,
    SideBySide,
    TopBottom,
    FrameSequence,
    CheckerBoard,
    SideBySideQuincunx,
    Lines,
    Columns,
}

#[derive(Clone, Copy, Debug)]
pub enum Stereo3DFlags {
    FlagsInvert,
}

#[derive(Clone, Copy, Debug)]
pub enum ColorRange {
    Unspecified,
    Mpeg,
    Jpeg,
}

impl ColorRange {
    pub(crate) const fn ffms2_color_ranges(self) -> FFMS_ColorRanges {
        match self {
            Self::Unspecified => FFMS_ColorRanges::FFMS_CR_UNSPECIFIED,
            Self::Mpeg => FFMS_ColorRanges::FFMS_CR_MPEG,
            Self::Jpeg => FFMS_ColorRanges::FFMS_CR_JPEG,
        }
    }
}

#[derive(Debug)]
pub struct VideoProperties(FFMS_VideoProperties);

impl VideoProperties {
    pub const fn fps_numerator(&self) -> usize {
        self.0.FPSNumerator as usize
    }

    pub const fn fps_denominator(&self) -> usize {
        self.0.FPSDenominator as usize
    }

    pub const fn rff_numerator(&self) -> usize {
        self.0.RFFNumerator as usize
    }

    pub const fn rff_denominator(&self) -> usize {
        self.0.RFFDenominator as usize
    }

    pub const fn frames_number(&self) -> usize {
        self.0.NumFrames as usize
    }

    pub const fn sar_numerator(&self) -> usize {
        self.0.SARNum as usize
    }

    pub const fn sar_denominator(&self) -> usize {
        self.0.SARDen as usize
    }

    pub const fn crop_top(&self) -> i32 {
        self.0.CropTop
    }

    pub const fn crop_bottom(&self) -> i32 {
        self.0.CropBottom
    }

    pub const fn crop_left(&self) -> i32 {
        self.0.CropLeft
    }

    pub const fn crop_right(&self) -> i32 {
        self.0.CropRight
    }

    pub const fn top_field_first(&self) -> i32 {
        self.0.TopFieldFirst
    }

    pub const fn colorspace(&self) -> i32 {
        self.0.ColorSpace
    }

    pub const fn color_range(&self) -> i32 {
        self.0.ColorRange
    }

    pub const fn first_time(&self) -> f64 {
        self.0.FirstTime
    }

    pub const fn last_time(&self) -> f64 {
        self.0.LastTime
    }

    pub const fn rotation(&self) -> i32 {
        self.0.Rotation
    }

    pub const fn stereo3d_type(&self) -> i32 {
        self.0.Stereo3DType
    }

    pub const fn stereo3d_flags(&self) -> i32 {
        self.0.Stereo3DFlags
    }

    pub const fn last_end_time(&self) -> f64 {
        self.0.LastEndTime
    }

    pub const fn has_mastering_display_primaries(&self) -> i32 {
        self.0.HasMasteringDisplayPrimaries
    }

    pub const fn mastering_display_primaries_x(&self) -> [f64; 3] {
        self.0.MasteringDisplayPrimariesX
    }

    pub const fn mastering_display_primaries_y(&self) -> [f64; 3] {
        self.0.MasteringDisplayPrimariesY
    }

    pub const fn mastering_display_white_point_x(&self) -> f64 {
        self.0.MasteringDisplayWhitePointX
    }

    pub const fn mastering_display_white_point_y(&self) -> f64 {
        self.0.MasteringDisplayWhitePointY
    }

    pub const fn has_mastering_display_luminance(&self) -> i32 {
        self.0.HasMasteringDisplayLuminance
    }

    pub const fn mastering_display_min_luminance(&self) -> f64 {
        self.0.MasteringDisplayMinLuminance
    }

    pub const fn mastering_display_max_luminance(&self) -> f64 {
        self.0.MasteringDisplayMaxLuminance
    }

    pub const fn has_content_light_level(&self) -> i32 {
        self.0.HasContentLightLevel
    }

    pub const fn content_light_level_max(&self) -> u32 {
        self.0.ContentLightLevelMax
    }

    pub const fn content_light_level_average(&self) -> u32 {
        self.0.ContentLightLevelAverage
    }

    pub const fn flip(&self) -> i32 {
        self.0.Flip
    }
}

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
        let video_source = unsafe {
            ffms2_sys::FFMS_CreateVideoSource(
                source.as_ptr(),
                Track as i32,
                Index.as_mut_ptr(),
                Threads as i32,
                SeekMode::ffms2_seek_mode(SeekMode) as i32,
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

        VideoProperties(*ref_video)
    }

    pub fn SetInputFormatV(
        &self,
        ColorSpace: usize,
        ColorRange: ColorRange,
        PixelFormat: usize,
    ) -> Result<()> {
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_SetInputFormatV(
                self.video_source,
                ColorSpace as i32,
                ColorRange::ffms2_color_ranges(ColorRange) as i32,
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
