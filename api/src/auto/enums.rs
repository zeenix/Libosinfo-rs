// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DeviceDriverSigningReq {
    None,
    Strict,
    Warn,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DeviceDriverSigningReq {
    type GlibType = ffi::OsinfoDeviceDriverSigningReq;

    fn to_glib(&self) -> ffi::OsinfoDeviceDriverSigningReq {
        match *self {
            DeviceDriverSigningReq::None => ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_NONE,
            DeviceDriverSigningReq::Strict => ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_STRICT,
            DeviceDriverSigningReq::Warn => ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_WARN,
            DeviceDriverSigningReq::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoDeviceDriverSigningReq> for DeviceDriverSigningReq {
    fn from_glib(value: ffi::OsinfoDeviceDriverSigningReq) -> Self {
        match value {
            0 => DeviceDriverSigningReq::None,
            1 => DeviceDriverSigningReq::Strict,
            2 => DeviceDriverSigningReq::Warn,
            value => DeviceDriverSigningReq::__Unknown(value),
        }
    }
}

impl StaticType for DeviceDriverSigningReq {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_device_driver_signing_req_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DeviceDriverSigningReq {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DeviceDriverSigningReq {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DeviceDriverSigningReq {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum InstallConfigParamPolicy {
    None,
    Required,
    Optional,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for InstallConfigParamPolicy {
    type GlibType = ffi::OsinfoInstallConfigParamPolicy;

    fn to_glib(&self) -> ffi::OsinfoInstallConfigParamPolicy {
        match *self {
            InstallConfigParamPolicy::None => ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_NONE,
            InstallConfigParamPolicy::Required => ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_REQUIRED,
            InstallConfigParamPolicy::Optional => ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_OPTIONAL,
            InstallConfigParamPolicy::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoInstallConfigParamPolicy> for InstallConfigParamPolicy {
    fn from_glib(value: ffi::OsinfoInstallConfigParamPolicy) -> Self {
        match value {
            0 => InstallConfigParamPolicy::None,
            1 => InstallConfigParamPolicy::Required,
            2 => InstallConfigParamPolicy::Optional,
            value => InstallConfigParamPolicy::__Unknown(value),
        }
    }
}

impl StaticType for InstallConfigParamPolicy {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_install_config_param_policy_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InstallConfigParamPolicy {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InstallConfigParamPolicy {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for InstallConfigParamPolicy {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MediaError {
    NoDescriptors,
    NoPvd,
    NoSvd,
    InsufficientMetadata,
    NotBootable,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MediaError {
    type GlibType = ffi::OsinfoMediaError;

    fn to_glib(&self) -> ffi::OsinfoMediaError {
        match *self {
            MediaError::NoDescriptors => ffi::OSINFO_MEDIA_ERROR_NO_DESCRIPTORS,
            MediaError::NoPvd => ffi::OSINFO_MEDIA_ERROR_NO_PVD,
            MediaError::NoSvd => ffi::OSINFO_MEDIA_ERROR_NO_SVD,
            MediaError::InsufficientMetadata => ffi::OSINFO_MEDIA_ERROR_INSUFFICIENT_METADATA,
            MediaError::NotBootable => ffi::OSINFO_MEDIA_ERROR_NOT_BOOTABLE,
            MediaError::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoMediaError> for MediaError {
    fn from_glib(value: ffi::OsinfoMediaError) -> Self {
        match value {
            0 => MediaError::NoDescriptors,
            1 => MediaError::NoPvd,
            2 => MediaError::NoSvd,
            3 => MediaError::InsufficientMetadata,
            4 => MediaError::NotBootable,
            value => MediaError::__Unknown(value),
        }
    }
}

impl ErrorDomain for MediaError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::osinfo_media_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(MediaError::NoDescriptors),
            1 => Some(MediaError::NoPvd),
            2 => Some(MediaError::NoSvd),
            3 => Some(MediaError::InsufficientMetadata),
            4 => Some(MediaError::NotBootable),
            value => Some(MediaError::__Unknown(value)),
        }
    }
}

impl StaticType for MediaError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_media_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MediaError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MediaError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for MediaError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PathFormat {
    Unix,
    Dos,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PathFormat {
    type GlibType = ffi::OsinfoPathFormat;

    fn to_glib(&self) -> ffi::OsinfoPathFormat {
        match *self {
            PathFormat::Unix => ffi::OSINFO_PATH_FORMAT_UNIX,
            PathFormat::Dos => ffi::OSINFO_PATH_FORMAT_DOS,
            PathFormat::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoPathFormat> for PathFormat {
    fn from_glib(value: ffi::OsinfoPathFormat) -> Self {
        match value {
            0 => PathFormat::Unix,
            1 => PathFormat::Dos,
            value => PathFormat::__Unknown(value),
        }
    }
}

impl StaticType for PathFormat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_path_format_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PathFormat {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PathFormat {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PathFormat {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProductRelationship {
    DerivesFrom,
    Upgrades,
    Clones,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ProductRelationship {
    type GlibType = ffi::OsinfoProductRelationship;

    fn to_glib(&self) -> ffi::OsinfoProductRelationship {
        match *self {
            ProductRelationship::DerivesFrom => ffi::OSINFO_PRODUCT_RELATIONSHIP_DERIVES_FROM,
            ProductRelationship::Upgrades => ffi::OSINFO_PRODUCT_RELATIONSHIP_UPGRADES,
            ProductRelationship::Clones => ffi::OSINFO_PRODUCT_RELATIONSHIP_CLONES,
            ProductRelationship::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoProductRelationship> for ProductRelationship {
    fn from_glib(value: ffi::OsinfoProductRelationship) -> Self {
        match value {
            0 => ProductRelationship::DerivesFrom,
            1 => ProductRelationship::Upgrades,
            2 => ProductRelationship::Clones,
            value => ProductRelationship::__Unknown(value),
        }
    }
}

impl StaticType for ProductRelationship {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_product_relationship_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ProductRelationship {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ProductRelationship {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ProductRelationship {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ReleaseStatus {
    Released,
    Snapshot,
    Prerelease,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ReleaseStatus {
    type GlibType = ffi::OsinfoReleaseStatus;

    fn to_glib(&self) -> ffi::OsinfoReleaseStatus {
        match *self {
            ReleaseStatus::Released => ffi::OSINFO_RELEASE_STATUS_RELEASED,
            ReleaseStatus::Snapshot => ffi::OSINFO_RELEASE_STATUS_SNAPSHOT,
            ReleaseStatus::Prerelease => ffi::OSINFO_RELEASE_STATUS_PRERELEASE,
            ReleaseStatus::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoReleaseStatus> for ReleaseStatus {
    fn from_glib(value: ffi::OsinfoReleaseStatus) -> Self {
        match value {
            0 => ReleaseStatus::Released,
            1 => ReleaseStatus::Snapshot,
            2 => ReleaseStatus::Prerelease,
            value => ReleaseStatus::__Unknown(value),
        }
    }
}

impl StaticType for ReleaseStatus {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_release_status_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ReleaseStatus {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ReleaseStatus {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ReleaseStatus {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

