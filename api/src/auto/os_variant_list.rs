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
    pub struct OsVariantList(Object<ffi::OsinfoOsVariantList, ffi::OsinfoOsVariantListClass>): List;

    match fn {
        get_type => || ffi::osinfo_os_variantlist_get_type(),
    }
}

impl OsVariantList {
    pub fn new() -> OsVariantList {
        unsafe {
            from_glib_full(ffi::osinfo_os_variantlist_new())
        }
    }
}

impl Default for OsVariantList {
    fn default() -> Self {
        Self::new()
    }
}