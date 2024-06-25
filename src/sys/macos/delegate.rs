use core::mem;

use ::objc2::{
    declare_class, msg_send_id, mutability,
    rc::{autoreleasepool, Allocated, Id},
    runtime::NSObjectProtocol,
    ClassType, DeclaredClass,
};
use objc2_foundation::NSObject;

use super::objc2::NSAppleEventDescriptor;

pub(super) struct Ivars {
    handler: fn(&str),
}

declare_class!(
    pub(super) struct Delegate;

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
                .and_then(|descriptor| unsafe { descriptor.string_value() });

            let handler = self.ivars().handler;

            if let Some(ns_string) = ns_string {
                autoreleasepool(|pool| {
                    handler(ns_string.as_str(pool));
                });
            }
        }
    }

    unsafe impl NSObjectProtocol for Delegate {}
);

impl Delegate {
    pub(super) fn new(handler: fn(&str)) -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), initWithHandler: handler as usize] }
    }
}
