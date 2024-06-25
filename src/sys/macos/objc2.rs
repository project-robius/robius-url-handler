// TODO: Add upstream PR to objc2 to support AE framework, removing the need for
// all these declarations.

use ::objc2::{
    extern_class, extern_methods, mutability,
    rc::Retained,
    runtime::{AnyObject, NSObjectProtocol, Sel},
    ClassType,
};
use objc2_foundation::{NSObject, NSString};

extern_class!(
    pub struct NSAppleEventManager;

    unsafe impl ClassType for NSAppleEventManager {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSAppleEventManager {}

extern_methods!(
    #[allow(non_snake_case)]
    unsafe impl NSAppleEventManager {
        #[method_id(@__retain_semantics Other sharedAppleEventManager)]
        pub unsafe fn shared() -> Retained<NSAppleEventManager>;

        #[method(setEventHandler:andSelector:forEventClass:andEventID:)]
        pub unsafe fn set_event_handler(
            &self,
            handler: &AnyObject,
            and_selector: Sel,
            for_event_class: u32,
            and_event_id: u32,
        );
    }
);

extern_class!(
    pub struct NSAppleEventDescriptor;

    unsafe impl ClassType for NSAppleEventDescriptor {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSAppleEventDescriptor {}

extern_methods!(
    #[allow(non_snake_case)]
    unsafe impl NSAppleEventDescriptor {
        #[method_id(@__retain_semantics Other paramDescriptorForKeyword:)]
        pub unsafe fn param_descriptor(
            &self,
            for_keyword: u32,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn string_value(&self) -> Option<Retained<NSString>>;
    }
);
