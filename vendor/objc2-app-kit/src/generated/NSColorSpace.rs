//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorSpaceModel(pub NSInteger);
impl NSColorSpaceModel {
    #[doc(alias = "NSColorSpaceModelUnknown")]
    pub const Unknown: Self = Self(-1);
    #[doc(alias = "NSColorSpaceModelGray")]
    pub const Gray: Self = Self(0);
    #[doc(alias = "NSColorSpaceModelRGB")]
    pub const RGB: Self = Self(1);
    #[doc(alias = "NSColorSpaceModelCMYK")]
    pub const CMYK: Self = Self(2);
    #[doc(alias = "NSColorSpaceModelLAB")]
    pub const LAB: Self = Self(3);
    #[doc(alias = "NSColorSpaceModelDeviceN")]
    pub const DeviceN: Self = Self(4);
    #[doc(alias = "NSColorSpaceModelIndexed")]
    pub const Indexed: Self = Self(5);
    #[doc(alias = "NSColorSpaceModelPatterned")]
    pub const Patterned: Self = Self(6);
}

unsafe impl Encode for NSColorSpaceModel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorSpaceModel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorSpace;

    unsafe impl ClassType for NSColorSpace {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSColorSpace {}

unsafe impl Sync for NSColorSpace {}

unsafe impl NSCoding for NSColorSpace {}

unsafe impl NSObjectProtocol for NSColorSpace {}

unsafe impl NSSecureCoding for NSColorSpace {}

extern_methods!(
    unsafe impl NSColorSpace {
        #[method_id(@__retain_semantics Init initWithICCProfileData:)]
        pub unsafe fn initWithICCProfileData(
            this: Allocated<Self>,
            icc_data: &NSData,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other ICCProfileData)]
        pub unsafe fn ICCProfileData(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Init initWithColorSyncProfile:)]
        pub unsafe fn initWithColorSyncProfile(
            this: Allocated<Self>,
            prof: NonNull<c_void>,
        ) -> Option<Retained<Self>>;

        #[method(colorSyncProfile)]
        pub unsafe fn colorSyncProfile(&self) -> *mut c_void;

        #[method(numberOfColorComponents)]
        pub unsafe fn numberOfColorComponents(&self) -> NSInteger;

        #[method(colorSpaceModel)]
        pub unsafe fn colorSpaceModel(&self) -> NSColorSpaceModel;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other sRGBColorSpace)]
        pub unsafe fn sRGBColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericGamma22GrayColorSpace)]
        pub unsafe fn genericGamma22GrayColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other extendedSRGBColorSpace)]
        pub unsafe fn extendedSRGBColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other extendedGenericGamma22GrayColorSpace)]
        pub unsafe fn extendedGenericGamma22GrayColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other displayP3ColorSpace)]
        pub unsafe fn displayP3ColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other adobeRGB1998ColorSpace)]
        pub unsafe fn adobeRGB1998ColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericRGBColorSpace)]
        pub unsafe fn genericRGBColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericGrayColorSpace)]
        pub unsafe fn genericGrayColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericCMYKColorSpace)]
        pub unsafe fn genericCMYKColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other deviceRGBColorSpace)]
        pub unsafe fn deviceRGBColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other deviceGrayColorSpace)]
        pub unsafe fn deviceGrayColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other deviceCMYKColorSpace)]
        pub unsafe fn deviceCMYKColorSpace() -> Retained<NSColorSpace>;

        #[method_id(@__retain_semantics Other availableColorSpacesWithModel:)]
        pub unsafe fn availableColorSpacesWithModel(
            model: NSColorSpaceModel,
        ) -> Retained<NSArray<NSColorSpace>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSColorSpace {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static NSUnknownColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::Unknown.0);

pub static NSGrayColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::Gray.0);

pub static NSRGBColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::RGB.0);

pub static NSCMYKColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::CMYK.0);

pub static NSLABColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::LAB.0);

pub static NSDeviceNColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::DeviceN.0);

pub static NSIndexedColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::Indexed.0);

pub static NSPatternColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::Patterned.0);