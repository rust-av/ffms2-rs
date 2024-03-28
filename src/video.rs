use std::cmp::Ordering;
use std::path::Path;

use std::ffi::CString;

use ffms2_sys::{FFMS_ColorRanges, FFMS_SeekMode, FFMS_Stereo3DFlags};

use crate::error::{Error, InternalError, Result};
use crate::frame::Resizers;
use crate::index::Index;
use crate::pixel::PixelFormat;

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
    /// Seeking decisions are based on the key frame positions.
    Normal,
    /// Unsafe normal mode.
    ///
    /// Same as `Normal` mode, but no error will be thrown when the exact
    /// destination has to be guessed.
    Unsafe,
    /// Aggressive mode.
    ///
    /// It seeks in the forward direction even if no closer key frame is known
    /// to exist.
    ///
    /// Only useful for testing purposes and those containers whose key frames
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

/// Pixel type packaging for a stereoscopic 3D video source.
#[derive(Clone, Copy, Debug, Default)]
pub enum Stereo3DType {
    /// Unknown type.
    #[default]
    Unknown,
    /// Not a stereoscopic video source.
    TwoDimensional,
    /// Video views are next to each other.
    ///
    /// ```
    /// LLLLRRRR
    /// LLLLRRRR
    /// LLLLRRRR
    /// ...
    /// ```
    SideBySide,
    /// Video views are on top of each other.
    ///
    /// ```
    /// LLLLLLLL
    /// LLLLLLLL
    /// RRRRRRRR
    /// RRRRRRRR
    /// ```
    TopBottom,
    /// Video views are alternated temporally.
    ///
    /// ```
    /// frame0   frame1   frame2   ...
    /// LLLLLLLL RRRRRRRR LLLLLLLL
    /// LLLLLLLL RRRRRRRR LLLLLLLL
    /// LLLLLLLL RRRRRRRR LLLLLLLL
    /// ...      ...      ...
    /// ```
    FrameSequence,
    /// Video views are packed in a checkerboard-like structure per pixel.
    ///
    /// ```
    /// LRLRLRLR
    /// RLRLRLRL
    /// LRLRLRLR
    /// ...
    /// ```
    CheckerBoard,
    /// Video views are next to each other, but when upscaling a checkerboard
    /// pattern is applied.
    ///
    /// ```
    /// LLLLRRRR          L L L L    R R R R
    /// LLLLRRRR    =>     L L L L  R R R R
    /// LLLLRRRR          L L L L    R R R R
    /// LLLLRRRR           L L L L  R R R R
    /// ```
    SideBySideQuincunx,
    /// Video views are packed per line, as if interlaced.
    ///
    /// ```
    /// LLLLLLLL
    /// RRRRRRRR
    /// LLLLLLLL
    /// ...
    /// ```
    Lines,
    /// Video views are packed per column.
    ///
    /// ```
    /// LRLRLRLR
    /// LRLRLRLR
    /// LRLRLRLR
    /// ...
    /// ```
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

/// Flags for a stereoscopic 3D video source.
#[derive(Clone, Copy, Debug, Default)]
pub enum Stereo3DFlags {
    /// Unknown flag.
    #[default]
    Unknown,
    /// Inverted views.
    ///
    /// Right/Bottom represents the left view.
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

/// Valid range of luma values for a YUV video source.
#[derive(Clone, Copy, Debug, Default)]
pub enum ColorRange {
    /// The range is not specified.
    #[default]
    Unspecified,
    /// TV range, also known as limited range.
    ///
    /// Range of luma values: [16, 235].
    /// Bit-depth: 8-bit
    Mpeg,
    /// PC range, also known as full range.
    ///
    /// Range of luma values: [0, 255].
    /// Bit-depth: 8-bit.
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

/// Flip direction to be applied to a frame before a rotation.
#[derive(Clone, Copy, Debug, Default)]
pub enum Flip {
    /// No flip operation.
    #[default]
    NoFlip,
    /// Horizontal flip direction.
    Horizontal,
    /// Vertical flip direction.
    Vertical,
}

/// Frame rate associated with a video source.
///
/// It is obtained dividing the numerator by the denominator field.
///
/// For `Matroska` video sources, this rational number is based on the
/// average frame duration of all frames, while, for everything else,
/// it is based on the duration of the first frame.
///
/// This value should not be used to extrapolate clock timestamps
/// for each frame, since it makes impossible to manage variable frame rates
/// properly.
///
/// This value is mostly useful for informational purposes and might
/// be considered reliable for old containers formats such as AVI.
///
/// It would be better to generate individual frame timestamps based on
/// the `[Frame.pts]` value instead of using this field.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameRate {
    /// Frame rate numerator.
    pub numerator: usize,
    /// Frame rate denominator.
    pub denominator: usize,
}

/// Repeat First Field (RFF) timebase associated with a MPEG video source.
#[derive(Clone, Copy, Debug, Default)]
pub struct RFFTimebase {
    /// RFF timebase numerator.
    pub numerator: usize,
    /// RFF timebase denominator.
    pub denominator: usize,
}

/// Sample aspect ratio for video source frames.
///
/// This value should be taken into account when computing the correct display
/// aspect ratio for anamorphic video sources.
#[derive(Clone, Copy, Debug, Default)]
pub struct SampleAspectRatio {
    /// Sample aspect ratio numerator.
    pub numerator: usize,
    /// Sample aspect ratio denominator.
    pub denominator: usize,
}

/// The number of pixels in each direction (top, bottom, left, right)
/// a frame should be cropped before displaying it.
///
/// It is recommended to use this metadata whenever frames must be displayed
/// in an accurate way.
#[derive(Clone, Copy, Debug, Default)]
pub struct Crop {
    /// Top direction.
    pub top: i32,
    /// Bottom direction.
    pub bottom: i32,
    /// Left direction.
    pub left: i32,
    /// Right direction.
    pub right: i32,
}

/// Video source metadata.
#[derive(Debug)]
pub struct VideoProperties {
    /// Frame rate associated with a video source.
    pub frame_rate: FrameRate,
    /// Repeat First Field (RFF) timebase associated with a MPEG video source.
    pub rff_timebase: RFFTimebase,
    /// Number of frames in a video source.
    pub frames_count: usize,
    /// Sample aspect ratio for video source frames.
    pub sar: SampleAspectRatio,
    /// The number of pixels a frame should be cropped in order to be displayed
    /// correctly.
    pub crop: Crop,
    /// Whether a video source has the top field first, otherwise it has the
    /// bottom field first.
    pub top_field_first: bool,
    /// First timestamp of a video source in seconds.
    ///
    /// Useful to know whether a video source has a delay, or for quickly
    /// determining its length in seconds.
    pub first_time: f64,
    /// Last timestamp of a video source in seconds.
    ///
    /// Useful to know whether a video source has a delay, or for quickly
    /// determining its length in seconds.
    pub last_time: f64,
    /// Video source rotation in degrees.
    pub rotation: usize,
    /// Pixel type format for a stereoscopic 3D video source.
    pub stereo3d_type: Stereo3DType,
    /// Flags for a stereoscopic 3D video source.
    pub stereo3d_flags: Stereo3DFlags,
    /// The end time of the last packet of a video source in milliseconds.
    pub last_end_time: f64,
    /// Whether the color primaries coordinates of the display used to master a
    /// video source content are accessible.
    ///
    /// A receiver can use this metadata to determine whether a video source
    /// content could contain colors or light levels which cannot be reproduced
    /// by the current display.
    pub has_mastering_display_primaries: bool,
    /// RGB chromaticity x-coordinates of the display used to master a
    /// video source content.
    pub mastering_display_primaries_x: [f64; 3],
    /// RGB chromaticity y-coordinates of the display used to master a
    /// video source content.
    pub mastering_display_primaries_y: [f64; 3],
    /// White point x-coordinate of the display used to master a
    /// video source content.
    pub mastering_display_white_point_x: f64,
    /// White point y-coordinate of the display used to master a
    /// video source content.
    pub mastering_display_white_point_y: f64,
    /// Whether the luminance values of the display used to master a video
    /// source content are accessible.
    pub has_mastering_display_luminance: bool,
    /// Minimum luminance of the display used to master a video
    /// source content in cd/m^2.
    pub mastering_display_min_luminance: f64,
    /// Maximum luminance of the display used to master a video
    /// source content in cd/m^2.
    pub mastering_display_max_luminance: f64,
    /// Whether a video source content has maximum and average light levels.
    /// Both of these values are measured over the duration of the content.
    ///
    /// A receiver can use this information to adjust the content light
    /// levels so that they match the capability of the current display.
    pub has_content_light_level: bool,
    /// Maximum light level of a video source content in cd/m^2.
    pub content_light_level_max: usize,
    /// Average light level of a video source content in cd/m^2.
    pub content_light_level_average: usize,
    /// Flip direction to be applied to a frame before a rotation.
    pub flip: Flip,
}

/// Video source manager.
///
/// Among its functionalities:
/// - Opening a video source which must be indexed first through `[Index]`
/// - Retrieving video frames data
/// - Setting data format for an output video source
pub struct VideoSource(*mut ffms2_sys::FFMS_VideoSource);

unsafe impl Send for VideoSource {}

impl VideoSource {
    /// Creates a new `[VideoSource]` instance.
    pub fn new(
        source_file: &Path,
        track_number: usize,
        index: &Index,
        threads: usize,
        seek_mode: SeekMode,
    ) -> Result<Self> {
        if track_number > index.tracks_count() - 1 {
            return Err(Error::WrongTrack);
        }

        if source_file.is_file() {
            return Err(Error::NotAFile);
        }

        let source =
            CString::new(source_file.to_str().ok_or(Error::StrConversion)?)?;

        let mut error = InternalError::new();
        let video_source = unsafe {
            ffms2_sys::FFMS_CreateVideoSource(
                source.as_ptr(),
                track_number as i32,
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

    /// Returns all video source properties.
    pub fn video_properties(&self) -> VideoProperties {
        let video_prop = unsafe { ffms2_sys::FFMS_GetVideoProperties(self.0) };
        let ref_video = unsafe { *video_prop };

        VideoProperties {
            frame_rate: FrameRate {
                numerator: ref_video.FPSNumerator as usize,
                denominator: ref_video.FPSDenominator as usize,
            },
            rff_timebase: RFFTimebase {
                numerator: ref_video.RFFNumerator as usize,
                denominator: ref_video.RFFDenominator as usize,
            },
            frames_count: ref_video.NumFrames as usize,
            sar: SampleAspectRatio {
                numerator: ref_video.SARNum as usize,
                denominator: ref_video.SARDen as usize,
            },
            crop: Crop {
                top: ref_video.CropTop,
                bottom: ref_video.CropBottom,
                left: ref_video.CropLeft,
                right: ref_video.CropRight,
            },
            top_field_first: ref_video.TopFieldFirst > 0,
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
            flip: match ref_video.Flip.cmp(&0) {
                Ordering::Equal => Flip::NoFlip,
                Ordering::Greater => Flip::Horizontal,
                Ordering::Less => Flip::Vertical,
            },
        }
    }

    /// Overrides the video source color space.
    ///
    /// This method is intended primarily for compatibility with programs which
    /// use the wrong YUV color space when converting to or from RGB, but it can
    /// also be useful for files which have incorrect color space flags.
    ///
    /// Values passed to this functions are not checked for sanity.
    /// For example, RGB files can be treated as if they are actually YUV files
    /// using this method, but even if this approach works, it does not produce
    /// useful results.
    pub fn set_input_format(
        &self,
        color_space: PixelFormat,
        color_range: ColorRange,
        pixel_format: PixelFormat,
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

    /// Resets the input format to the values specified in the video source
    /// file.
    pub fn reset_input_format(&self) {
        unsafe {
            ffms2_sys::FFMS_ResetInputFormatV(self.0);
        }
    }

    /// Sets the color space and frame dimensions to be used for
    /// output video source frames.
    /// For example, tt can be used to convert a video source to
    /// greyscale or monochrome.
    pub fn set_output_format(
        &self,
        colorspaces: &[PixelFormat],
        width: usize,
        height: usize,
        resizer: Resizers,
    ) -> Result<()> {
        let mut error = InternalError::new();

        let mut colorspaces = colorspaces
            .iter()
            .map(|colorspace| *colorspace as i32)
            .collect::<Vec<i32>>();

        colorspaces.push(-1);

        let err = unsafe {
            ffms2_sys::FFMS_SetOutputFormatV2(
                self.0,
                colorspaces.as_ptr(),
                width as i32,
                height as i32,
                Resizers::ffms2_resizer(resizer) as i32,
                error.as_mut_ptr(),
            )
        };

        if err != 0 {
            Err(error.into())
        } else {
            Ok(())
        }
    }

    /// Resets the output format for a video source so that no conversion
    /// is applied.
    ///
    /// Note that the results of this function may vary wildly, particularly
    /// whether a video source changes its resolution in the middle.
    pub fn reset_output_format(&self) {
        unsafe {
            ffms2_sys::FFMS_ResetOutputFormatV(self.0);
        }
    }

    #[inline(always)]
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
