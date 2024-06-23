use std::mem;

use objc2::{
    declare_class, extern_class, extern_methods, msg_send_id, mutability,
    rc::{autoreleasepool, Allocated, Id, Retained},
    runtime::{AnyObject, NSObjectProtocol, Sel},
    sel, ClassType, DeclaredClass,
};
use objc2_foundation::{NSObject, NSString};

pub fn register_handler(handler: fn(&str)) {
    let shared = unsafe { NSAppleEventManager::sharedAppleEventManager() };
    // TODO: Don't leak?
    let delegate = Box::leak(Box::new(Delegate::new(handler)));

    // TODO: Explain
    const SPECIAL_SAUCE: u32 = u32::from_be_bytes(*b"GURL");

    unsafe {
        shared.setEventHandler(
            delegate,
            sel!(event:replyEvent:),
            SPECIAL_SAUCE,
            SPECIAL_SAUCE,
        )
    };
}

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
        pub unsafe fn sharedAppleEventManager() -> Retained<NSAppleEventManager>;

        #[method(setEventHandler:andSelector:forEventClass:andEventID:)]
        pub unsafe fn setEventHandler(
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

struct Ivars {
    handler: fn(&str),
}

declare_class!(
    struct Delegate;

    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "RobiusUrlHandlerDelegate";
    }

    impl DeclaredClass for Delegate {
        type Ivars = Ivars;
    }

    unsafe impl Delegate {
        #[method_id(initWithHandler:)]
        fn init_with(this: Allocated<Self>, handler: usize) -> Option<Id<Self>> {
            let this = this.set_ivars(Ivars {
                // https://doc.rust-lang.org/std/primitive.fn.html#casting-to-and-from-integers
                handler: unsafe { mem::transmute::<*const (), fn(&str)>(handler as *const ())},
            });
            unsafe { msg_send_id![super(this), init] }
        }

        #[method(event:replyEvent:)]
        fn handle_event(&self, event: &NSAppleEventDescriptor, _: &NSAppleEventDescriptor) {
            const KEY_DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

            let ns_string = unsafe { event.param_descriptor(KEY_DIRECT_OBJECT) }
                .map(|descriptor| unsafe { descriptor.string_value() })
                .flatten();

            let handler = self.ivars().handler;

            if let Some(ns_string) = ns_string {
                autoreleasepool(|pool| {
                    handler(ns_string.as_str(pool));
                });
            } else {
                handler("stinky");
            }

        }
    }

    unsafe impl NSObjectProtocol for Delegate {}
);

impl Delegate {
    fn new(handler: fn(&str)) -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), initWithHandler: handler as usize] }
    }
}
