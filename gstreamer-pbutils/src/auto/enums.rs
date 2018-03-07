// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DiscovererResult {
    Ok,
    UriInvalid,
    Error,
    Timeout,
    Busy,
    MissingPlugins,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DiscovererResult {
    type GlibType = ffi::GstDiscovererResult;

    fn to_glib(&self) -> ffi::GstDiscovererResult {
        match *self {
            DiscovererResult::Ok => ffi::GST_DISCOVERER_OK,
            DiscovererResult::UriInvalid => ffi::GST_DISCOVERER_URI_INVALID,
            DiscovererResult::Error => ffi::GST_DISCOVERER_ERROR,
            DiscovererResult::Timeout => ffi::GST_DISCOVERER_TIMEOUT,
            DiscovererResult::Busy => ffi::GST_DISCOVERER_BUSY,
            DiscovererResult::MissingPlugins => ffi::GST_DISCOVERER_MISSING_PLUGINS,
            DiscovererResult::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstDiscovererResult> for DiscovererResult {
    fn from_glib(value: ffi::GstDiscovererResult) -> Self {
        skip_assert_initialized!();
        match value {
            0 => DiscovererResult::Ok,
            1 => DiscovererResult::UriInvalid,
            2 => DiscovererResult::Error,
            3 => DiscovererResult::Timeout,
            4 => DiscovererResult::Busy,
            5 => DiscovererResult::MissingPlugins,
            value => DiscovererResult::__Unknown(value),
        }
    }
}

impl StaticType for DiscovererResult {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_discoverer_result_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DiscovererResult {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DiscovererResult {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DiscovererResult {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
