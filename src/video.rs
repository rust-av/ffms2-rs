use std::ffi::CString;
use std::path::Path;

use ffms2_sys::{
    FFMS_ColorRanges, FFMS_SeekMode, FFMS_Stereo3DFlags, FFMS_VideoProperties,
};

use crate::error::{Error, InternalError, Result};
use crate::frame::Resizers;
use crate::index::Index;

/// File seeking mode.
///
/// Each mode provides a different way of managing file seeking.
#[derive(Clone, Copy, Debug)]
pub enum SeekMode {
    /// Linear access without rewind.
    ///
    /// In this mode, an error is thrown whether each successive frame number
    /// request is smaller than the last one.
    ///
    /// It is only intended for opening images, but it might work the same
    /// even with not so well known video formats.
    LinearNoRW,
    /// Linear access.
    ///
    /// If a frame `n` is requested, without having requested before frames from
    /// 0 to `n-1`, in this very order, then all frames from 0 to `n-1`
    /// will have to be decoded before frame `n` can be delivered.
    ///
    /// This mode is pretty slow, but needed for some kinds of formats.
    Linear,
    /// Safe normal mode.
    ///
    /// Seeking decisions are based on the keyframe positions.
    Normal,
    /// Unsafe normal mode.
    ///
    /// Same as `Normal` mode, but no error will be thrown when the exact
    /// destination has to be guessed.
    Unsafe,
    /// Aggressive mode.
    ///
    /// It seeks in the forward direction even if no closer keyframe is known
    /// to exist.
    ///
    /// Only useful for testing purposes and those containers whose keyframes
    /// are not reported properly.
    Aggressive,
}

