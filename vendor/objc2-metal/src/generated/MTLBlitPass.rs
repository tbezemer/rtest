//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptor;

    unsafe impl ClassType for MTLBlitPassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLBlitPassSampleBufferAttachmentDescriptor {}

unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptor {
        #[cfg(feature = "MTLCounters")]
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        #[cfg(feature = "MTLCounters")]
        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(
            &self,
            start_of_encoder_sample_index: NSUInteger,
        );

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray;

    unsafe impl ClassType for MTLBlitPassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Retained<MTLBlitPassSampleBufferAttachmentDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLBlitPassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassDescriptor;

    unsafe impl ClassType for MTLBlitPassDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLBlitPassDescriptor {}

unsafe impl NSObjectProtocol for MTLBlitPassDescriptor {}

extern_methods!(
    unsafe impl MTLBlitPassDescriptor {
        #[method_id(@__retain_semantics Other blitPassDescriptor)]
        pub unsafe fn blitPassDescriptor() -> Retained<MTLBlitPassDescriptor>;

        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Retained<MTLBlitPassSampleBufferAttachmentDescriptorArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLBlitPassDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);