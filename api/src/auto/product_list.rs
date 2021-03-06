// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use Filter;
use List;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ProductList(Object<ffi::OsinfoProductList, ffi::OsinfoProductListClass>): List;

    match fn {
        get_type => || ffi::osinfo_productlist_get_type(),
    }
}

impl ProductList {
    pub fn new() -> ProductList {
        unsafe {
            from_glib_full(ffi::osinfo_productlist_new())
        }
    }
}

impl Default for ProductList {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ProductListExt {
    fn new_copy(&self) -> Option<ProductList>;

    fn new_filtered<P: IsA<Filter>>(&self, filter: &P) -> Option<ProductList>;

    fn new_intersection<P: IsA<ProductList>>(&self, sourceTwo: &P) -> Option<ProductList>;

    fn new_union<P: IsA<ProductList>>(&self, sourceTwo: &P) -> Option<ProductList>;
}

impl<O: IsA<ProductList>> ProductListExt for O {
    fn new_copy(&self) -> Option<ProductList> {
        unsafe {
            from_glib_full(ffi::osinfo_productlist_new_copy(self.to_glib_none().0))
        }
    }

    fn new_filtered<P: IsA<Filter>>(&self, filter: &P) -> Option<ProductList> {
        unsafe {
            from_glib_full(ffi::osinfo_productlist_new_filtered(self.to_glib_none().0, filter.to_glib_none().0))
        }
    }

    fn new_intersection<P: IsA<ProductList>>(&self, sourceTwo: &P) -> Option<ProductList> {
        unsafe {
            from_glib_full(ffi::osinfo_productlist_new_intersection(self.to_glib_none().0, sourceTwo.to_glib_none().0))
        }
    }

    fn new_union<P: IsA<ProductList>>(&self, sourceTwo: &P) -> Option<ProductList> {
        unsafe {
            from_glib_full(ffi::osinfo_productlist_new_union(self.to_glib_none().0, sourceTwo.to_glib_none().0))
        }
    }
}
