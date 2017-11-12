// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use Device;
use DeviceLink;
use DeviceLinkList;
use DeviceList;
use Entity;
use Filter;
use Os;
use Platform;
use ffi;
use glib;
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
    pub struct Deployment(Object<ffi::OsinfoDeployment, ffi::OsinfoDeploymentClass>): Entity;

    match fn {
        get_type => || ffi::osinfo_deployment_get_type(),
    }
}

impl Deployment {
    pub fn new(id: &str, os: &Os, platform: &Platform) -> Deployment {
        unsafe {
            from_glib_full(ffi::osinfo_deployment_new(id.to_glib_none().0, os.to_glib_none().0, platform.to_glib_none().0))
        }
    }
}

pub trait DeploymentExt {
    fn add_device(&self, dev: &Device) -> Option<DeviceLink>;

    fn get_device_links<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceLinkList>;

    fn get_devices<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceList>;

    fn get_os(&self) -> Option<Os>;

    fn get_platform(&self) -> Option<Platform>;

    fn get_preferred_device<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<Device>;

    fn get_preferred_device_link<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceLink>;

    fn connect_property_os_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_platform_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Deployment> + IsA<glib::object::Object>> DeploymentExt for O {
    fn add_device(&self, dev: &Device) -> Option<DeviceLink> {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_add_device(self.to_glib_none().0, dev.to_glib_none().0))
        }
    }

    fn get_device_links<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceLinkList> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::osinfo_deployment_get_device_links(self.to_glib_none().0, filter.0))
        }
    }

    fn get_devices<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceList> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::osinfo_deployment_get_devices(self.to_glib_none().0, filter.0))
        }
    }

    fn get_os(&self) -> Option<Os> {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_os(self.to_glib_none().0))
        }
    }

    fn get_platform(&self) -> Option<Platform> {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_platform(self.to_glib_none().0))
        }
    }

    fn get_preferred_device<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<Device> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_preferred_device(self.to_glib_none().0, filter.0))
        }
    }

    fn get_preferred_device_link<'a, P: IsA<Filter> + 'a, Q: Into<Option<&'a P>>>(&self, filter: Q) -> Option<DeviceLink> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_preferred_device_link(self.to_glib_none().0, filter.0))
        }
    }

    fn connect_property_os_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::os",
                transmute(notify_os_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_platform_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::platform",
                transmute(notify_platform_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_os_trampoline<P>(this: *mut ffi::OsinfoDeployment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Deployment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Deployment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_platform_trampoline<P>(this: *mut ffi::OsinfoDeployment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Deployment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Deployment::from_glib_borrow(this).downcast_unchecked())
}
