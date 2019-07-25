use crate::*;

use ffms2_sys::*;

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

pub struct Frame<'a> {
    frame: FFMS_Frame,
    data: &'a [u8],
}
