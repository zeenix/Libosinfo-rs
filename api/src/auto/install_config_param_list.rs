// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use List;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct InstallConfigParamList(Object<ffi::OsinfoInstallConfigParamList, ffi::OsinfoInstallConfigParamListClass>): List;

    match fn {
        get_type => || ffi::osinfo_install_config_paramlist_get_type(),
    }
}

impl InstallConfigParamList {
    pub fn new() -> InstallConfigParamList {
        unsafe {
            from_glib_full(ffi::osinfo_install_config_paramlist_new())
        }
    }
}

impl Default for InstallConfigParamList {
    fn default() -> Self {
        Self::new()
    }
}
