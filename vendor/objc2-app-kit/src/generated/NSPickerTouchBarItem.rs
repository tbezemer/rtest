//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPickerTouchBarItemSelectionMode(pub NSInteger);
impl NSPickerTouchBarItemSelectionMode {
    #[doc(alias = "NSPickerTouchBarItemSelectionModeSelectOne")]
    pub const SelectOne: Self = Self(0);
    #[doc(alias = "NSPickerTouchBarItemSelectionModeSelectAny")]
    pub const SelectAny: Self = Self(1);
    #[doc(alias = "NSPickerTouchBarItemSelectionModeMomentary")]
    pub const Momentary: Self = Self(2);
}

unsafe impl Encode for NSPickerTouchBarItemSelectionMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPickerTouchBarItemSelectionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPickerTouchBarItemControlRepresentation(pub NSInteger);
impl NSPickerTouchBarItemControlRepresentation {
    #[doc(alias = "NSPickerTouchBarItemControlRepresentationAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSPickerTouchBarItemControlRepresentationExpanded")]
    pub const Expanded: Self = Self(1);
    #[doc(alias = "NSPickerTouchBarItemControlRepresentationCollapsed")]
    pub const Collapsed: Self = Self(2);
}

unsafe impl Encode for NSPickerTouchBarItemControlRepresentation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPickerTouchBarItemControlRepresentation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSTouchBarItem")]
    pub struct NSPickerTouchBarItem;

    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl ClassType for NSPickerTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSCoding for NSPickerTouchBarItem {}

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSPickerTouchBarItem {}

extern_methods!(
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[method_id(@__retain_semantics Other pickerTouchBarItemWithIdentifier:labels:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_labels_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            labels: &NSArray<NSString>,
            selection_mode: NSPickerTouchBarItemSelectionMode,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other pickerTouchBarItemWithIdentifier:images:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_images_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            images: &NSArray<NSImage>,
            selection_mode: NSPickerTouchBarItemSelectionMode,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method(controlRepresentation)]
        pub unsafe fn controlRepresentation(&self) -> NSPickerTouchBarItemControlRepresentation;

        #[method(setControlRepresentation:)]
        pub unsafe fn setControlRepresentation(
            &self,
            control_representation: NSPickerTouchBarItemControlRepresentation,
        );

        #[method_id(@__retain_semantics Other collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Retained<NSString>;

        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsed_representation_label: &NSString,
        );

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsed_representation_image: Option<&NSImage>,
        );

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other selectionColor)]
        pub unsafe fn selectionColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setSelectionColor:)]
        pub unsafe fn setSelectionColor(&self, selection_color: Option<&NSColor>);

        #[method(selectionMode)]
        pub unsafe fn selectionMode(&self) -> NSPickerTouchBarItemSelectionMode;

        #[method(setSelectionMode:)]
        pub unsafe fn setSelectionMode(&self, selection_mode: NSPickerTouchBarItemSelectionMode);

        #[method(numberOfOptions)]
        pub unsafe fn numberOfOptions(&self) -> NSInteger;

        #[method(setNumberOfOptions:)]
        pub unsafe fn setNumberOfOptions(&self, number_of_options: NSInteger);

        #[cfg(feature = "NSImage")]
        #[method(setImage:atIndex:)]
        pub unsafe fn setImage_atIndex(&self, image: Option<&NSImage>, index: NSInteger);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other imageAtIndex:)]
        pub unsafe fn imageAtIndex(&self, index: NSInteger) -> Option<Retained<NSImage>>;

        #[method(setLabel:atIndex:)]
        pub unsafe fn setLabel_atIndex(&self, label: &NSString, index: NSInteger);

        #[method_id(@__retain_semantics Other labelAtIndex:)]
        pub unsafe fn labelAtIndex(&self, index: NSInteger) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(setEnabled:atIndex:)]
        pub unsafe fn setEnabled_atIndex(&self, enabled: bool, index: NSInteger);

        #[method(isEnabledAtIndex:)]
        pub unsafe fn isEnabledAtIndex(&self, index: NSInteger) -> bool;

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Retained<NSString>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);