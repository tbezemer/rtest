//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSHapticFeedbackPattern(pub NSInteger);
impl NSHapticFeedbackPattern {
    #[doc(alias = "NSHapticFeedbackPatternGeneric")]
    pub const Generic: Self = Self(0);
    #[doc(alias = "NSHapticFeedbackPatternAlignment")]
    pub const Alignment: Self = Self(1);
    #[doc(alias = "NSHapticFeedbackPatternLevelChange")]
    pub const LevelChange: Self = Self(2);
}

unsafe impl Encode for NSHapticFeedbackPattern {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSHapticFeedbackPattern {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSHapticFeedbackPerformanceTime(pub NSUInteger);
impl NSHapticFeedbackPerformanceTime {
    #[doc(alias = "NSHapticFeedbackPerformanceTimeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSHapticFeedbackPerformanceTimeNow")]
    pub const Now: Self = Self(1);
    #[doc(alias = "NSHapticFeedbackPerformanceTimeDrawCompleted")]
    pub const DrawCompleted: Self = Self(2);
}

unsafe impl Encode for NSHapticFeedbackPerformanceTime {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSHapticFeedbackPerformanceTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSHapticFeedbackPerformer: NSObjectProtocol {
        #[method(performFeedbackPattern:performanceTime:)]
        unsafe fn performFeedbackPattern_performanceTime(
            &self,
            pattern: NSHapticFeedbackPattern,
            performance_time: NSHapticFeedbackPerformanceTime,
        );
    }

    unsafe impl ProtocolType for dyn NSHapticFeedbackPerformer {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHapticFeedbackManager;

    unsafe impl ClassType for NSHapticFeedbackManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSHapticFeedbackManager {}

extern_methods!(
    unsafe impl NSHapticFeedbackManager {
        #[method_id(@__retain_semantics Other defaultPerformer)]
        pub unsafe fn defaultPerformer() -> Retained<ProtocolObject<dyn NSHapticFeedbackPerformer>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHapticFeedbackManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);