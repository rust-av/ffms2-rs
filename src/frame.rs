use crate::video::*;
use crate::*;

use ffmpeg_the_third::ffi::AVPixelFormat;
use ffmpeg_the_third::format::Pixel;
use ffms2_sys::*;

use std::ffi::CString;
use std::panic;
use std::ptr;
use std::slice;

create_enum!(
    Resizers,
    FFMS_Resizers,
    resizers,
    (
        RESIZER_FAST_BILINEAR,
        RESIZER_BILINEAR,
        RESIZER_BICUBIC,
        RESIZER_X,
        RESIZER_POINT,
        RESIZER_AREA,
        RESIZER_BICUBLIN,
        RESIZER_GAUSS,
        RESIZER_SINC,
        RESIZER_LANCZOS,
        RESIZER_SPLINE,
    )
);

simple_enum!(
    ChromaLocations,
    (
        LOC_UNSPECIFIED,
        LOC_LEFT,
        LOC_CENTER,
        LOC_TOPLEFT,
        LOC_TOP,
        LOC_BOTTOMLEFT,
        LOC_BOTTOM,
    )
);

create_struct!(
    FrameInfo,
    frame_info,
    FFMS_FrameInfo,
    (PTS, RepeatPict, KeyFrame, OriginalPTS),
    (0, 0, 0, 0)
);

impl FrameInfo {
    pub fn KeyFrame(&self) -> usize {
        self.frame_info.KeyFrame as usize
    }

    pub(crate) fn create_struct(frame_info: &FFMS_FrameInfo) -> Self {
        FrameInfo {
            frame_info: *frame_info,
        }
    }
}

create_struct!(
    Frame,
    frame,
    FFMS_Frame,
    (
        Data,
        Linesize,
        EncodedWidth,
        EncodedHeight,
        EncodedPixelFormat,
        ScaledWidth,
        ScaledHeight,
        ConvertedPixelFormat,
        KeyFrame,
        RepeatPict,
        InterlacedFrame,
        TopFieldFirst,
        PictType,
        ColorSpace,
        ColorRange,
        ColorPrimaries,
        TransferCharateristics,
        ChromaLocation,
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
        ContentLightLevelAverage
    ),
    (
        [ptr::null(); 4],
        [0; 4],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0.0; 3],
        [0.0; 3],
        0.0,
        0.0,
        0,
        0.0,
        0.0,
        0,
        0,
        0
    )
);

pub struct FrameResolution {
    pub width: i32,
    pub height: i32,
}

impl Frame {
    pub fn GetFrame(V: &mut VideoSource, n: usize) -> Result<Self, Error> {
        let mut error: Error = Default::default();

        let c_frame = unsafe {
            FFMS_GetFrame(V.as_mut_ptr(), n as i32, error.as_mut_ptr())
        };

        if c_frame.is_null() {
            Err(error)
        } else {
            let ref_frame = unsafe { &*c_frame };

            Ok(Frame { frame: *ref_frame })
        }
    }

    pub fn GetFrameByTime(
        V: &mut VideoSource,
        Time: f64,
    ) -> Result<Self, Error> {
        let mut error: Error = Default::default();

        let c_frame = unsafe {
            FFMS_GetFrameByTime(V.as_mut_ptr(), Time, error.as_mut_ptr())
        };

        if c_frame.is_null() {
            Err(error)
        } else {
            let ref_frame = unsafe { &*c_frame };

            Ok(Frame { frame: *ref_frame })
        }
    }

    pub fn GetPixFmt(Name: &str) -> i32 {
        let source = CString::new(Name).unwrap();
        unsafe { FFMS_GetPixFmt(source.as_ptr()) }
    }

    pub fn set_data(&mut self, data: [&[u8]; 4]) {
        self.frame.Data = [
            data[0].as_ptr(),
            data[1].as_ptr(),
            data[2].as_ptr(),
            data[3].as_ptr(),
        ];
    }

    pub fn get_frame_resolution(&self) -> FrameResolution {
        let width = if self.frame.ScaledWidth == -1 {
            self.frame.EncodedWidth
        } else {
            self.frame.ScaledWidth
        };
        let height = if self.frame.ScaledHeight == -1 {
            self.frame.EncodedHeight
        } else {
            self.frame.ScaledHeight
        };

        FrameResolution { width, height }
    }

    pub fn get_pixel_data(&self) -> Result<Vec<Option<&[u8]>>, String> {
        let data = self.frame.Data;
        let num_planes = 4;
        let mut data_vec = Vec::with_capacity(num_planes);
        let linesize = self.frame.Linesize;

        // very sketchy, try to find a better way
        let pix_fmt: AVPixelFormat =
            unsafe { mem::transmute(self.frame.EncodedPixelFormat) };
        let pix_fmt = match panic::catch_unwind(|| Pixel::from(pix_fmt)) {
            Ok(pix_fmt) => pix_fmt,
            Err(_) => {
                return Err("Couldn't find pixel format".to_string());
            }
        };

        let log2_chroma_h = match Pixel::from(pix_fmt).descriptor() {
            Some(pix_fmt_descriptor) => pix_fmt_descriptor.log2_chroma_h(),
            None => {
                return Err(
                    "Couldn't find pixel format descriptor".to_string()
                );
            }
        };

        for i in 0..num_planes {
            if linesize[i] == 0 {
                data_vec.push(None);
            } else {
                let sub_h = if i == 0 { 1 } else { 1 << log2_chroma_h };
                let plane_slice_length =
                    linesize[i] * self.EncodedHeight / sub_h;
                let plane_slice = unsafe {
                    slice::from_raw_parts(data[i], plane_slice_length as usize)
                };

                data_vec.push(Some(plane_slice));
            }
        }

        Ok(data_vec)
    }

    pub fn set_LineSize(&mut self, linesize: &[usize; 4]) {
        self.frame.Linesize = [
            linesize[0] as i32,
            linesize[1] as i32,
            linesize[2] as i32,
            linesize[3] as i32,
        ];
    }
}
