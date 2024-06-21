use std::mem;

use objc2::{
    declare_class, extern_class, extern_methods, msg_send_id, mutability,
    rc::{Allocated, Id, Retained},
    runtime::{AnyObject, NSObjectProtocol, Sel},
    sel, ClassType, DeclaredClass,
};
use objc2_foundation::NSObject;

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
        #[method_id(@__retain_semantics Other paramDescriptor:)]
        pub unsafe fn paramDescriptor(
            &self,
            for_keyword: u32,
        ) -> Option<Retained<NSAppleEventDescriptor>>;
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
        fn handle_event(&self, _event: &NSAppleEventDescriptor, _: &NSAppleEventDescriptor) {
            let handler = self.ivars().handler;

            // const K_INTERNET_EVENT_CLASS: &[u8; 4] = b"----";
            // const THING: u32 = u32::from_be_bytes(*K_INTERNET_EVENT_CLASS);
            // let r = unsafe { event.paramDescriptor(THING) };
            // let s = format!("{:?}", r.is_none());

            handler("hello");
        }
    }

    unsafe impl NSObjectProtocol for Delegate {}
);

impl Delegate {
    fn new(handler: fn(&str)) -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), initWithHandler: handler as usize] }
    }
}
