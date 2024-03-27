use std::mem;
use std::slice;

use std::ffi::CString;

//use ffmpeg_the_third::ffi::AVPixelFormat;
//use ffmpeg_the_third::format::Pixel;

use ffms2_sys::{FFMS_Frame, FFMS_FrameInfo, FFMS_Resizers};

use crate::error::{Error, InternalError, Result};
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
    /// Original decoding timestamp of a frame.
    ///
    /// All timestamps are normalized in order to be contiguous and this
    /// behavior might break some kind of video sources.
    ///
    /// This field should be used when a video source presents discontinuous
    /// timestamps such as Variable Frame Rate (VFR) formats.
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

/// Output frame resolution in pixels.
#[derive(Debug)]
pub struct FrameResolution {
    /// Frame width.
    pub width: usize,
    /// Frame height.
    pub height: usize,
}

#[derive(Debug)]
pub struct Frame {
    /// The length in bytes of each frame plane scan line. The number of scan
    /// lines is equal to the number of frame planes, thus `PLANES_COUNT`.
    ///
    /// The total size in bytes of a frame plane is obtained with the following
    /// computation:
    ///
    /// `linesize[i]` * `[VideoProperties.height]`
    ///
    /// Some pixel formats though, most notably `YV12`, have vertical chroma
    /// subsampling, and then the U/V planes may be of a different height than
    /// the primary plane. In that case, a frame is stored inverted in memory
    /// and a plane is retrieved starting from the last row of data.
    pub linesize: [usize; PLANES_COUNT],
    /// Output frame resolution in pixels.
    pub resolution: FrameResolution,
    /// The original frame width resolution, in pixels, as encoded in the
    /// compressed file, before any scaling is applied.
    ///
    /// It must not necessarily be the same for all frames in a video source.
    pub encoded_width: usize,
    /// The original frame height resolution, in pixels, as encoded in the
    /// compressed file, before any scaling is applied.
    ///
    /// It must not necessarily be the same for all frames in a video source.
    pub encoded_height: usize,
    /// The original frame pixel format, as encoded in the compressed file.
    pub encoded_pixel_format: PixelFormat,
    /// The output frame width resolution in pixels after a scaling has been
    /// applied. This represents the frame width resolution of planes
    /// returned by the `planes` function.
    pub scaled_width: usize,
    /// The output frame height resolution in pixels after a scaling has been
    /// applied. This represents the frame height resolution of planes
    /// returned by the `planes` function.
    pub scaled_height: usize,
    /// The output frame pixel format.
    pub converted_pixel_format: PixelFormat,
    /// Whether a frame is a keyframe.
    pub keyframe: usize,
    /// Repeat First Field (RFF) flag for a MPEG frame.
    ///
    /// A frame must be displayed for `1 + repeat_picture` time units,
    /// where the time units are expressed in the special
    /// `[VideoSource.RFFTimebase]`.
    ///
    /// Usual timestamps must be ignored since since they are fundamentally
    /// incompatible with RFF data.
    pub repeat_picture: usize,
    /// Whether a frame has been coded as interlaced.
    pub interlaced_frame: bool,
    /// Whether a frame has the top field first, otherwise it has the bottom
    /// field first.
    ///
    /// Only relevant when [`interlaced_frame`] is `true`.
    pub top_field_first: bool,
    /// Compressed frame coding type.
    ///
    /// - I: Intra
    /// - P: Predicted
    /// - B: Bi-dir predicted
    /// - S: S(GMC)-VOP MPEG4
    /// - i: Switching Intra
    /// - p: Switching Predicted
    /// - b: FF_BI_TYPE
    /// - ?: Unknown
    pub picture_type: char,
    /// YUV color space.
    pub color_space: PixelFormat,
    /// Valid range of luma values for a YUV video source.
    pub color_range: ColorRange,
    /// Frame color primaries.
    pub color_primaries: usize,
    /// Frame transfer characteristics.
    pub transfer_characteristics: usize,
    /// Chroma samples location in a frame.
    pub chroma_location: ChromaLocation,
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
    frame: FFMS_Frame,
}

impl Frame {
    /// Decodes and returns the `[Frame]` data associated with the input frame
    /// number from the given video source.
    ///
    /// This method is not thread-safe, so it can retrieve only one frame
    /// at a time.
    pub fn new(
        video_source: &mut VideoSource,
        frame_number: usize,
    ) -> Result<Self> {
        if frame_number > video_source.video_properties().frames_count - 1 {
            return Err(Error::WrongFrame);
        }

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

    /// Decodes and returns the `[Frame]` data associated with the input
    /// timestamp, in seconds, from the given video source.
    ///
    /// This method will retrieve the frame that starts closest to the input
    /// timestamp.
    pub fn frame_by_time(
        video_source: &mut VideoSource,
        timestamp: f64,
    ) -> Result<Self> {
        if timestamp.is_nan() || timestamp.is_sign_negative() {
            return Err(Error::WrongTimestamp);
        }

        let mut error = InternalError::new();

        let ffms2_frame = unsafe {
            ffms2_sys::FFMS_GetFrameByTime(
                video_source.as_mut_ptr(),
                timestamp,
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

    /// Translates a given color space/pixel format expressed in natural
    /// language into a [`PixelFormat`].
    pub fn pixel_format(name: &str) -> Result<PixelFormat> {
        let source = CString::new(name)?;
        let pixel_format =
            unsafe { ffms2_sys::FFMS_GetPixFmt(source.as_ptr()) };
        Ok(PixelFormat::new(pixel_format))
    }

    /// Returns all supported frame planes.
    ///
    /// When a plane, or all planes, cannot be retrieved for whatever reason,
    /// `None` is returned.
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

    /// Returns the possible `Dolby Vision RPU` data contained in a frame.
    pub fn dolby_vision_rpu(&self) -> Option<&[u8]> {
        unsafe {
            slice::from_raw_parts(
                self.frame.DolbyVisionRPU,
                self.frame.DolbyVisionRPUSize as usize,
            )
        }
    }

    /// Returns the possible `HDR10+` dynamic metadata contained in a frame.
    pub fn hdr10_plus(&self) -> Option<&[u8]> {
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
            interlaced_frame: frame.InterlacedFrame > 0,
            top_field_first: frame.TopFieldFirst > 0,
            picture_type: (frame.PictType as u8) as char,
            color_space: PixelFormat::new(frame.ColorSpace),
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
