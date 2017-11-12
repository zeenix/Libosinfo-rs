// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use Entity;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Filter(Object<ffi::OsinfoFilter, ffi::OsinfoFilterClass>);

    match fn {
        get_type => || ffi::osinfo_filter_get_type(),
    }
}

impl Filter {
    pub fn new() -> Filter {
        unsafe {
            from_glib_full(ffi::osinfo_filter_new())
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FilterExt {
    fn add_constraint(&self, propName: &str, propVal: &str);

    fn clear_constraint(&self, propName: &str);

    fn clear_constraints(&self);

    fn get_constraint_keys(&self) -> Vec<String>;

    fn get_constraint_values(&self, propName: &str) -> Vec<String>;

    fn matches<P: IsA<Entity>>(&self, entity: &P) -> bool;
}

impl<O: IsA<Filter>> FilterExt for O {
    fn add_constraint(&self, propName: &str, propVal: &str) {
        unsafe {
            ffi::osinfo_filter_add_constraint(self.to_glib_none().0, propName.to_glib_none().0, propVal.to_glib_none().0);
        }
    }

    fn clear_constraint(&self, propName: &str) {
        unsafe {
            ffi::osinfo_filter_clear_constraint(self.to_glib_none().0, propName.to_glib_none().0);
        }
    }

    fn clear_constraints(&self) {
        unsafe {
            ffi::osinfo_filter_clear_constraints(self.to_glib_none().0);
        }
    }

    fn get_constraint_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_filter_get_constraint_keys(self.to_glib_none().0))
        }
    }

    fn get_constraint_values(&self, propName: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_filter_get_constraint_values(self.to_glib_none().0, propName.to_glib_none().0))
        }
    }

    fn matches<P: IsA<Entity>>(&self, entity: &P) -> bool {
        unsafe {
            from_glib(ffi::osinfo_filter_matches(self.to_glib_none().0, entity.to_glib_none().0))
        }
    }
}
