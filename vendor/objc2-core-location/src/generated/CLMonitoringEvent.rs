//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLMonitoringState(pub NSUInteger);
impl CLMonitoringState {
    #[doc(alias = "CLMonitoringStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CLMonitoringStateSatisfied")]
    pub const Satisfied: Self = Self(1);
    #[doc(alias = "CLMonitoringStateUnsatisfied")]
    pub const Unsatisfied: Self = Self(2);
    #[doc(alias = "CLMonitoringStateUnmonitored")]
    pub const Unmonitored: Self = Self(3);
}

unsafe impl Encode for CLMonitoringState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CLMonitoringState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLMonitoringEvent;

    unsafe impl ClassType for CLMonitoringEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CLMonitoringEvent {}

unsafe impl NSObjectProtocol for CLMonitoringEvent {}

unsafe impl NSSecureCoding for CLMonitoringEvent {}

extern_methods!(
    unsafe impl CLMonitoringEvent {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "CLCondition")]
        #[method_id(@__retain_semantics Other refinement)]
        pub unsafe fn refinement(&self) -> Option<Retained<CLCondition>>;

        #[method(state)]
        pub unsafe fn state(&self) -> CLMonitoringState;

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);