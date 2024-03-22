use std::mem;
use std::slice;

use std::ffi::CString;

//use ffmpeg_the_third::ffi::AVPixelFormat;
//use ffmpeg_the_third::format::Pixel;

use ffms2_sys::{FFMS_Frame, FFMS_FrameInfo, FFMS_Resizers};

use crate::error::{InternalError, Result};
use crate::pixel::PixelFormat;
use crate::video::{ColorRange, VideoSource};

/// Supported number of frame planes.
pub const PLANES_COUNT: usize = 4;

/// Frame resizing/scaling algorithms.
#[derive(Clone, Copy, Debug)]
pub enum Resizers {
    /// Fast bilinear scaling algorithm.
    FastBilinear,
    /// Bilinear scaling algorithm.
    Bilinear,
    /// Bicubic scaling algorithm.
    Bicubic,
    /// Experimental scaling algorithm.
    Experimental,
    /// Nearest neighbor rescaling algorithm.
    NearestNeighbor,
    /// Averaging area rescaling algorithm.
    Area,
    /// Bicubic scaling algorithm for the luma plane, while bilinear
    /// for chroma planes.
    Bicublin,
    /// Gaussian rescaling algorithm.
    Gauss,
    /// Sinc rescaling algorithm.
    Sinc,
    /// Lanczos rescaling algorithm.
    Lanczos,
    /// Natural bicubic spline rescaling algorithm.
    Spline,
}

impl Resizers {
    pub(crate) const fn ffms2_resizer(self) -> FFMS_Resizers {
        match self {
            Self::FastBilinear => FFMS_Resizers::FFMS_RESIZER_FAST_BILINEAR,
            Self::Bilinear => FFMS_Resizers::FFMS_RESIZER_BILINEAR,
            Self::Bicubic => FFMS_Resizers::FFMS_RESIZER_BICUBIC,
            Self::Experimental => FFMS_Resizers::FFMS_RESIZER_X,
            Self::NearestNeighbor => FFMS_Resizers::FFMS_RESIZER_POINT,
            Self::Area => FFMS_Resizers::FFMS_RESIZER_AREA,
            Self::Bicublin => FFMS_Resizers::FFMS_RESIZER_BICUBLIN,
            Self::Gauss => FFMS_Resizers::FFMS_RESIZER_GAUSS,
            Self::Sinc => FFMS_Resizers::FFMS_RESIZER_SINC,
            Self::Lanczos => FFMS_Resizers::FFMS_RESIZER_LANCZOS,
            Self::Spline => FFMS_Resizers::FFMS_RESIZER_SPLINE,
        }
    }
}

/// Chroma samples location in a frame.
///
/// The illustration shows the location of the first (top-left) chroma sample
/// of a frame.
///
/// The left scheme shows **only** luma samples.
/// The right scheme shows the location of the chroma sample.
///
/// To have a real and complete visualization, the left scheme must be
/// overlapped to the right scheme, but this is not possible due to
/// text limitations.
///
///                  a   b         c   d
///                  v   v         v   v
///                  ______        ______
/// 1st luma line > |X   X ...    |3 4 X ...
///                 |             |1 2
/// 2nd luma line > |X   X ...    |5 6 X ...
///
/// *X*: _luma samples_
///
/// # Chroma locations
///
/// - *1* = _Left_
/// - *2* = _Center_
/// - *3* = _Top-left_
/// - *4* = _Top_
/// - *5* = _Bottom-left_
/// - *6* = _Bottom_
///
/// # Samples descriptions
///
/// - *a* = _1st horizontal luma sample location_
/// - *b* = _2nd horizontal luma sample location_
/// - *c* = _1st top-left chroma sample location_
/// - *d* = _2nd horizontal luma sample location_
#[derive(Clone, Copy, Debug, Default)]
pub enum ChromaLocation {
    /// Unspecified location.
    #[default]
    Unspecified,
    /// Left.
    ///
    /// MPEG-2/4 4:2:0, H.264 default for 4:2:0.
    Left,
    /// Center.
    ///
    /// MPEG-1 4:2:0, JPEG 4:2:0, H.263 4:2:0.
    Center,
    /// Top-left.
    ///
    /// ITU-R 601, SMPTE 274M 296M S314M(DV 4:1:1), mpeg2 4:2:2.
    TopLeft,
    /// Top.
    Top,
    /// Bottom-left.
    BottomLeft,
    /// Bottom.
    Bottom,
}

