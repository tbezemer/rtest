//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CNEntityType(pub NSInteger);
impl CNEntityType {
    #[doc(alias = "CNEntityTypeContacts")]
    pub const Contacts: Self = Self(0);
}

unsafe impl Encode for CNEntityType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CNEntityType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CNAuthorizationStatus(pub NSInteger);
impl CNAuthorizationStatus {
    #[doc(alias = "CNAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "CNAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(1);
    #[doc(alias = "CNAuthorizationStatusDenied")]
    pub const Denied: Self = Self(2);
    #[doc(alias = "CNAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for CNAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CNAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactStore;

    unsafe impl ClassType for CNContactStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CNContactStore {}

extern_methods!(
    unsafe impl CNContactStore {
        #[method(authorizationStatusForEntityType:)]
        pub unsafe fn authorizationStatusForEntityType(
            entity_type: CNEntityType,
        ) -> CNAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(requestAccessForEntityType:completionHandler:)]
        pub unsafe fn requestAccessForEntityType_completionHandler(
            &self,
            entity_type: CNEntityType,
            completion_handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other unifiedContactsMatchingPredicate:keysToFetch:error:_)]
        pub unsafe fn unifiedContactsMatchingPredicate_keysToFetch_error(
            &self,
            predicate: &NSPredicate,
            keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Result<Retained<NSArray<CNContact>>, Retained<NSError>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other unifiedContactWithIdentifier:keysToFetch:error:_)]
        pub unsafe fn unifiedContactWithIdentifier_keysToFetch_error(
            &self,
            identifier: &NSString,
            keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Result<Retained<CNContact>, Retained<NSError>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other unifiedMeContactWithKeysToFetch:error:_)]
        pub unsafe fn unifiedMeContactWithKeysToFetch_error(
            &self,
            keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Result<Retained<CNContact>, Retained<NSError>>;

        #[cfg(all(
            feature = "CNContact",
            feature = "CNContactFetchRequest",
            feature = "CNFetchRequest",
            feature = "CNFetchResult"
        ))]
        #[method_id(@__retain_semantics Other enumeratorForContactFetchRequest:error:_)]
        pub unsafe fn enumeratorForContactFetchRequest_error(
            &self,
            request: &CNContactFetchRequest,
        ) -> Result<Retained<CNFetchResult<NSEnumerator<CNContact>>>, Retained<NSError>>;

        #[cfg(all(
            feature = "CNChangeHistoryEvent",
            feature = "CNChangeHistoryFetchRequest",
            feature = "CNFetchRequest",
            feature = "CNFetchResult"
        ))]
        #[method_id(@__retain_semantics Other enumeratorForChangeHistoryFetchRequest:error:_)]
        pub unsafe fn enumeratorForChangeHistoryFetchRequest_error(
            &self,
            request: &CNChangeHistoryFetchRequest,
        ) -> Result<Retained<CNFetchResult<NSEnumerator<CNChangeHistoryEvent>>>, Retained<NSError>>;

        #[cfg(all(
            feature = "CNContact",
            feature = "CNContactFetchRequest",
            feature = "CNFetchRequest",
            feature = "block2"
        ))]
        #[method(enumerateContactsWithFetchRequest:error:usingBlock:)]
        pub unsafe fn enumerateContactsWithFetchRequest_error_usingBlock(
            &self,
            fetch_request: &CNContactFetchRequest,
            error: Option<&mut Option<Retained<NSError>>>,
            block: &block2::Block<dyn Fn(NonNull<CNContact>, NonNull<Bool>) + '_>,
        ) -> bool;

        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other groupsMatchingPredicate:error:_)]
        pub unsafe fn groupsMatchingPredicate_error(
            &self,
            predicate: Option<&NSPredicate>,
        ) -> Result<Retained<NSArray<CNGroup>>, Retained<NSError>>;

        #[cfg(feature = "CNContainer")]
        #[method_id(@__retain_semantics Other containersMatchingPredicate:error:_)]
        pub unsafe fn containersMatchingPredicate_error(
            &self,
            predicate: Option<&NSPredicate>,
        ) -> Result<Retained<NSArray<CNContainer>>, Retained<NSError>>;

        #[cfg(feature = "CNSaveRequest")]
        #[method(executeSaveRequest:error:_)]
        pub unsafe fn executeSaveRequest_error(
            &self,
            save_request: &CNSaveRequest,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other currentHistoryToken)]
        pub unsafe fn currentHistoryToken(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other defaultContainerIdentifier)]
        pub unsafe fn defaultContainerIdentifier(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static CNContactStoreDidChangeNotification: &'static NSString;
}