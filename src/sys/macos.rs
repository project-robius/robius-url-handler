mod delegate;
mod objc2;

use ::objc2::sel;
use delegate::Delegate;
use objc2::NSAppleEventManager;

pub(crate) fn register_handler(handler: fn(&str)) {
    let shared = unsafe { NSAppleEventManager::shared() };
    // TODO: Don't leak?
    // TODO: Do we want to return an error if?
    let delegate = Box::leak(Box::new(Delegate::new(handler)));

    // TODO: Explain
    const SPECIAL_SAUCE: u32 = u32::from_be_bytes(*b"GURL");

    unsafe {
        shared.set_event_handler(
            delegate,
            sel!(event:replyEvent:),
            SPECIAL_SAUCE,
            SPECIAL_SAUCE,
        )
    };
}