impl ChromaLocation {
    const fn new(chroma_locations: i32) -> Self {
        use ffms2_sys::FFMS_ChromaLocations::*;
        match chroma_locations {
            e if e == FFMS_LOC_UNSPECIFIED as i32 => Self::Unspecified,
            e if e == FFMS_LOC_LEFT as i32 => Self::Left,
            e if e == FFMS_LOC_CENTER as i32 => Self::Center,
            e if e == FFMS_LOC_TOPLEFT as i32 => Self::TopLeft,
            e if e == FFMS_LOC_TOP as i32 => Self::Top,
            e if e == FFMS_LOC_BOTTOMLEFT as i32 => Self::BottomLeft,
            e if e == FFMS_LOC_BOTTOM as i32 => Self::Bottom,
            _ => Self::Unspecified,
        }
    }
}

/// Video frame metadata.
#[derive(Debug)]
pub struct FrameInfo {
    /// The decoding timestamp of a frame.
    ///
    /// To convert this to a timestamp in clock milliseconds, use:
    ///
    /// (`[FrameInfo.pts]` * `[TrackTimebase.numerator]`) / `[TrackTimebase.denominator]`.
    pub pts: u64,
    /// Repeat First Field (RFF) flag for a MPEG frame.
    ///
    /// A frame must be displayed for `1 + repeat_picture` time units,
    /// where the time units are expressed in the special
    /// `[VideoSource.RFFTimebase]`.
    ///
    /// Usual timestamps must be ignored since since they are fundamentally
    /// incompatible with RFF data.
    pub repeat_picture: usize,
    /// Whether a frame is a keyframe.
    pub keyframe: bool,
    pub original_pts: usize,
}

impl FrameInfo {
    pub(crate) fn new(frame_info: FFMS_FrameInfo) -> Self {
        Self {
            pts: frame_info.PTS as u64,
            repeat_picture: frame_info.RepeatPict as usize,
            keyframe: frame_info.KeyFrame > 0,
            original_pts: frame_info.OriginalPTS as usize,
        }
    }
}

#[derive(Debug)]
pub struct FrameResolution {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Frame {
    pub linesize: [usize; PLANES_COUNT],
    pub resolution: FrameResolution,
    pub encoded_width: usize,
    pub encoded_height: usize,
    pub encoded_pixel_format: PixelFormat,
    pub scaled_width: usize,
    pub scaled_height: usize,
    pub converted_pixel_format: PixelFormat,
    pub keyframe: usize,
    pub repeat_picture: usize,
    pub interlaced_frame: usize,
    pub top_field_first: usize,
    pub picture_type: u8,
    /// YUV color space for a video track.
    pub color_space: usize,
    /// Valid range of luma values for a YUV video track.
    pub color_range: ColorRange,
    pub color_primaries: usize,
    pub transfer_characteristics: usize,
    pub chroma_location: ChromaLocation,
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
    frame: FFMS_Frame,
}

impl Frame {
    pub fn new(
        video_source: &mut VideoSource,
        frame_number: usize,
    ) -> Result<Self> {
        let mut error = InternalError::new();

        let ffms2_frame = unsafe {
            ffms2_sys::FFMS_GetFrame(
                video_source.as_mut_ptr(),
                frame_number as i32,
                error.as_mut_ptr(),
            )
        };

        if ffms2_frame.is_null() {
            Err(error.into())
        } else {
            let ref_frame = unsafe { &*ffms2_frame };

            Ok(Self::create_frame(*ref_frame))
        }
    }

    pub fn frame_by_time(
        video_source: &mut VideoSource,
        time: f64,
    ) -> Result<Self> {
        let mut error = InternalError::new();

        let ffms2_frame = unsafe {
            ffms2_sys::FFMS_GetFrameByTime(
                video_source.as_mut_ptr(),
                time,
                error.as_mut_ptr(),
            )
        };

        if ffms2_frame.is_null() {
            Err(error.into())
        } else {
            let ref_frame = unsafe { &*ffms2_frame };

            Ok(Self::create_frame(*ref_frame))
        }
    }

    pub fn pixel_format(name: &str) -> Result<PixelFormat> {
        let source = CString::new(name)?;
        let pixel_format =
            unsafe { ffms2_sys::FFMS_GetPixFmt(source.as_ptr()) };
        Ok(PixelFormat::new(pixel_format))
    }

