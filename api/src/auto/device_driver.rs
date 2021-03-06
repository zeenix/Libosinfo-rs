// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use DeviceList;
use Entity;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DeviceDriver(Object<ffi::OsinfoDeviceDriver, ffi::OsinfoDeviceDriverClass>): Entity;

    match fn {
        get_type => || ffi::osinfo_device_driver_get_type(),
    }
}

pub trait DeviceDriverExt {
    fn get_architecture(&self) -> Option<String>;

    fn get_devices(&self) -> Option<DeviceList>;

    fn get_files(&self) -> Vec<String>;

    fn get_location(&self) -> Option<String>;

    fn get_pre_installable(&self) -> bool;

    fn get_signed(&self) -> bool;
}

impl<O: IsA<DeviceDriver>> DeviceDriverExt for O {
    fn get_architecture(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_device_driver_get_architecture(self.to_glib_none().0))
        }
    }

    fn get_devices(&self) -> Option<DeviceList> {
        unsafe {
            from_glib_none(ffi::osinfo_device_driver_get_devices(self.to_glib_none().0))
        }
    }

    fn get_files(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_device_driver_get_files(self.to_glib_none().0))
        }
    }

    fn get_location(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_device_driver_get_location(self.to_glib_none().0))
        }
    }

    fn get_pre_installable(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_device_driver_get_pre_installable(self.to_glib_none().0))
        }
    }

    fn get_signed(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_device_driver_get_signed(self.to_glib_none().0))
        }
    }
}
