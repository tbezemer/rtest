//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    pub struct NSSecureTextField;

    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl ClassType for NSSecureTextField {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTextField;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSSecureTextField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSSecureTextField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityNavigableStaticText for NSSecureTextField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityStaticText for NSSecureTextField {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSSecureTextField {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSSecureTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSCoding for NSSecureTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSSecureTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSObjectProtocol for NSSecureTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextContent",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSTextContent for NSSecureTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSecureTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSSecureTextField {}

extern_methods!(
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSecureTextField {}
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSecureTextField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSecureTextField {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSecureTextField {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    pub struct NSSecureTextFieldCell;

    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl ClassType for NSSecureTextFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibility for NSSecureTextFieldCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSSecureTextFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCoding for NSSecureTextFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCopying for NSSecureTextFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSObjectProtocol for NSSecureTextFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSecureTextFieldCell {}

extern_methods!(
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSSecureTextFieldCell {
        #[method(echosBullets)]
        pub unsafe fn echosBullets(&self) -> bool;

        #[method(setEchosBullets:)]
        pub unsafe fn setEchosBullets(&self, echos_bullets: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSSecureTextFieldCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSSecureTextFieldCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSSecureTextFieldCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);