    /*pub fn planes(&self) -> Option<[Option<&[u8]>; PLANES_NUMBER]> {
        let mut planes: [Option<&[u8]>; PLANES_NUMBER] = Default::default();

        let log2_chroma_h =
            match Self::i32_to_pixel_format(self.frame.EncodedPixelFormat)
                .descriptor()
            {
                Some(pix_fmt_descriptor) => pix_fmt_descriptor.log2_chroma_h(),
                None => return None,
            };

        for (i, (plane, (data, linesize))) in planes
            .iter_mut()
            .zip(
                self.frame
                    .Data
                    .into_iter()
                    .zip(self.frame.Linesize.into_iter()),
            )
            .enumerate()
        {
            if linesize == 0 {
                *plane = None;
            } else {
                let sub_h = if i == 0 { 0 } else { log2_chroma_h };
                let plane_slice_length =
                    (linesize * self.frame.EncodedHeight) >> sub_h;
                let plane_slice = unsafe {
                    slice::from_raw_parts(data, plane_slice_length as usize)
                };

                *plane = Some(plane_slice);
            }
        }

        Some(planes)
    }*/

    pub fn dolby_vision_rpu(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.frame.DolbyVisionRPU,
                self.frame.DolbyVisionRPUSize as usize,
            )
        }
    }

    pub fn hdr10_plus(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.frame.HDR10Plus,
                self.frame.HDR10PlusSize as usize,
            )
        }
    }

    const fn linesize(frame: &FFMS_Frame) -> [usize; PLANES_COUNT] {
        [
            frame.Linesize[0] as usize,
            frame.Linesize[1] as usize,
            frame.Linesize[2] as usize,
            frame.Linesize[3] as usize,
        ]
    }

    const fn frame_resolution(frame: &FFMS_Frame) -> FrameResolution {
        let width = if frame.ScaledWidth == -1 {
            frame.EncodedWidth
        } else {
            frame.ScaledWidth
        };

        let height = if frame.ScaledHeight == -1 {
            frame.EncodedHeight
        } else {
            frame.ScaledHeight
        };

        FrameResolution {
            width: width as usize,
            height: height as usize,
        }
    }

    const fn create_frame(frame: FFMS_Frame) -> Self {
        Self {
            linesize: Self::linesize(&frame),
            resolution: Self::frame_resolution(&frame),
            encoded_width: frame.EncodedWidth as usize,
            encoded_height: frame.EncodedHeight as usize,
            encoded_pixel_format: PixelFormat::new(frame.EncodedPixelFormat),
            scaled_width: frame.ScaledWidth as usize,
            scaled_height: frame.ScaledHeight as usize,
            converted_pixel_format: PixelFormat::new(
                frame.ConvertedPixelFormat,
            ),
            keyframe: frame.KeyFrame as usize,
            repeat_picture: frame.RepeatPict as usize,
            interlaced_frame: frame.InterlacedFrame as usize,
            top_field_first: frame.TopFieldFirst as usize,
            picture_type: frame.PictType as u8,
            color_space: frame.ColorSpace as usize,
            color_range: ColorRange::new(frame.ColorRange),
            color_primaries: frame.ColorPrimaries as usize,

            transfer_characteristics: frame.TransferCharateristics as usize,

            chroma_location: ChromaLocation::new(frame.ChromaLocation),
            has_mastering_display_primaries: frame
                .HasMasteringDisplayPrimaries
                > 0,
            mastering_display_primaries_x: frame.MasteringDisplayPrimariesX,
            mastering_display_primaries_y: frame.MasteringDisplayPrimariesY,
            mastering_display_white_point_x: frame.MasteringDisplayWhitePointX,
            mastering_display_white_point_y: frame.MasteringDisplayWhitePointY,
            has_mastering_display_luminance: frame
                .HasMasteringDisplayLuminance
                > 0,
            mastering_display_min_luminance: frame
                .MasteringDisplayMinLuminance,
            mastering_display_max_luminance: frame
                .MasteringDisplayMaxLuminance,
            has_content_light_level: frame.HasContentLightLevel > 0,
            content_light_level_max: frame.ContentLightLevelMax as usize,
            content_light_level_average: frame.ContentLightLevelAverage
                as usize,
            frame,
        }
    }

    /*fn i32_to_pixel_format(i32_pixel: i32) -> Pixel {
        // This is not good, but we can't think of any better way to do this.
        // See https://github.com/rust-av/ffms2-rs/pull/29#discussion_r1115397695
        // What we've considered:
        // 1. A large match statement mapping the i32 (from FFMS2) to the AVPixelFormat enum.
        //    This is an unreasonable amount of work since the AVPixelFormat enum is different
        //    across versions and build configurations of FFmpeg, all of which we would need to support.
        // 2. Parsing the pixel format string.
        //    Although FFMS2 provides a function to get the i32 from the pixel format string,
        //    there's no function for the other way around.
        // 3. Making a PR in FFMS2 to expose chrome height in the frame struct.
        //    This is the best solution; we just gotta find someone to do it.
        let pix_fmt: AVPixelFormat = unsafe { mem::transmute(i32_pixel) };
        Pixel::from(pix_fmt)
    }*/
}
