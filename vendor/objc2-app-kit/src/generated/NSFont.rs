//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NSFontIdentityMatrix: NonNull<CGFloat>;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFont;

    unsafe impl ClassType for NSFont {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSFont {}

unsafe impl NSCopying for NSFont {}

unsafe impl NSObjectProtocol for NSFont {}

unsafe impl NSSecureCoding for NSFont {}

extern_methods!(
    unsafe impl NSFont {
        #[method_id(@__retain_semantics Other fontWithName:size:)]
        pub unsafe fn fontWithName_size(
            font_name: &NSString,
            font_size: CGFloat,
        ) -> Option<Retained<NSFont>>;

        #[method_id(@__retain_semantics Other fontWithName:matrix:)]
        pub unsafe fn fontWithName_matrix(
            font_name: &NSString,
            font_matrix: NonNull<CGFloat>,
        ) -> Option<Retained<NSFont>>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other fontWithDescriptor:size:)]
        pub unsafe fn fontWithDescriptor_size(
            font_descriptor: &NSFontDescriptor,
            font_size: CGFloat,
        ) -> Option<Retained<NSFont>>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other fontWithDescriptor:textTransform:)]
        pub unsafe fn fontWithDescriptor_textTransform(
            font_descriptor: &NSFontDescriptor,
            text_transform: Option<&NSAffineTransform>,
        ) -> Option<Retained<NSFont>>;

        #[method_id(@__retain_semantics Other userFontOfSize:)]
        pub unsafe fn userFontOfSize(font_size: CGFloat) -> Option<Retained<NSFont>>;

        #[method_id(@__retain_semantics Other userFixedPitchFontOfSize:)]
        pub unsafe fn userFixedPitchFontOfSize(font_size: CGFloat) -> Option<Retained<NSFont>>;

        #[method(setUserFont:)]
        pub unsafe fn setUserFont(font: Option<&NSFont>);

        #[method(setUserFixedPitchFont:)]
        pub unsafe fn setUserFixedPitchFont(font: Option<&NSFont>);

        #[method_id(@__retain_semantics Other systemFontOfSize:)]
        pub unsafe fn systemFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other boldSystemFontOfSize:)]
        pub unsafe fn boldSystemFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other labelFontOfSize:)]
        pub unsafe fn labelFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other titleBarFontOfSize:)]
        pub unsafe fn titleBarFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other menuFontOfSize:)]
        pub unsafe fn menuFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other menuBarFontOfSize:)]
        pub unsafe fn menuBarFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other messageFontOfSize:)]
        pub unsafe fn messageFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other paletteFontOfSize:)]
        pub unsafe fn paletteFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other toolTipsFontOfSize:)]
        pub unsafe fn toolTipsFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other controlContentFontOfSize:)]
        pub unsafe fn controlContentFontOfSize(font_size: CGFloat) -> Retained<NSFont>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other systemFontOfSize:weight:)]
        pub unsafe fn systemFontOfSize_weight(
            font_size: CGFloat,
            weight: NSFontWeight,
        ) -> Retained<NSFont>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other monospacedDigitSystemFontOfSize:weight:)]
        pub unsafe fn monospacedDigitSystemFontOfSize_weight(
            font_size: CGFloat,
            weight: NSFontWeight,
        ) -> Retained<NSFont>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other systemFontOfSize:weight:width:)]
        pub unsafe fn systemFontOfSize_weight_width(
            font_size: CGFloat,
            weight: NSFontWeight,
            width: NSFontWidth,
        ) -> Retained<NSFont>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other monospacedSystemFontOfSize:weight:)]
        pub unsafe fn monospacedSystemFontOfSize_weight(
            font_size: CGFloat,
            weight: NSFontWeight,
        ) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other fontWithSize:)]
        pub unsafe fn fontWithSize(&self, font_size: CGFloat) -> Retained<NSFont>;

        #[method(systemFontSize)]
        pub unsafe fn systemFontSize() -> CGFloat;

        #[method(smallSystemFontSize)]
        pub unsafe fn smallSystemFontSize() -> CGFloat;

        #[method(labelFontSize)]
        pub unsafe fn labelFontSize() -> CGFloat;

        #[cfg(feature = "NSCell")]
        #[method(systemFontSizeForControlSize:)]
        pub unsafe fn systemFontSizeForControlSize(control_size: NSControlSize) -> CGFloat;

        #[method_id(@__retain_semantics Other fontName)]
        pub unsafe fn fontName(&self) -> Retained<NSString>;

        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[method(matrix)]
        pub unsafe fn matrix(&self) -> NonNull<CGFloat>;

        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other fontDescriptor)]
        pub unsafe fn fontDescriptor(&self) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other textTransform)]
        pub unsafe fn textTransform(&self) -> Retained<NSAffineTransform>;

        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;

        #[method(mostCompatibleStringEncoding)]
        pub unsafe fn mostCompatibleStringEncoding(&self) -> NSStringEncoding;

        #[method_id(@__retain_semantics Other coveredCharacterSet)]
        pub unsafe fn coveredCharacterSet(&self) -> Retained<NSCharacterSet>;

        #[method(boundingRectForFont)]
        pub unsafe fn boundingRectForFont(&self) -> NSRect;

        #[method(maximumAdvancement)]
        pub unsafe fn maximumAdvancement(&self) -> NSSize;

        #[method(ascender)]
        pub unsafe fn ascender(&self) -> CGFloat;

        #[method(descender)]
        pub unsafe fn descender(&self) -> CGFloat;

        #[method(leading)]
        pub unsafe fn leading(&self) -> CGFloat;

        #[method(underlinePosition)]
        pub unsafe fn underlinePosition(&self) -> CGFloat;

        #[method(underlineThickness)]
        pub unsafe fn underlineThickness(&self) -> CGFloat;

        #[method(italicAngle)]
        pub unsafe fn italicAngle(&self) -> CGFloat;

        #[method(capHeight)]
        pub unsafe fn capHeight(&self) -> CGFloat;

        #[method(xHeight)]
        pub unsafe fn xHeight(&self) -> CGFloat;

        #[method(isFixedPitch)]
        pub unsafe fn isFixedPitch(&self) -> bool;

        #[method(set)]
        pub unsafe fn set(&self);

        #[cfg(feature = "NSGraphicsContext")]
        #[method(setInContext:)]
        pub unsafe fn setInContext(&self, graphics_context: &NSGraphicsContext);

        #[method_id(@__retain_semantics Other verticalFont)]
        pub unsafe fn verticalFont(&self) -> Retained<NSFont>;

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFont {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NSAntialiasThresholdChangedNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSFontSetChangedNotification: &'static NSNotificationName;
}

