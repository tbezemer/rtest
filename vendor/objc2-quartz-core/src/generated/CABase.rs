//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[inline]
pub extern "C" fn CACurrentMediaTime() -> CFTimeInterval {
    extern "C" {
        fn CACurrentMediaTime() -> CFTimeInterval;
    }
    unsafe { CACurrentMediaTime() }
}