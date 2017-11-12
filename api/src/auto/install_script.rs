// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use AvatarFormat;
use Entity;
use Error;
use InstallConfig;
use InstallConfigParam;
use InstallConfigParamList;
use Media;
use Os;
use PathFormat;
use ffi;
use gio;
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
    pub struct InstallScript(Object<ffi::OsinfoInstallScript, ffi::OsinfoInstallScriptClass>): Entity;

    match fn {
        get_type => || ffi::osinfo_install_script_get_type(),
    }
}

impl InstallScript {
    pub fn new(id: &str) -> InstallScript {
        unsafe {
            from_glib_full(ffi::osinfo_install_script_new(id.to_glib_none().0))
        }
    }

    pub fn new_data(id: &str, profile: &str, templateData: &str) -> InstallScript {
        unsafe {
            from_glib_full(ffi::osinfo_install_script_new_data(id.to_glib_none().0, profile.to_glib_none().0, templateData.to_glib_none().0))
        }
    }

    pub fn new_uri(id: &str, profile: &str, templateUri: &str) -> InstallScript {
        unsafe {
            from_glib_full(ffi::osinfo_install_script_new_uri(id.to_glib_none().0, profile.to_glib_none().0, templateUri.to_glib_none().0))
        }
    }
}

pub trait InstallScriptExt {
    fn generate<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, os: &Os, config: &InstallConfig, cancellable: P) -> Result<String, Error>;

    //fn generate_async<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, os: &Os, config: &InstallConfig, cancellable: P, callback: Q, user_data: R);

    fn generate_command_line(&self, os: &Os, config: &InstallConfig) -> Option<String>;

    fn generate_command_line_for_media(&self, media: &Media, config: &InstallConfig) -> Option<String>;

    //fn generate_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<String, Error>;

    fn generate_for_media<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, media: &Media, config: &InstallConfig, cancellable: P) -> Result<String, Error>;

    //fn generate_for_media_async<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, media: &Media, config: &InstallConfig, cancellable: P, callback: Q, user_data: R);

    //fn generate_for_media_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<String, Error>;

    fn generate_output<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(&self, os: &Os, config: &InstallConfig, output_dir: &P, cancellable: Q) -> Result<gio::File, Error>;

    //fn generate_output_async<'a, 'b, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>, R: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, os: &Os, config: &InstallConfig, output_dir: &P, cancellable: Q, callback: R, user_data: S);

    //fn generate_output_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<gio::File, Error>;

    fn generate_output_for_media<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(&self, media: &Media, config: &InstallConfig, output_dir: &P, cancellable: Q) -> Result<gio::File, Error>;

    //fn generate_output_for_media_async<'a, 'b, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>, R: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, media: &Media, config: &InstallConfig, output_dir: &P, cancellable: Q, callback: R, user_data: S);

    //fn generate_output_for_media_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<gio::File, Error>;

    fn get_avatar_format(&self) -> Option<AvatarFormat>;

    fn get_can_post_install_drivers(&self) -> bool;

    fn get_can_pre_install_drivers(&self) -> bool;

    fn get_config_param(&self, name: &str) -> Option<InstallConfigParam>;

    fn get_config_param_list(&self) -> Vec<InstallScript>;

    fn get_config_params(&self) -> Option<InstallConfigParamList>;

    fn get_expected_filename(&self) -> Option<String>;

    fn get_needs_internet(&self) -> bool;

    fn get_output_filename(&self) -> Option<String>;

    fn get_output_prefix(&self) -> Option<String>;

    fn get_path_format(&self) -> PathFormat;

    fn get_product_key_format(&self) -> Option<String>;

    fn get_profile(&self) -> Option<String>;

    fn get_template_data(&self) -> Option<String>;

    fn get_template_uri(&self) -> Option<String>;

    fn has_config_param(&self, config_param: &InstallConfigParam) -> bool;

    fn has_config_param_name(&self, name: &str) -> bool;

    fn set_output_prefix(&self, prefix: &str);

    fn connect_property_avatar_format_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_path_format_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_product_key_format_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_profile_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_template_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_template_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InstallScript> + IsA<glib::object::Object>> InstallScriptExt for O {
    fn generate<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, os: &Os, config: &InstallConfig, cancellable: P) -> Result<String, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::osinfo_install_script_generate(self.to_glib_none().0, os.to_glib_none().0, config.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn generate_async<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, os: &Os, config: &InstallConfig, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_async() }
    //}

    fn generate_command_line(&self, os: &Os, config: &InstallConfig) -> Option<String> {
        unsafe {
            from_glib_full(ffi::osinfo_install_script_generate_command_line(self.to_glib_none().0, os.to_glib_none().0, config.to_glib_none().0))
        }
    }

    fn generate_command_line_for_media(&self, media: &Media, config: &InstallConfig) -> Option<String> {
        unsafe {
            from_glib_full(ffi::osinfo_install_script_generate_command_line_for_media(self.to_glib_none().0, media.to_glib_none().0, config.to_glib_none().0))
        }
    }

    //fn generate_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<String, Error> {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_finish() }
    //}

    fn generate_for_media<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, media: &Media, config: &InstallConfig, cancellable: P) -> Result<String, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::osinfo_install_script_generate_for_media(self.to_glib_none().0, media.to_glib_none().0, config.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn generate_for_media_async<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, media: &Media, config: &InstallConfig, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_for_media_async() }
    //}

    //fn generate_for_media_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<String, Error> {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_for_media_finish() }
    //}

    fn generate_output<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(&self, os: &Os, config: &InstallConfig, output_dir: &P, cancellable: Q) -> Result<gio::File, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::osinfo_install_script_generate_output(self.to_glib_none().0, os.to_glib_none().0, config.to_glib_none().0, output_dir.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn generate_output_async<'a, 'b, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>, R: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, os: &Os, config: &InstallConfig, output_dir: &P, cancellable: Q, callback: R, user_data: S) {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_output_async() }
    //}

    //fn generate_output_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<gio::File, Error> {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_output_finish() }
    //}

    fn generate_output_for_media<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(&self, media: &Media, config: &InstallConfig, output_dir: &P, cancellable: Q) -> Result<gio::File, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::osinfo_install_script_generate_output_for_media(self.to_glib_none().0, media.to_glib_none().0, config.to_glib_none().0, output_dir.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn generate_output_for_media_async<'a, 'b, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>, R: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, media: &Media, config: &InstallConfig, output_dir: &P, cancellable: Q, callback: R, user_data: S) {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_output_for_media_async() }
    //}

    //fn generate_output_for_media_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<gio::File, Error> {
    //    unsafe { TODO: call ffi::osinfo_install_script_generate_output_for_media_finish() }
    //}

    fn get_avatar_format(&self) -> Option<AvatarFormat> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_avatar_format(self.to_glib_none().0))
        }
    }

    fn get_can_post_install_drivers(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_script_get_can_post_install_drivers(self.to_glib_none().0))
        }
    }

    fn get_can_pre_install_drivers(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_script_get_can_pre_install_drivers(self.to_glib_none().0))
        }
    }

    fn get_config_param(&self, name: &str) -> Option<InstallConfigParam> {
        unsafe {
            from_glib_full(ffi::osinfo_install_script_get_config_param(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_config_param_list(&self) -> Vec<InstallScript> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_install_script_get_config_param_list(self.to_glib_none().0))
        }
    }

    fn get_config_params(&self) -> Option<InstallConfigParamList> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_config_params(self.to_glib_none().0))
        }
    }

    fn get_expected_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_expected_filename(self.to_glib_none().0))
        }
    }

    fn get_needs_internet(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_script_get_needs_internet(self.to_glib_none().0))
        }
    }

    fn get_output_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_output_filename(self.to_glib_none().0))
        }
    }

    fn get_output_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_output_prefix(self.to_glib_none().0))
        }
    }

    fn get_path_format(&self) -> PathFormat {
        unsafe {
            from_glib(ffi::osinfo_install_script_get_path_format(self.to_glib_none().0))
        }
    }

    fn get_product_key_format(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_product_key_format(self.to_glib_none().0))
        }
    }

    fn get_profile(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_profile(self.to_glib_none().0))
        }
    }

    fn get_template_data(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_template_data(self.to_glib_none().0))
        }
    }

    fn get_template_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::osinfo_install_script_get_template_uri(self.to_glib_none().0))
        }
    }

    fn has_config_param(&self, config_param: &InstallConfigParam) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_script_has_config_param(self.to_glib_none().0, config_param.to_glib_none().0))
        }
    }

    fn has_config_param_name(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_script_has_config_param_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn set_output_prefix(&self, prefix: &str) {
        unsafe {
            ffi::osinfo_install_script_set_output_prefix(self.to_glib_none().0, prefix.to_glib_none().0);
        }
    }

    fn connect_property_avatar_format_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::avatar-format",
                transmute(notify_avatar_format_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_path_format_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::path-format",
                transmute(notify_path_format_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_product_key_format_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::product-key-format",
                transmute(notify_product_key_format_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_profile_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::profile",
                transmute(notify_profile_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_template_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::template-data",
                transmute(notify_template_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_template_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::template-uri",
                transmute(notify_template_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_avatar_format_trampoline<P>(this: *mut ffi::OsinfoInstallScript, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InstallScript> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InstallScript::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_path_format_trampoline<P>(this: *mut ffi::OsinfoInstallScript, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InstallScript> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InstallScript::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_product_key_format_trampoline<P>(this: *mut ffi::OsinfoInstallScript, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InstallScript> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InstallScript::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_profile_trampoline<P>(this: *mut ffi::OsinfoInstallScript, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InstallScript> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InstallScript::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_template_data_trampoline<P>(this: *mut ffi::OsinfoInstallScript, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InstallScript> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InstallScript::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_template_uri_trampoline<P>(this: *mut ffi::OsinfoInstallScript, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InstallScript> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InstallScript::from_glib_borrow(this).downcast_unchecked())
}
