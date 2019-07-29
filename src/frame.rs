use crate::*;
use crate::video::*;

use ffms2_sys::*;

use std::ptr;
use std::ffi::CString;
use std::mem;

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

create_enum!(
    ChromaLocations,
    FFMS_ChromaLocations,
    chroma_locations,
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
    ),
    (
        [ptr::null(); 4], [0; 4], 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
    ),
    (
        (cfg(feature = "ffms2-2-21-0"), ColorPrimaries, 0),
        (cfg(feature = "ffms2-2-21-0"), TransferCharateristics, 0),
        (cfg(feature = "ffms2-2-21-0"), ChromaLocation, 0),
        (
            cfg(feature = "ffms2-2-27-0"),
            HasMasteringDisplayPrimaries,
            0
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayPrimariesX,
            [0.0; 3]
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayPrimariesY,
            [0.0; 3]
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayWhitePointX,
            0.0
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayWhitePointY,
            0.0
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            HasMasteringDisplayLuminance,
            0
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayMinLuminance,
            0.0
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayMaxLuminance,
            0.0
        ),
        (cfg(feature = "ffms2-2-27-0"), HasContentLightLevel, 0),
        (cfg(feature = "ffms2-2-27-0"), ContentLightLevelMax, 0),
        (cfg(feature = "ffms2-2-27-0"), ContentLightLevelAverage, 0)
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
    ),
    (   usize, usize, usize, usize, usize, usize, usize,
        usize, usize, usize, i8, usize, usize
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
    )
);

set_feature_params!(
    Frame,
    frame,
    (
        (
            cfg(feature = "ffms2-2-21-0"),
            ColorPrimaries,
            usize,
            ColorPrimaries as i32
        ),
        (
            cfg(feature = "ffms2-2-21-0"),
            TransferCharateristics,
            usize,
            TransferCharateristics as i32
        ),
        (
            cfg(feature = "ffms2-2-21-0"),
            ChromaLocation,
            usize,
            ChromaLocation as i32
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            HasMasteringDisplayPrimaries,
            usize,
            HasMasteringDisplayPrimaries as i32
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayPrimariesX,
            &[f64; 3],
            *MasteringDisplayPrimariesX as [f64; 3]
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayPrimariesY,
            &[f64; 3],
            *MasteringDisplayPrimariesY as [f64; 3]
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayWhitePointX,
            f64,
            MasteringDisplayWhitePointX as f64
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayWhitePointY,
            f64,
            MasteringDisplayWhitePointY as f64
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            HasMasteringDisplayLuminance,
            usize,
            HasMasteringDisplayLuminance as i32
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayMinLuminance,
            f64,
            MasteringDisplayMinLuminance as f64
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            MasteringDisplayMaxLuminance,
            f64,
            MasteringDisplayMaxLuminance as f64
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            HasContentLightLevel,
            usize,
            HasContentLightLevel as i32
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            ContentLightLevelMax,
            usize,
            ContentLightLevelMax as u32
        ),
        (
            cfg(feature = "ffms2-2-27-0"),
            ContentLightLevelAverage,
            usize,
            ContentLightLevelAverage as u32
        )
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
            let ref_frame = unsafe {
                mem::transmute::<*const FFMS_Frame, &FFMS_Frame>(c_frame)
            };

            Ok(Frame { frame: *ref_frame })
        }
    }

    pub fn GetFrameByTime(V: &mut VideoSource, Time: f64) -> Result<Self, Error> {
        let mut error: Error = Default::default();

        let c_frame = unsafe {
            FFMS_GetFrameByTime(V.as_mut_ptr(), Time, error.as_mut_ptr())
        };

        if c_frame.is_null() {
            Err(error)
        } else {
            let ref_frame = unsafe {
                mem::transmute::<*const FFMS_Frame, &FFMS_Frame>(c_frame)
            };

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

    pub fn set_LineSize(&mut self, linesize: &[usize; 4]) {
        self.frame.Linesize = [
            linesize[0] as i32,
            linesize[1] as i32,
            linesize[2] as i32,
            linesize[3] as i32,
        ];
    }
}
