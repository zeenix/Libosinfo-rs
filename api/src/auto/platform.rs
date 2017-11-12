// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use Device;
use DeviceLink;
use DeviceLinkList;
use DeviceList;
use Entity;
use Filter;
use Product;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Platform(Object<ffi::OsinfoPlatform, ffi::OsinfoPlatformClass>): Product, Entity;

    match fn {
        get_type => || ffi::osinfo_platform_get_type(),
    }
}

impl Platform {
    pub fn new(id: &str) -> Platform {
        unsafe {
            from_glib_full(ffi::osinfo_platform_new(id.to_glib_none().0))
        }
    }
}

pub trait PlatformExt {
    fn add_device(&self, dev: &Device) -> Option<DeviceLink>;

    fn get_all_devices<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceList>;

    fn get_device_links<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceLinkList>;

    fn get_devices<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceList>;
}

impl<O: IsA<Platform>> PlatformExt for O {
    fn add_device(&self, dev: &Device) -> Option<DeviceLink> {
        unsafe {
            from_glib_none(ffi::osinfo_platform_add_device(self.to_glib_none().0, dev.to_glib_none().0))
        }
    }

    fn get_all_devices<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceList> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::osinfo_platform_get_all_devices(self.to_glib_none().0, filter.0))
        }
    }

    fn get_device_links<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceLinkList> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::osinfo_platform_get_device_links(self.to_glib_none().0, filter.0))
        }
    }

    fn get_devices<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceList> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::osinfo_platform_get_devices(self.to_glib_none().0, filter.0))
        }
    }
}
