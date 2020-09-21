use crate::video::*;
use crate::*;

use ffms2_sys::*;

use std::ffi::CString;
use std::ptr;

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
    (usize, usize, usize, usize),
    (0, 0, 0, 0),
    (
        PTS as i64,
        RepeatPict as i32,
        KeyFrame as i32,
        OriginalPTS as i64
    )
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

set_struct!(Frame, frame, FFMS_Frame);

default_struct!(
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

set_params!(
    Frame,
    frame,
    (
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
        usize, usize, usize, usize, usize, usize, usize, usize, usize, usize,
        i8, usize, usize, usize, usize, usize, usize, &[f64; 3], &[f64; 3],
        f64, f64, usize, f64, f64, usize, usize, usize
    ),
    (
        EncodedWidth as i32,
        EncodedHeight as i32,
        EncodedPixelFormat as i32,
        ScaledWidth as i32,
        ScaledHeight as i32,
        ConvertedPixelFormat as i32,
        KeyFrame as i32,
        RepeatPict as i32,
        InterlacedFrame as i32,
        TopFieldFirst as i32,
        PictType as i8,
        ColorSpace as i32,
        ColorRange as i32,
        ColorPrimaries as i32,
        TransferCharateristics as i32,
        ChromaLocation as i32,
        HasMasteringDisplayPrimaries as i32,
        *MasteringDisplayPrimariesX as [f64; 3],
        *MasteringDisplayPrimariesY as [f64; 3],
        MasteringDisplayWhitePointX as f64,
        MasteringDisplayWhitePointY as f64,
        HasMasteringDisplayLuminance as i32,
        MasteringDisplayMinLuminance as f64,
        MasteringDisplayMaxLuminance as f64,
        HasContentLightLevel as i32,
        ContentLightLevelMax as u32,
        ContentLightLevelAverage as u32
    )
);

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

    pub fn Data(&self) -> Vec<&u8> {
        let data = self.frame.Data;
        unsafe { vec![&*data[0], &*data[1], &*data[2], &*data[3]] }
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
