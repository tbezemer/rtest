//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIIndirectScribbleInteraction;

    unsafe impl ClassType for UIIndirectScribbleInteraction {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIIndirectScribbleInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIIndirectScribbleInteraction {}

extern_methods!(
    unsafe impl UIIndirectScribbleInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: &ProtocolObject<dyn UIIndirectScribbleInteractionDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIIndirectScribbleInteractionDelegate>>>;

        #[method(isHandlingWriting)]
        pub unsafe fn isHandlingWriting(&self) -> bool;
    }
);

pub type UIScribbleElementIdentifier = TodoProtocols;

extern_protocol!(
    pub unsafe trait UIIndirectScribbleInteractionDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[method(indirectScribbleInteraction:requestElementsInRect:completion:)]
        unsafe fn indirectScribbleInteraction_requestElementsInRect_completion(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            rect: CGRect,
            completion: &block2::Block<dyn Fn(NonNull<NSArray<UIScribbleElementIdentifier>>)>,
        );

        #[method(indirectScribbleInteraction:isElementFocused:)]
        unsafe fn indirectScribbleInteraction_isElementFocused(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            element_identifier: &UIScribbleElementIdentifier,
        ) -> bool;

        #[method(indirectScribbleInteraction:frameForElement:)]
        unsafe fn indirectScribbleInteraction_frameForElement(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            element_identifier: &UIScribbleElementIdentifier,
        ) -> CGRect;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "block2"
        ))]
        #[method(indirectScribbleInteraction:focusElementIfNeeded:referencePoint:completion:)]
        unsafe fn indirectScribbleInteraction_focusElementIfNeeded_referencePoint_completion(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            element_identifier: &UIScribbleElementIdentifier,
            focus_reference_point: CGPoint,
            completion: &block2::Block<dyn Fn(*mut UIResponder)>,
        );

        #[optional]
        #[method(indirectScribbleInteraction:shouldDelayFocusForElement:)]
        unsafe fn indirectScribbleInteraction_shouldDelayFocusForElement(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            element_identifier: &UIScribbleElementIdentifier,
        ) -> bool;

        #[optional]
        #[method(indirectScribbleInteraction:willBeginWritingInElement:)]
        unsafe fn indirectScribbleInteraction_willBeginWritingInElement(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            element_identifier: &UIScribbleElementIdentifier,
        );

        #[optional]
        #[method(indirectScribbleInteraction:didFinishWritingInElement:)]
        unsafe fn indirectScribbleInteraction_didFinishWritingInElement(
            &self,
            interaction: &UIIndirectScribbleInteraction,
            element_identifier: &UIScribbleElementIdentifier,
        );
    }

    unsafe impl ProtocolType for dyn UIIndirectScribbleInteractionDelegate {}
);