use std::mem;
use std::slice;

use std::ffi::CString;

use ffmpeg_the_third::ffi::AVPixelFormat;
use ffmpeg_the_third::format::Pixel;

use ffms2_sys::{FFMS_Frame, FFMS_FrameInfo, FFMS_Resizers};

use crate::error::{InternalError, Result};
use crate::video::{ColorRange, VideoSource};

const PLANES_NUMBER: usize = 4;

#[derive(Clone, Copy, Debug)]
pub enum Resizers {
    FastBilinear,
    Bilinear,
    Bicubic,
    X,
    Point,
    Area,
    Bicublin,
    Gauss,
    Sinc,
    Lanczos,
    Spline,
}

impl Resizers {
    pub(crate) const fn ffms2_resizer(self) -> FFMS_Resizers {
        match self {
            Self::FastBilinear => FFMS_Resizers::FFMS_RESIZER_FAST_BILINEAR,
            Self::Bilinear => FFMS_Resizers::FFMS_RESIZER_BILINEAR,
            Self::Bicubic => FFMS_Resizers::FFMS_RESIZER_BICUBIC,
            Self::X => FFMS_Resizers::FFMS_RESIZER_X,
            Self::Point => FFMS_Resizers::FFMS_RESIZER_POINT,
            Self::Area => FFMS_Resizers::FFMS_RESIZER_AREA,
            Self::Bicublin => FFMS_Resizers::FFMS_RESIZER_BICUBLIN,
            Self::Gauss => FFMS_Resizers::FFMS_RESIZER_GAUSS,
            Self::Sinc => FFMS_Resizers::FFMS_RESIZER_SINC,
            Self::Lanczos => FFMS_Resizers::FFMS_RESIZER_LANCZOS,
            Self::Spline => FFMS_Resizers::FFMS_RESIZER_SPLINE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ChromaLocation {
    #[default]
    Unspecified,
    Left,
    Center,
    TopLeft,
    Top,
    BottomLeft,
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

#[derive(Debug)]
pub struct FrameInfo(FFMS_FrameInfo);

impl FrameInfo {
    pub const fn pts(&self) -> usize {
        self.0.PTS as usize
    }

    pub const fn repeat_picture(&self) -> usize {
        self.0.RepeatPict as usize
    }

    pub const fn keyframe(&self) -> usize {
        self.0.KeyFrame as usize
    }

    pub const fn original_pts(&self) -> usize {
        self.0.OriginalPTS as usize
    }

    pub(crate) fn new(frame_info: &FFMS_FrameInfo) -> Self {
        FrameInfo(*frame_info)
    }
}

#[derive(Debug)]
pub struct FrameResolution {
    width: i32,
    height: i32,
}

impl FrameResolution {
    pub const fn width(&self) -> usize {
        self.width as usize
    }

    pub const fn height(&self) -> usize {
        self.height as usize
    }
}

#[derive(Debug)]
pub struct Frame(FFMS_Frame);

impl Frame {
    pub const fn linesize(&self) -> [usize; PLANES_NUMBER] {
        [
            self.0.Linesize[0] as usize,
            self.0.Linesize[1] as usize,
            self.0.Linesize[2] as usize,
            self.0.Linesize[3] as usize,
        ]
    }

    pub const fn encoded_width(&self) -> usize {
        self.0.EncodedWidth as usize
    }

    pub const fn encoded_height(&self) -> usize {
        self.0.EncodedHeight as usize
    }

    pub const fn encoded_pixel_format(&self) -> usize {
        self.0.EncodedPixelFormat as usize
    }

    pub const fn scaled_width(&self) -> usize {
        self.0.ScaledWidth as usize
    }

    pub const fn scaled_height(&self) -> usize {
        self.0.ScaledHeight as usize
    }

    pub const fn converted_pixel_format(&self) -> usize {
        self.0.ConvertedPixelFormat as usize
    }

    pub const fn keyframe(&self) -> usize {
        self.0.KeyFrame as usize
    }

    pub const fn repeat_picture(&self) -> usize {
        self.0.RepeatPict as usize
    }

    pub const fn interlaced_frame(&self) -> usize {
        self.0.InterlacedFrame as usize
    }

    pub const fn top_field_first(&self) -> usize {
        self.0.TopFieldFirst as usize
    }

    pub const fn picture_type(&self) -> u8 {
        self.0.PictType as u8
    }

    pub const fn colorspace(&self) -> usize {
        self.0.ColorSpace as usize
    }

    pub const fn color_range(&self) -> ColorRange {
        ColorRange::new(self.0.ColorRange)
    }

    pub const fn color_primaries(&self) -> usize {
        self.0.ColorPrimaries as usize
    }

    pub const fn transfer_characteristics(&self) -> usize {
        self.0.TransferCharateristics as usize
    }

    pub const fn chroma_location(&self) -> ChromaLocation {
        ChromaLocation::new(self.0.ChromaLocation)
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

    pub const fn frame_resolution(&self) -> FrameResolution {
        let width = if self.0.ScaledWidth == -1 {
            self.0.EncodedWidth
        } else {
            self.0.ScaledWidth
        };
        let height = if self.0.ScaledHeight == -1 {
            self.0.EncodedHeight
        } else {
            self.0.ScaledHeight
        };

        FrameResolution { width, height }
    }

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

            Ok(Self(*ref_frame))
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

            Ok(Self(*ref_frame))
        }
    }

    pub fn pixel_format(name: &str) -> Pixel {
        let source = CString::new(name).unwrap();
        let pixel_format =
            unsafe { ffms2_sys::FFMS_GetPixFmt(source.as_ptr()) };
        Self::i32_to_pixel_format(pixel_format)
    }

    pub fn dolby_vision_rpu(&self) -> &[u8] {
        let rpu_slice = unsafe {
            slice::from_raw_parts(
                self.0.DolbyVisionRPU,
                self.0.DolbyVisionRPUSize as usize,
            )
        };
        rpu_slice
    }

    pub fn hdr10_plus(&self) -> &[u8] {
        let rpu_slice = unsafe {
            slice::from_raw_parts(
                self.0.HDR10Plus,
                self.0.HDR10PlusSize as usize,
            )
        };
        rpu_slice
    }

    pub fn planes(&self) -> Option<[Option<&[u8]>; PLANES_NUMBER]> {
        let mut planes: [Option<&[u8]>; PLANES_NUMBER] = Default::default();

        let log2_chroma_h =
            match Self::i32_to_pixel_format(self.0.EncodedPixelFormat)
                .descriptor()
            {
                Some(pix_fmt_descriptor) => pix_fmt_descriptor.log2_chroma_h(),
                None => return None,
            };

        for (i, (plane, (data, linesize))) in planes
            .iter_mut()
            .zip(self.0.Data.into_iter().zip(self.0.Linesize.into_iter()))
            .enumerate()
        {
            if linesize == 0 {
                *plane = None;
            } else {
                let sub_h = if i == 0 { 0 } else { log2_chroma_h };
                let plane_slice_length =
                    (linesize * self.0.EncodedHeight) >> sub_h;
                let plane_slice = unsafe {
                    slice::from_raw_parts(data, plane_slice_length as usize)
                };

                *plane = Some(plane_slice);
            }
        }

        Some(planes)
    }

    fn i32_to_pixel_format(i32_pixel: i32) -> Pixel {
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
    }
}
