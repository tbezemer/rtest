//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type CAScrollLayerScrollMode = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAScrollLayer;

    #[cfg(feature = "CALayer")]
    unsafe impl ClassType for CAScrollLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CAScrollLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CAScrollLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CAScrollLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CAScrollLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CAScrollLayer {
        #[method(scrollToPoint:)]
        pub unsafe fn scrollToPoint(&self, p: CGPoint);

        #[method(scrollToRect:)]
        pub unsafe fn scrollToRect(&self, r: CGRect);

        #[method_id(@__retain_semantics Other scrollMode)]
        pub unsafe fn scrollMode(&self) -> Retained<CAScrollLayerScrollMode>;

        #[method(setScrollMode:)]
        pub unsafe fn setScrollMode(&self, scroll_mode: &CAScrollLayerScrollMode);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CAScrollLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CALayer")]
    unsafe impl CAScrollLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// CALayerScrolling
    #[cfg(feature = "CALayer")]
    unsafe impl CALayer {
        #[method(scrollPoint:)]
        pub unsafe fn scrollPoint(&self, p: CGPoint);

        #[method(scrollRectToVisible:)]
        pub unsafe fn scrollRectToVisible(&self, r: CGRect);

        #[method(visibleRect)]
        pub unsafe fn visibleRect(&self) -> CGRect;
    }
);

extern "C" {
    pub static kCAScrollNone: &'static CAScrollLayerScrollMode;
}

extern "C" {
    pub static kCAScrollVertically: &'static CAScrollLayerScrollMode;
}

extern "C" {
    pub static kCAScrollHorizontally: &'static CAScrollLayerScrollMode;
}

extern "C" {
    pub static kCAScrollBoth: &'static CAScrollLayerScrollMode;
}