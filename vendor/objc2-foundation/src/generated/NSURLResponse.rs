//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLResponse;

    unsafe impl ClassType for NSURLResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSURLResponse {}

unsafe impl Sync for NSURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSURLResponse {}

unsafe impl NSObjectProtocol for NSURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSURLResponse {}

extern_methods!(
    unsafe impl NSURLResponse {
        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:MIMEType:expectedContentLength:textEncodingName:)]
        pub unsafe fn initWithURL_MIMEType_expectedContentLength_textEncodingName(
            this: Allocated<Self>,
            url: &NSURL,
            mime_type: Option<&NSString>,
            length: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other MIMEType)]
        pub unsafe fn MIMEType(&self) -> Option<Retained<NSString>>;

        #[method(expectedContentLength)]
        pub unsafe fn expectedContentLength(&self) -> c_longlong;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other textEncodingName)]
        pub unsafe fn textEncodingName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other suggestedFilename)]
        pub unsafe fn suggestedFilename(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHTTPURLResponse;

    unsafe impl ClassType for NSHTTPURLResponse {
        #[inherits(NSObject)]
        type Super = NSURLResponse;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSHTTPURLResponse {}

unsafe impl Sync for NSHTTPURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSHTTPURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSHTTPURLResponse {}

unsafe impl NSObjectProtocol for NSHTTPURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSHTTPURLResponse {}

extern_methods!(
    unsafe impl NSHTTPURLResponse {
        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:statusCode:HTTPVersion:headerFields:)]
        pub unsafe fn initWithURL_statusCode_HTTPVersion_headerFields(
            this: Allocated<Self>,
            url: &NSURL,
            status_code: NSInteger,
            http_version: Option<&NSString>,
            header_fields: Option<&NSDictionary<NSString, NSString>>,
        ) -> Option<Retained<Self>>;

        #[method(statusCode)]
        pub unsafe fn statusCode(&self) -> NSInteger;

        #[cfg(feature = "NSDictionary")]
        #[method_id(@__retain_semantics Other allHeaderFields)]
        pub unsafe fn allHeaderFields(&self) -> Retained<NSDictionary>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForHTTPHeaderField:)]
        pub unsafe fn valueForHTTPHeaderField(
            &self,
            field: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForStatusCode:)]
        pub unsafe fn localizedStringForStatusCode(status_code: NSInteger) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSURLResponse`
    unsafe impl NSHTTPURLResponse {
        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:MIMEType:expectedContentLength:textEncodingName:)]
        pub unsafe fn initWithURL_MIMEType_expectedContentLength_textEncodingName(
            this: Allocated<Self>,
            url: &NSURL,
            mime_type: Option<&NSString>,
            length: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHTTPURLResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);