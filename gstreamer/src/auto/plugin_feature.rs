// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use Object;
use Plugin;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PluginFeature(Object<ffi::GstPluginFeature, ffi::GstPluginFeatureClass>): Object;

    match fn {
        get_type => || ffi::gst_plugin_feature_get_type(),
    }
}

unsafe impl Send for PluginFeature {}
unsafe impl Sync for PluginFeature {}

pub trait PluginFeatureExt {
    fn check_version(&self, min_major: u32, min_minor: u32, min_micro: u32) -> bool;

    fn get_plugin(&self) -> Option<Plugin>;

    fn get_plugin_name(&self) -> Option<String>;

    fn get_rank(&self) -> u32;

    fn load(&self) -> Option<PluginFeature>;

    fn set_rank(&self, rank: u32);
}

impl<O: IsA<PluginFeature>> PluginFeatureExt for O {
    fn check_version(&self, min_major: u32, min_minor: u32, min_micro: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_plugin_feature_check_version(self.to_glib_none().0, min_major, min_minor, min_micro))
        }
    }

    fn get_plugin(&self) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_plugin_feature_get_plugin(self.to_glib_none().0))
        }
    }

    fn get_plugin_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_feature_get_plugin_name(self.to_glib_none().0))
        }
    }

    fn get_rank(&self) -> u32 {
        unsafe {
            ffi::gst_plugin_feature_get_rank(self.to_glib_none().0)
        }
    }

    fn load(&self) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(ffi::gst_plugin_feature_load(self.to_glib_none().0))
        }
    }

    fn set_rank(&self, rank: u32) {
        unsafe {
            ffi::gst_plugin_feature_set_rank(self.to_glib_none().0, rank);
        }
    }
}
