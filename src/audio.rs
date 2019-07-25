use crate::*;

use ffms2_sys::*;

create_enum!(
    AudioChannel,
    FFMS_AudioChannel,
    audio_channel,
    (
        CH_FRONT_LEFT,
        CH_FRONT_RIGHT,
        CH_FRONT_CENTER,
        CH_LOW_FREQUENCY,
        CH_BACK_LEFT,
        CH_BACK_RIGHT,
        CH_FRONT_LEFT_OF_CENTER,
        CH_FRONT_RIGHT_OF_CENTER,
        CH_BACK_CENTER,
        CH_SIDE_LEFT,
        CH_SIDE_RIGHT,
        CH_TOP_CENTER,
        CH_TOP_FRONT_LEFT,
        CH_TOP_FRONT_CENTER,
        CH_TOP_FRONT_RIGHT,
        CH_TOP_BACK_LEFT,
        CH_TOP_BACK_CENTER,
        CH_TOP_BACK_RIGHT,
        CH_STEREO_LEFT,
        CH_STEREO_RIGHT,
    )
);

create_enum!(
    AudioDelay,
    FFMS_AudioDelayModes,
    audio_delay_modes,
    (DELAY_NO_SHIFT, DELAY_TIME_ZERO, DELAY_FIRST_VIDEO_TRACK)
);

set_struct!(AudioProperties, audio_properties, FFMS_AudioProperties);

default_struct!(
    AudioProperties,
    audio_properties,
    FFMS_AudioProperties,
    (
        SampleFormat,
        SampleRate,
        BitsPerSample,
        Channels,
        ChannelLayout,
        NumSamples,
        FirstTime,
        LastTime,
    ),
    (0, 0, 0, 0, 0, 0, 0.0, 0.0,),
    ((cfg(feature = "ffms2-2-30-0"), LastEndTime, 0.0))
);

set_params!(
    AudioProperties,
    audio_properties,
    (
        SampleFormat,
        SampleRate,
        BitsPerSample,
        Channels,
        ChannelLayout,
        NumSamples,
        FirstTime,
        LastTime,
    ),
    (usize, usize, usize, usize, usize, usize, f64, f64,),
    (
        SampleFormat as i32,
        SampleRate as i32,
        BitsPerSample as i32,
        Channels as i32,
        ChannelLayout as i64,
        NumSamples as i64,
        FirstTime as f64,
        LastTime as f64,
    )
);

set_feature_params!(
    AudioProperties,
    audio_properties,
    (cfg(feature = "ffms2-2-30-0"), LastEndTime, f64, 0.0)
);
