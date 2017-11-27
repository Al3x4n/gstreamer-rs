// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AudioChannelPosition {
    None,
    Mono,
    Invalid,
    FrontLeft,
    FrontRight,
    FrontCenter,
    Lfe1,
    RearLeft,
    RearRight,
    FrontLeftOfCenter,
    FrontRightOfCenter,
    RearCenter,
    Lfe2,
    SideLeft,
    SideRight,
    TopFrontLeft,
    TopFrontRight,
    TopFrontCenter,
    TopCenter,
    TopRearLeft,
    TopRearRight,
    TopSideLeft,
    TopSideRight,
    TopRearCenter,
    BottomFrontCenter,
    BottomFrontLeft,
    BottomFrontRight,
    WideLeft,
    WideRight,
    SurroundLeft,
    SurroundRight,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for AudioChannelPosition {
    type GlibType = ffi::GstAudioChannelPosition;

    fn to_glib(&self) -> ffi::GstAudioChannelPosition {
        match *self {
            AudioChannelPosition::None => ffi::GST_AUDIO_CHANNEL_POSITION_NONE,
            AudioChannelPosition::Mono => ffi::GST_AUDIO_CHANNEL_POSITION_MONO,
            AudioChannelPosition::Invalid => ffi::GST_AUDIO_CHANNEL_POSITION_INVALID,
            AudioChannelPosition::FrontLeft => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT,
            AudioChannelPosition::FrontRight => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT,
            AudioChannelPosition::FrontCenter => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_CENTER,
            AudioChannelPosition::Lfe1 => ffi::GST_AUDIO_CHANNEL_POSITION_LFE1,
            AudioChannelPosition::RearLeft => ffi::GST_AUDIO_CHANNEL_POSITION_REAR_LEFT,
            AudioChannelPosition::RearRight => ffi::GST_AUDIO_CHANNEL_POSITION_REAR_RIGHT,
            AudioChannelPosition::FrontLeftOfCenter => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER,
            AudioChannelPosition::FrontRightOfCenter => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER,
            AudioChannelPosition::RearCenter => ffi::GST_AUDIO_CHANNEL_POSITION_REAR_CENTER,
            AudioChannelPosition::Lfe2 => ffi::GST_AUDIO_CHANNEL_POSITION_LFE2,
            AudioChannelPosition::SideLeft => ffi::GST_AUDIO_CHANNEL_POSITION_SIDE_LEFT,
            AudioChannelPosition::SideRight => ffi::GST_AUDIO_CHANNEL_POSITION_SIDE_RIGHT,
            AudioChannelPosition::TopFrontLeft => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_LEFT,
            AudioChannelPosition::TopFrontRight => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_RIGHT,
            AudioChannelPosition::TopFrontCenter => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_CENTER,
            AudioChannelPosition::TopCenter => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_CENTER,
            AudioChannelPosition::TopRearLeft => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_LEFT,
            AudioChannelPosition::TopRearRight => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_RIGHT,
            AudioChannelPosition::TopSideLeft => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_LEFT,
            AudioChannelPosition::TopSideRight => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_RIGHT,
            AudioChannelPosition::TopRearCenter => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_CENTER,
            AudioChannelPosition::BottomFrontCenter => ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_CENTER,
            AudioChannelPosition::BottomFrontLeft => ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_LEFT,
            AudioChannelPosition::BottomFrontRight => ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_RIGHT,
            AudioChannelPosition::WideLeft => ffi::GST_AUDIO_CHANNEL_POSITION_WIDE_LEFT,
            AudioChannelPosition::WideRight => ffi::GST_AUDIO_CHANNEL_POSITION_WIDE_RIGHT,
            AudioChannelPosition::SurroundLeft => ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_LEFT,
            AudioChannelPosition::SurroundRight => ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT,
            AudioChannelPosition::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioChannelPosition> for AudioChannelPosition {
    fn from_glib(value: ffi::GstAudioChannelPosition) -> Self {
        skip_assert_initialized!();
        match value {
            -3 => AudioChannelPosition::None,
            -2 => AudioChannelPosition::Mono,
            -1 => AudioChannelPosition::Invalid,
            0 => AudioChannelPosition::FrontLeft,
            1 => AudioChannelPosition::FrontRight,
            2 => AudioChannelPosition::FrontCenter,
            3 => AudioChannelPosition::Lfe1,
            4 => AudioChannelPosition::RearLeft,
            5 => AudioChannelPosition::RearRight,
            6 => AudioChannelPosition::FrontLeftOfCenter,
            7 => AudioChannelPosition::FrontRightOfCenter,
            8 => AudioChannelPosition::RearCenter,
            9 => AudioChannelPosition::Lfe2,
            10 => AudioChannelPosition::SideLeft,
            11 => AudioChannelPosition::SideRight,
            12 => AudioChannelPosition::TopFrontLeft,
            13 => AudioChannelPosition::TopFrontRight,
            14 => AudioChannelPosition::TopFrontCenter,
            15 => AudioChannelPosition::TopCenter,
            16 => AudioChannelPosition::TopRearLeft,
            17 => AudioChannelPosition::TopRearRight,
            18 => AudioChannelPosition::TopSideLeft,
            19 => AudioChannelPosition::TopSideRight,
            20 => AudioChannelPosition::TopRearCenter,
            21 => AudioChannelPosition::BottomFrontCenter,
            22 => AudioChannelPosition::BottomFrontLeft,
            23 => AudioChannelPosition::BottomFrontRight,
            24 => AudioChannelPosition::WideLeft,
            25 => AudioChannelPosition::WideRight,
            26 => AudioChannelPosition::SurroundLeft,
            27 => AudioChannelPosition::SurroundRight,
            value => AudioChannelPosition::__Unknown(value),
        }
    }
}

impl StaticType for AudioChannelPosition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_channel_position_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioChannelPosition {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioChannelPosition {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AudioChannelPosition {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AudioFormat {
    Unknown,
    Encoded,
    S8,
    U8,
    S16le,
    S16be,
    U16le,
    U16be,
    S2432le,
    S2432be,
    U2432le,
    U2432be,
    S32le,
    S32be,
    U32le,
    U32be,
    S24le,
    S24be,
    U24le,
    U24be,
    S20le,
    S20be,
    U20le,
    U20be,
    S18le,
    S18be,
    U18le,
    U18be,
    F32le,
    F32be,
    F64le,
    F64be,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for AudioFormat {
    type GlibType = ffi::GstAudioFormat;

    fn to_glib(&self) -> ffi::GstAudioFormat {
        match *self {
            AudioFormat::Unknown => ffi::GST_AUDIO_FORMAT_UNKNOWN,
            AudioFormat::Encoded => ffi::GST_AUDIO_FORMAT_ENCODED,
            AudioFormat::S8 => ffi::GST_AUDIO_FORMAT_S8,
            AudioFormat::U8 => ffi::GST_AUDIO_FORMAT_U8,
            AudioFormat::S16le => ffi::GST_AUDIO_FORMAT_S16LE,
            AudioFormat::S16be => ffi::GST_AUDIO_FORMAT_S16BE,
            AudioFormat::U16le => ffi::GST_AUDIO_FORMAT_U16LE,
            AudioFormat::U16be => ffi::GST_AUDIO_FORMAT_U16BE,
            AudioFormat::S2432le => ffi::GST_AUDIO_FORMAT_S24_32LE,
            AudioFormat::S2432be => ffi::GST_AUDIO_FORMAT_S24_32BE,
            AudioFormat::U2432le => ffi::GST_AUDIO_FORMAT_U24_32LE,
            AudioFormat::U2432be => ffi::GST_AUDIO_FORMAT_U24_32BE,
            AudioFormat::S32le => ffi::GST_AUDIO_FORMAT_S32LE,
            AudioFormat::S32be => ffi::GST_AUDIO_FORMAT_S32BE,
            AudioFormat::U32le => ffi::GST_AUDIO_FORMAT_U32LE,
            AudioFormat::U32be => ffi::GST_AUDIO_FORMAT_U32BE,
            AudioFormat::S24le => ffi::GST_AUDIO_FORMAT_S24LE,
            AudioFormat::S24be => ffi::GST_AUDIO_FORMAT_S24BE,
            AudioFormat::U24le => ffi::GST_AUDIO_FORMAT_U24LE,
            AudioFormat::U24be => ffi::GST_AUDIO_FORMAT_U24BE,
            AudioFormat::S20le => ffi::GST_AUDIO_FORMAT_S20LE,
            AudioFormat::S20be => ffi::GST_AUDIO_FORMAT_S20BE,
            AudioFormat::U20le => ffi::GST_AUDIO_FORMAT_U20LE,
            AudioFormat::U20be => ffi::GST_AUDIO_FORMAT_U20BE,
            AudioFormat::S18le => ffi::GST_AUDIO_FORMAT_S18LE,
            AudioFormat::S18be => ffi::GST_AUDIO_FORMAT_S18BE,
            AudioFormat::U18le => ffi::GST_AUDIO_FORMAT_U18LE,
            AudioFormat::U18be => ffi::GST_AUDIO_FORMAT_U18BE,
            AudioFormat::F32le => ffi::GST_AUDIO_FORMAT_F32LE,
            AudioFormat::F32be => ffi::GST_AUDIO_FORMAT_F32BE,
            AudioFormat::F64le => ffi::GST_AUDIO_FORMAT_F64LE,
            AudioFormat::F64be => ffi::GST_AUDIO_FORMAT_F64BE,
            AudioFormat::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFormat> for AudioFormat {
    fn from_glib(value: ffi::GstAudioFormat) -> Self {
        skip_assert_initialized!();
        match value {
            0 => AudioFormat::Unknown,
            1 => AudioFormat::Encoded,
            2 => AudioFormat::S8,
            3 => AudioFormat::U8,
            4 => AudioFormat::S16le,
            5 => AudioFormat::S16be,
            6 => AudioFormat::U16le,
            7 => AudioFormat::U16be,
            8 => AudioFormat::S2432le,
            9 => AudioFormat::S2432be,
            10 => AudioFormat::U2432le,
            11 => AudioFormat::U2432be,
            12 => AudioFormat::S32le,
            13 => AudioFormat::S32be,
            14 => AudioFormat::U32le,
            15 => AudioFormat::U32be,
            16 => AudioFormat::S24le,
            17 => AudioFormat::S24be,
            18 => AudioFormat::U24le,
            19 => AudioFormat::U24be,
            20 => AudioFormat::S20le,
            21 => AudioFormat::S20be,
            22 => AudioFormat::U20le,
            23 => AudioFormat::U20be,
            24 => AudioFormat::S18le,
            25 => AudioFormat::S18be,
            26 => AudioFormat::U18le,
            27 => AudioFormat::U18be,
            28 => AudioFormat::F32le,
            29 => AudioFormat::F32be,
            30 => AudioFormat::F64le,
            31 => AudioFormat::F64be,
            value => AudioFormat::__Unknown(value),
        }
    }
}

impl StaticType for AudioFormat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_format_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioFormat {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioFormat {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AudioFormat {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AudioLayout {
    Interleaved,
    NonInterleaved,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for AudioLayout {
    type GlibType = ffi::GstAudioLayout;

    fn to_glib(&self) -> ffi::GstAudioLayout {
        match *self {
            AudioLayout::Interleaved => ffi::GST_AUDIO_LAYOUT_INTERLEAVED,
            AudioLayout::NonInterleaved => ffi::GST_AUDIO_LAYOUT_NON_INTERLEAVED,
            AudioLayout::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioLayout> for AudioLayout {
    fn from_glib(value: ffi::GstAudioLayout) -> Self {
        skip_assert_initialized!();
        match value {
            0 => AudioLayout::Interleaved,
            1 => AudioLayout::NonInterleaved,
            value => AudioLayout::__Unknown(value),
        }
    }
}

impl StaticType for AudioLayout {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_layout_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioLayout {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioLayout {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AudioLayout {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StreamVolumeFormat {
    Linear,
    Cubic,
    Db,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StreamVolumeFormat {
    type GlibType = ffi::GstStreamVolumeFormat;

    fn to_glib(&self) -> ffi::GstStreamVolumeFormat {
        match *self {
            StreamVolumeFormat::Linear => ffi::GST_STREAM_VOLUME_FORMAT_LINEAR,
            StreamVolumeFormat::Cubic => ffi::GST_STREAM_VOLUME_FORMAT_CUBIC,
            StreamVolumeFormat::Db => ffi::GST_STREAM_VOLUME_FORMAT_DB,
            StreamVolumeFormat::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamVolumeFormat> for StreamVolumeFormat {
    fn from_glib(value: ffi::GstStreamVolumeFormat) -> Self {
        skip_assert_initialized!();
        match value {
            0 => StreamVolumeFormat::Linear,
            1 => StreamVolumeFormat::Cubic,
            2 => StreamVolumeFormat::Db,
            value => StreamVolumeFormat::__Unknown(value),
        }
    }
}

