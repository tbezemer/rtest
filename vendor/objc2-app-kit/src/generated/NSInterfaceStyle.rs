//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const NSNoInterfaceStyle: c_uint = 0;
#[deprecated]
pub const NSNextStepInterfaceStyle: c_uint = 1;
#[deprecated]
pub const NSWindows95InterfaceStyle: c_uint = 2;
#[deprecated]
pub const NSMacintoshInterfaceStyle: c_uint = 3;

pub type NSInterfaceStyle = NSUInteger;

extern "C" {
    #[cfg(feature = "NSResponder")]
    #[deprecated]
    pub fn NSInterfaceStyleForKey(
        key: Option<&NSString>,
        responder: Option<&NSResponder>,
    ) -> NSInterfaceStyle;
}

extern_methods!(
    /// NSInterfaceStyle
    #[cfg(feature = "NSResponder")]
    unsafe impl NSResponder {
        #[deprecated]
        #[method(interfaceStyle)]
        pub unsafe fn interfaceStyle(&self) -> NSInterfaceStyle;

        #[deprecated]
        #[method(setInterfaceStyle:)]
        pub unsafe fn setInterfaceStyle(&self, interface_style: NSInterfaceStyle);
    }
);

extern "C" {
    pub static NSInterfaceStyleDefault: Option<&'static NSString>;
}