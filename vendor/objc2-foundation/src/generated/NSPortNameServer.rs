//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSPortNameServer;

    unsafe impl ClassType for NSPortNameServer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSPortNameServer {}

extern_methods!(
    unsafe impl NSPortNameServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other systemDefaultPortNameServer)]
        pub unsafe fn systemDefaultPortNameServer() -> Retained<NSPortNameServer>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removePortForName:)]
        pub unsafe fn removePortForName(&self, name: &NSString) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPortNameServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSMachBootstrapServer;

    unsafe impl ClassType for NSMachBootstrapServer {
        #[inherits(NSObject)]
        type Super = NSPortNameServer;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSMachBootstrapServer {}

extern_methods!(
    unsafe impl NSMachBootstrapServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Retained<AnyObject>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[method_id(@__retain_semantics Other servicePortWithName:)]
        pub unsafe fn servicePortWithName(&self, name: &NSString) -> Option<Retained<NSPort>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMachBootstrapServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSMessagePortNameServer;

    unsafe impl ClassType for NSMessagePortNameServer {
        #[inherits(NSObject)]
        type Super = NSPortNameServer;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSMessagePortNameServer {}

extern_methods!(
    unsafe impl NSMessagePortNameServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Retained<AnyObject>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Retained<NSPort>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMessagePortNameServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSSocketPortNameServer;

    unsafe impl ClassType for NSSocketPortNameServer {
        #[inherits(NSObject)]
        type Super = NSPortNameServer;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSSocketPortNameServer {}

extern_methods!(
    unsafe impl NSSocketPortNameServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Retained<AnyObject>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removePortForName:)]
        pub unsafe fn removePortForName(&self, name: &NSString) -> bool;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:nameServerPortNumber:)]
        pub unsafe fn portForName_host_nameServerPortNumber(
            &self,
            name: &NSString,
            host: Option<&NSString>,
            port_number: u16,
        ) -> Option<Retained<NSPort>>;

        #[cfg(all(feature = "NSPort", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:nameServerPortNumber:)]
        pub unsafe fn registerPort_name_nameServerPortNumber(
            &self,
            port: &NSPort,
            name: &NSString,
            port_number: u16,
        ) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(defaultNameServerPortNumber)]
        pub unsafe fn defaultNameServerPortNumber(&self) -> u16;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setDefaultNameServerPortNumber:)]
        pub unsafe fn setDefaultNameServerPortNumber(&self, default_name_server_port_number: u16);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSocketPortNameServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);