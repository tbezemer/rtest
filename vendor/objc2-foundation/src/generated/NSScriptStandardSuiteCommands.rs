//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSaveOptions(pub NSUInteger);
impl NSSaveOptions {
    #[doc(alias = "NSSaveOptionsYes")]
    pub const Yes: Self = Self(0);
    #[doc(alias = "NSSaveOptionsNo")]
    pub const No: Self = Self(1);
    #[doc(alias = "NSSaveOptionsAsk")]
    pub const Ask: Self = Self(2);
}

unsafe impl Encode for NSSaveOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSSaveOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSCloneCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSCloneCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSCloneCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCloneCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCloneCommand {
        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Retained<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCloneCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCloneCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSCloseCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSCloseCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSCloseCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCloseCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCloseCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCloseCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCloseCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSCountCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSCountCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSCountCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCountCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCountCommand {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCountCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCountCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSCreateCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSCreateCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSCreateCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCreateCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCreateCommand {
        #[cfg(all(feature = "NSClassDescription", feature = "NSScriptClassDescription"))]
        #[method_id(@__retain_semantics Other createClassDescription)]
        pub unsafe fn createClassDescription(&self) -> Retained<NSScriptClassDescription>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other resolvedKeyDictionary)]
        pub unsafe fn resolvedKeyDictionary(&self) -> Retained<NSDictionary<NSString, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCreateCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSCreateCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSDeleteCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSDeleteCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSDeleteCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSDeleteCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSDeleteCommand {
        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Retained<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSDeleteCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSDeleteCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSExistsCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSExistsCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSExistsCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSExistsCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSExistsCommand {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSExistsCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSExistsCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSGetCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSGetCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSGetCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSGetCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSGetCommand {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSGetCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSGetCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSMoveCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSMoveCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSMoveCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSMoveCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSMoveCommand {
        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Retained<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSMoveCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSMoveCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSQuitCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSQuitCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSQuitCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSQuitCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSQuitCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSQuitCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSQuitCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSScriptCommand")]
    pub struct NSSetCommand;

    #[cfg(feature = "NSScriptCommand")]
    unsafe impl ClassType for NSSetCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSScriptCommand"))]
unsafe impl NSCoding for NSSetCommand {}

#[cfg(feature = "NSScriptCommand")]
unsafe impl NSObjectProtocol for NSSetCommand {}

extern_methods!(
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSSetCommand {
        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Retained<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSSetCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSScriptCommand")]
    unsafe impl NSSetCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);