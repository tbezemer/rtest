//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCompoundPredicateType(pub NSUInteger);
impl NSCompoundPredicateType {
    pub const NSNotPredicateType: Self = Self(0);
    pub const NSAndPredicateType: Self = Self(1);
    pub const NSOrPredicateType: Self = Self(2);
}

unsafe impl Encode for NSCompoundPredicateType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCompoundPredicateType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPredicate")]
    pub struct NSCompoundPredicate;

    #[cfg(feature = "NSPredicate")]
    unsafe impl ClassType for NSCompoundPredicate {
        #[inherits(NSObject)]
        type Super = NSPredicate;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl NSCoding for NSCompoundPredicate {}

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl NSCopying for NSCompoundPredicate {}

#[cfg(feature = "NSPredicate")]
unsafe impl NSObjectProtocol for NSCompoundPredicate {}

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl NSSecureCoding for NSCompoundPredicate {}

extern_methods!(
    #[cfg(feature = "NSPredicate")]
    unsafe impl NSCompoundPredicate {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithType:subpredicates:)]
        pub unsafe fn initWithType_subpredicates(
            this: Allocated<Self>,
            r#type: NSCompoundPredicateType,
            subpredicates: &NSArray<NSPredicate>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(compoundPredicateType)]
        pub unsafe fn compoundPredicateType(&self) -> NSCompoundPredicateType;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other subpredicates)]
        pub unsafe fn subpredicates(&self) -> Retained<NSArray>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other andPredicateWithSubpredicates:)]
        pub unsafe fn andPredicateWithSubpredicates(
            subpredicates: &NSArray<NSPredicate>,
        ) -> Retained<NSCompoundPredicate>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other orPredicateWithSubpredicates:)]
        pub unsafe fn orPredicateWithSubpredicates(
            subpredicates: &NSArray<NSPredicate>,
        ) -> Retained<NSCompoundPredicate>;

        #[method_id(@__retain_semantics Other notPredicateWithSubpredicate:)]
        pub unsafe fn notPredicateWithSubpredicate(
            predicate: &NSPredicate,
        ) -> Retained<NSCompoundPredicate>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPredicate")]
    unsafe impl NSCompoundPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);