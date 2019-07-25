use crate::*;

create_enum!(
    TrackType,
    FFMS_TrackType,
    track_type,
    (
        TYPE_UNKNOWN,
        TYPE_VIDEO,
        TYPE_AUDIO,
        TYPE_DATA,
        TYPE_SUBTITLE,
        TYPE_ATTACHMENT,
    )
);

create_struct!(
    TrackTimeBase,
    track_time_base,
    FFMS_TrackTimeBase,
    (Num, Den),
    (usize, usize),
    (0, 0),
    (Num as i64, Den as i64)
);