pub type NSGlyph = c_uint;

pub const NSControlGlyph: c_uint = 0x00FFFFFF;
pub const NSNullGlyph: c_uint = 0x0;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontRenderingMode(pub NSUInteger);
impl NSFontRenderingMode {
    pub const NSFontDefaultRenderingMode: Self = Self(0);
    pub const NSFontAntialiasedRenderingMode: Self = Self(1);
    pub const NSFontIntegerAdvancementsRenderingMode: Self = Self(2);
    pub const NSFontAntialiasedIntegerAdvancementsRenderingMode: Self = Self(3);
}

unsafe impl Encode for NSFontRenderingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFontRenderingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMultibyteGlyphPacking(pub NSUInteger);
impl NSMultibyteGlyphPacking {
    #[deprecated]
    pub const NSNativeShortGlyphPacking: Self = Self(5);
}

unsafe impl Encode for NSMultibyteGlyphPacking {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSMultibyteGlyphPacking {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[deprecated]
    pub fn NSConvertGlyphsToPackedGlyphs(
        gl_buf: NonNull<NSGlyph>,
        count: NSInteger,
        packing: NSMultibyteGlyphPacking,
        packed_glyphs: NonNull<c_char>,
    ) -> NSInteger;
}

extern_methods!(
    /// NSFont_Deprecated
    unsafe impl NSFont {
        #[method(glyphWithName:)]
        pub unsafe fn glyphWithName(&self, name: &NSString) -> NSGlyph;

        #[method(boundingRectForGlyph:)]
        pub unsafe fn boundingRectForGlyph(&self, glyph: NSGlyph) -> NSRect;

        #[method(advancementForGlyph:)]
        pub unsafe fn advancementForGlyph(&self, glyph: NSGlyph) -> NSSize;

        #[method(getBoundingRects:forGlyphs:count:)]
        pub unsafe fn getBoundingRects_forGlyphs_count(
            &self,
            bounds: NSRectArray,
            glyphs: NonNull<NSGlyph>,
            glyph_count: NSUInteger,
        );

        #[method(getAdvancements:forGlyphs:count:)]
        pub unsafe fn getAdvancements_forGlyphs_count(
            &self,
            advancements: NSSizeArray,
            glyphs: NonNull<NSGlyph>,
            glyph_count: NSUInteger,
        );

        #[method(getAdvancements:forPackedGlyphs:length:)]
        pub unsafe fn getAdvancements_forPackedGlyphs_length(
            &self,
            advancements: NSSizeArray,
            packed_glyphs: NonNull<c_void>,
            length: NSUInteger,
        );

        #[method_id(@__retain_semantics Other printerFont)]
        pub unsafe fn printerFont(&self) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other screenFont)]
        pub unsafe fn screenFont(&self) -> Retained<NSFont>;

        #[method_id(@__retain_semantics Other screenFontWithRenderingMode:)]
        pub unsafe fn screenFontWithRenderingMode(
            &self,
            rendering_mode: NSFontRenderingMode,
        ) -> Retained<NSFont>;

        #[method(renderingMode)]
        pub unsafe fn renderingMode(&self) -> NSFontRenderingMode;
    }
);

extern_methods!(
    /// NSFont_TextStyles
    unsafe impl NSFont {
        #[cfg(feature = "NSFontDescriptor")]
        #[method_id(@__retain_semantics Other preferredFontForTextStyle:options:)]
        pub unsafe fn preferredFontForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, AnyObject>,
        ) -> Retained<NSFont>;
    }
);