impl SeekMode {
    const fn ffms2_seek_mode(self) -> FFMS_SeekMode {
        match self {
            Self::LinearNoRW => FFMS_SeekMode::FFMS_SEEK_LINEAR_NO_RW,
            Self::Linear => FFMS_SeekMode::FFMS_SEEK_LINEAR,
            Self::Normal => FFMS_SeekMode::FFMS_SEEK_NORMAL,
            Self::Unsafe => FFMS_SeekMode::FFMS_SEEK_UNSAFE,
            Self::Aggressive => FFMS_SeekMode::FFMS_SEEK_AGGRESSIVE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Stereo3DType {
    #[default]
    Unknown,
    TwoDimensional,
    SideBySide,
    TopBottom,
    FrameSequence,
    CheckerBoard,
    SideBySideQuincunx,
    Lines,
    Columns,
}

impl Stereo3DType {
    const fn new(stereo_3d_type: i32) -> Self {
        use ffms2_sys::FFMS_Stereo3DType::*;
        match stereo_3d_type {
            e if e == FFMS_S3D_TYPE_2D as i32 => Self::TwoDimensional,
            e if e == FFMS_S3D_TYPE_SIDEBYSIDE as i32 => Self::SideBySide,
            e if e == FFMS_S3D_TYPE_TOPBOTTOM as i32 => Self::TopBottom,
            e if e == FFMS_S3D_TYPE_FRAMESEQUENCE as i32 => {
                Self::FrameSequence
            }
            e if e == FFMS_S3D_TYPE_CHECKERBOARD as i32 => Self::CheckerBoard,
            e if e == FFMS_S3D_TYPE_SIDEBYSIDE_QUINCUNX as i32 => {
                Self::SideBySideQuincunx
            }
            e if e == FFMS_S3D_TYPE_LINES as i32 => Self::Lines,
            e if e == FFMS_S3D_TYPE_COLUMNS as i32 => Self::Columns,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Stereo3DFlags {
    #[default]
    Unknown,
    Invert,
}

impl Stereo3DFlags {
    const fn new(stereo_3d_flags: i32) -> Self {
        match stereo_3d_flags {
            e if e == FFMS_Stereo3DFlags::FFMS_S3D_FLAGS_INVERT as i32 => {
                Self::Invert
            }
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ColorRange {
    #[default]
    Unspecified,
    Mpeg,
    Jpeg,
}

impl ColorRange {
    const fn ffms2_color_ranges(self) -> FFMS_ColorRanges {
        match self {
            Self::Unspecified => FFMS_ColorRanges::FFMS_CR_UNSPECIFIED,
            Self::Mpeg => FFMS_ColorRanges::FFMS_CR_MPEG,
            Self::Jpeg => FFMS_ColorRanges::FFMS_CR_JPEG,
        }
    }

    pub(crate) const fn new(color_range: i32) -> Self {
        use ffms2_sys::FFMS_ColorRanges::*;
        match color_range {
            e if e == FFMS_CR_UNSPECIFIED as i32 => Self::Unspecified,
            e if e == FFMS_CR_MPEG as i32 => Self::Mpeg,
            e if e == FFMS_CR_JPEG as i32 => Self::Jpeg,

            _ => Self::Unspecified,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Flip {
    #[default]
    Unknown,
    Vertical,
    Horizontal,
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

    pub const fn frames_count(&self) -> usize {
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

    pub const fn colorspace(&self) -> usize {
        self.0.ColorSpace as usize
    }

    pub const fn color_range(&self) -> ColorRange {
        ColorRange::new(self.0.ColorRange)
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

    pub const fn stereo3d_type(&self) -> Stereo3DType {
        Stereo3DType::new(self.0.Stereo3DType)
    }

    pub const fn stereo3d_flags(&self) -> Stereo3DFlags {
        Stereo3DFlags::new(self.0.Stereo3DFlags)
    }

    pub const fn last_end_time(&self) -> f64 {
        self.0.LastEndTime
    }

    pub const fn has_mastering_display_primaries(&self) -> bool {
        self.0.HasMasteringDisplayPrimaries > 0
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

    pub const fn has_mastering_display_luminance(&self) -> bool {
        self.0.HasMasteringDisplayLuminance > 0
    }

    pub const fn mastering_display_min_luminance(&self) -> f64 {
        self.0.MasteringDisplayMinLuminance
    }

    pub const fn mastering_display_max_luminance(&self) -> f64 {
        self.0.MasteringDisplayMaxLuminance
    }

    pub const fn has_content_light_level(&self) -> bool {
        self.0.HasContentLightLevel > 0
    }

    pub const fn content_light_level_max(&self) -> u32 {
        self.0.ContentLightLevelMax
    }

    pub const fn content_light_level_average(&self) -> u32 {
        self.0.ContentLightLevelAverage
    }

    pub const fn flip(&self) -> Flip {
        if self.0.Flip == -1 {
            Flip::Vertical
        } else if self.0.Flip == 1 {
            Flip::Horizontal
        } else {
            Flip::Unknown
        }
    }
}

/// Video source manager.
///
/// Among its functionalities:
/// - Opening a video source
/// - Retrieving video frames data
/// - Setting the output video source data format
pub struct VideoSource(*mut ffms2_sys::FFMS_VideoSource);

unsafe impl Send for VideoSource {}

impl VideoSource {
    /// Creates a new `[VideoSource]` instance.
    pub fn new(
        source_file: &Path,
        track: usize,
        index: &Index,
        threads: usize,
        seek_mode: SeekMode,
    ) -> Result<Self> {
        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;

        let mut error = InternalError::new();
        let video_source = unsafe {
            ffms2_sys::FFMS_CreateVideoSource(
                source.as_ptr(),
                track as i32,
                index.as_mut_ptr(),
                threads as i32,
                SeekMode::ffms2_seek_mode(seek_mode) as i32,
                error.as_mut_ptr(),
            )
        };

        if video_source.is_null() {
            Err(error.into())
        } else {
            Ok(Self(video_source))
        }
    }

    /// Returns the properties associated with a video source.
    pub fn video_properties(&self) -> VideoProperties {
        let video_prop = unsafe { ffms2_sys::FFMS_GetVideoProperties(self.0) };
        let ref_video = unsafe { &*video_prop };

        VideoProperties(*ref_video)
    }

    /// Sets frame format for input video source.
    pub fn set_input_format(
        &self,
        color_space: usize,
        color_range: ColorRange,
        pixel_format: usize,
    ) -> Result<()> {
        let mut error = InternalError::new();
        let err = unsafe {
            ffms2_sys::FFMS_SetInputFormatV(
                self.0,
                color_space as i32,
                ColorRange::ffms2_color_ranges(color_range) as i32,
                pixel_format as i32,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    /// Resets frame format for input video source.
    pub fn reset_input_format(&self) {
        unsafe {
            ffms2_sys::FFMS_ResetInputFormatV(self.0);
        }
    }

    pub fn set_output_format_v2(
        &self,
        target_formats: &mut Vec<i32>,
        width: usize,
        height: usize,
        resizer: Resizers,
    ) -> Result<()> {
        let mut error = InternalError::new();
        target_formats.push(-1);
        let err = unsafe {
            ffms2_sys::FFMS_SetOutputFormatV2(
                self.0,
                target_formats.as_ptr(),
                width as i32,
                height as i32,
                Resizers::ffms2_resizer(resizer) as i32,
                error.as_mut_ptr(),
            )
        };
        target_formats.pop();

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    pub fn reset_output_format(&self) {
        unsafe {
            ffms2_sys::FFMS_ResetOutputFormatV(self.0);
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffms2_sys::FFMS_VideoSource {
        self.0
    }
}

impl Drop for VideoSource {
    fn drop(&mut self) {
        unsafe {
            ffms2_sys::FFMS_DestroyVideoSource(self.0);
        }
    }
}
