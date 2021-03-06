// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib;
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
    pub struct Entity(Object<ffi::OsinfoEntity, ffi::OsinfoEntityClass>);

    match fn {
        get_type => || ffi::osinfo_entity_get_type(),
    }
}

pub trait EntityExt {
    fn add_param(&self, key: &str, value: &str);

    fn clear_param(&self, key: &str);

    fn get_id(&self) -> Option<String>;

    fn get_param_keys(&self) -> Vec<String>;

    fn get_param_value(&self, key: &str) -> Option<String>;

    fn get_param_value_boolean(&self, key: &str) -> bool;

    fn get_param_value_boolean_with_default(&self, key: &str, default_value: bool) -> bool;

    fn get_param_value_enum(&self, key: &str, enum_type: glib::types::Type, default_value: i32) -> i32;

    fn get_param_value_int64(&self, key: &str) -> i64;

    fn get_param_value_int64_with_default(&self, key: &str, default_value: i64) -> i64;

    fn get_param_value_list(&self, key: &str) -> Vec<String>;

    fn set_param(&self, key: &str, value: &str);

    fn set_param_boolean(&self, key: &str, value: bool);

    fn set_param_enum(&self, key: &str, value: i32, enum_type: glib::types::Type);

    fn set_param_int64(&self, key: &str, value: i64);

    fn set_property_id(&self, id: Option<&str>);

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Entity> + IsA<glib::object::Object>> EntityExt for O {
    fn add_param(&self, key: &str, value: &str) {
        unsafe {
            ffi::osinfo_entity_add_param(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn clear_param(&self, key: &str) {
        unsafe {
            ffi::osinfo_entity_clear_param(self.to_glib_none().0, key.to_glib_none().0);
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_entity_get_id(self.to_glib_none().0))
        }
    }

    fn get_param_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_entity_get_param_keys(self.to_glib_none().0))
        }
    }

    fn get_param_value(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_entity_get_param_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_param_value_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::osinfo_entity_get_param_value_boolean(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_param_value_boolean_with_default(&self, key: &str, default_value: bool) -> bool {
        unsafe {
            from_glib(ffi::osinfo_entity_get_param_value_boolean_with_default(self.to_glib_none().0, key.to_glib_none().0, default_value.to_glib()))
        }
    }

    fn get_param_value_enum(&self, key: &str, enum_type: glib::types::Type, default_value: i32) -> i32 {
        unsafe {
            ffi::osinfo_entity_get_param_value_enum(self.to_glib_none().0, key.to_glib_none().0, enum_type.to_glib(), default_value)
        }
    }

    fn get_param_value_int64(&self, key: &str) -> i64 {
        unsafe {
            ffi::osinfo_entity_get_param_value_int64(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_param_value_int64_with_default(&self, key: &str, default_value: i64) -> i64 {
        unsafe {
            ffi::osinfo_entity_get_param_value_int64_with_default(self.to_glib_none().0, key.to_glib_none().0, default_value)
        }
    }

    fn get_param_value_list(&self, key: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_entity_get_param_value_list(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn set_param(&self, key: &str, value: &str) {
        unsafe {
            ffi::osinfo_entity_set_param(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_param_boolean(&self, key: &str, value: bool) {
        unsafe {
            ffi::osinfo_entity_set_param_boolean(self.to_glib_none().0, key.to_glib_none().0, value.to_glib());
        }
    }

    fn set_param_enum(&self, key: &str, value: i32, enum_type: glib::types::Type) {
        unsafe {
            ffi::osinfo_entity_set_param_enum(self.to_glib_none().0, key.to_glib_none().0, value, enum_type.to_glib());
        }
    }

    fn set_param_int64(&self, key: &str, value: i64) {
        unsafe {
            ffi::osinfo_entity_set_param_int64(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    fn set_property_id(&self, id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "id".to_glib_none().0, Value::from(id).to_glib_none().0);
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::id",
                transmute(notify_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_id_trampoline<P>(this: *mut ffi::OsinfoEntity, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Entity> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Entity::from_glib_borrow(this).downcast_unchecked())
}
