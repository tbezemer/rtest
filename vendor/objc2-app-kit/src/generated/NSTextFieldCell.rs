//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextFieldBezelStyle(pub NSUInteger);
impl NSTextFieldBezelStyle {
    pub const NSTextFieldSquareBezel: Self = Self(0);
    pub const NSTextFieldRoundedBezel: Self = Self(1);
}

unsafe impl Encode for NSTextFieldBezelStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextFieldBezelStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    pub struct NSTextFieldCell;

    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl ClassType for NSTextFieldCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSTextFieldCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSTextFieldCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCoding for NSTextFieldCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCopying for NSTextFieldCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSTextFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTextFieldCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSTextFieldCell {
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

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: Option<&NSColor>);

        #[cfg(all(feature = "NSResponder", feature = "NSText", feature = "NSView"))]
        #[method_id(@__retain_semantics Other setUpFieldEditorAttributes:)]
        pub unsafe fn setUpFieldEditorAttributes(&self, text_obj: &NSText) -> Retained<NSText>;

        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSTextFieldBezelStyle;

        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSTextFieldBezelStyle);

        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Retained<NSString>>;

        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[method(setWantsNotificationForMarkedText:)]
        pub unsafe fn setWantsNotificationForMarkedText(&self, flag: bool);

        #[method_id(@__retain_semantics Other allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowed_input_source_locales: Option<&NSArray<NSString>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSTextFieldCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSTextFieldCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);