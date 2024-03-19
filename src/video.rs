use std::ffi::CString;
use std::path::Path;

use ffms2_sys::{FFMS_ColorRanges, FFMS_SeekMode, FFMS_Stereo3DFlags};

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
pub struct VideoProperties {
    pub fps_numerator: usize,
    pub fps_denominator: usize,
    pub rff_numerator: usize,
    pub rff_denominator: usize,
    pub frames_count: usize,
    pub sar_numerator: usize,
    pub sar_denominator: usize,
    pub crop_top: i32,
    pub crop_bottom: i32,
    pub crop_left: i32,
    pub crop_right: i32,
    pub top_field_first: usize,
    pub color_space: usize,
    pub color_range: ColorRange,
    pub first_time: f64,
    pub last_time: f64,
    pub rotation: usize,
    pub stereo3d_type: Stereo3DType,
    pub stereo3d_flags: Stereo3DFlags,
    pub last_end_time: f64,
    pub has_mastering_display_primaries: bool,
    pub mastering_display_primaries_x: [f64; 3],
    pub mastering_display_primaries_y: [f64; 3],
    pub mastering_display_white_point_x: f64,
    pub mastering_display_white_point_y: f64,
    pub has_mastering_display_luminance: bool,
    pub mastering_display_min_luminance: f64,
    pub mastering_display_max_luminance: f64,
    pub has_content_light_level: bool,
    pub content_light_level_max: usize,
    pub content_light_level_average: usize,
    pub flip: Flip,
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
        let ref_video = unsafe { *video_prop };

        VideoProperties {
            fps_numerator: ref_video.FPSNumerator as usize,
            fps_denominator: ref_video.FPSDenominator as usize,
            rff_numerator: ref_video.RFFNumerator as usize,
            rff_denominator: ref_video.RFFDenominator as usize,
            frames_count: ref_video.NumFrames as usize,
            sar_numerator: ref_video.SARNum as usize,
            sar_denominator: ref_video.SARDen as usize,
            crop_top: ref_video.CropTop,
            crop_bottom: ref_video.CropBottom,
            crop_left: ref_video.CropLeft,
            crop_right: ref_video.CropRight,
            top_field_first: ref_video.TopFieldFirst as usize,
            color_space: ref_video.ColorSpace as usize,
            color_range: ColorRange::new(ref_video.ColorRange),
            first_time: ref_video.FirstTime,
            last_time: ref_video.LastTime,
            rotation: ref_video.Rotation as usize,
            stereo3d_type: Stereo3DType::new(ref_video.Stereo3DType),
            stereo3d_flags: Stereo3DFlags::new(ref_video.Stereo3DFlags),
            last_end_time: ref_video.LastEndTime,
            has_mastering_display_primaries: ref_video
                .HasMasteringDisplayPrimaries
                > 0,
            mastering_display_primaries_x: ref_video
                .MasteringDisplayPrimariesX,
            mastering_display_primaries_y: ref_video
                .MasteringDisplayPrimariesY,
            mastering_display_white_point_x: ref_video
                .MasteringDisplayWhitePointX,
            mastering_display_white_point_y: ref_video
                .MasteringDisplayWhitePointY,
            has_mastering_display_luminance: ref_video
                .HasMasteringDisplayLuminance
                > 0,
            mastering_display_min_luminance: ref_video
                .MasteringDisplayMinLuminance,
            mastering_display_max_luminance: ref_video
                .MasteringDisplayMaxLuminance,
            has_content_light_level: ref_video.HasContentLightLevel > 0,
            content_light_level_max: ref_video.ContentLightLevelMax as usize,
            content_light_level_average: ref_video.ContentLightLevelAverage
                as usize,
            flip: if ref_video.Flip == -1 {
                Flip::Vertical
            } else if ref_video.Flip == 1 {
                Flip::Horizontal
            } else {
                Flip::Unknown
            },
        }
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
