// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use Caps;
use Element;
use Object;
use Structure;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Device(Object<ffi::GstDevice, ffi::GstDeviceClass>): Object;

    match fn {
        get_type => || ffi::gst_device_get_type(),
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

pub trait DeviceExt {
    fn create_element<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<Element>;

    fn get_caps(&self) -> Option<Caps>;

    fn get_device_class(&self) -> Option<String>;

    fn get_display_name(&self) -> Option<String>;

    fn get_properties(&self) -> Option<Structure>;

    fn has_classes(&self, classes: &str) -> bool;

    fn has_classesv(&self, classes: &[&str]) -> bool;

    fn reconfigure_element<P: IsA<Element>>(&self, element: &P) -> bool;

    fn get_property_caps(&self) -> Option<Caps>;

    fn get_property_device_class(&self) -> Option<String>;

    fn get_property_display_name(&self) -> Option<String>;

    fn get_property_properties(&self) -> Option<Structure>;

    fn connect_removed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_device_class_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_display_name_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_properties_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Device> + IsA<glib::object::Object>> DeviceExt for O {
    fn create_element<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<Element> {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_device_create_element(self.to_glib_none().0, name.0))
        }
    }

    fn get_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_device_get_caps(self.to_glib_none().0))
        }
    }

    fn get_device_class(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_device_get_device_class(self.to_glib_none().0))
        }
    }

    fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_device_get_display_name(self.to_glib_none().0))
        }
    }

    fn get_properties(&self) -> Option<Structure> {
        unsafe {
            from_glib_full(ffi::gst_device_get_properties(self.to_glib_none().0))
        }
    }

    fn has_classes(&self, classes: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_device_has_classes(self.to_glib_none().0, classes.to_glib_none().0))
        }
    }

    fn has_classesv(&self, classes: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::gst_device_has_classesv(self.to_glib_none().0, classes.to_glib_none().0))
        }
    }

    fn reconfigure_element<P: IsA<Element>>(&self, element: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_device_reconfigure_element(self.to_glib_none().0, element.to_glib_none().0))
        }
    }

    fn get_property_caps(&self) -> Option<Caps> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <Caps as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "caps".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_device_class(&self) -> Option<String> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <String as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "device-class".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_display_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <String as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "display-name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_properties(&self) -> Option<Structure> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <Structure as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "properties".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_removed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "removed",
                transmute(removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caps",
                transmute(notify_caps_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_device_class_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::device-class",
                transmute(notify_device_class_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_display_name_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display-name",
                transmute(notify_display_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_properties_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::properties",
                transmute(notify_properties_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn removed_trampoline<P>(this: *mut ffi::GstDevice, f: glib_ffi::gpointer)
where P: IsA<Device> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_caps_trampoline<P>(this: *mut ffi::GstDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_device_class_trampoline<P>(this: *mut ffi::GstDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_display_name_trampoline<P>(this: *mut ffi::GstDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_properties_trampoline<P>(this: *mut ffi::GstDevice, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Device> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Device::from_glib_borrow(this).downcast_unchecked())
}
