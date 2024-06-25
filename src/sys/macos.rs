mod delegate;
mod objc2;

use ::objc2::sel;
use delegate::Delegate;
use objc2::NSAppleEventManager;

pub(crate) fn register_handler(handler: fn(&str)) {
    let shared = unsafe { NSAppleEventManager::shared() };
    let delegate = Box::leak(Box::new(Delegate::new(handler)));

    const INTERNET_EVENT_CLASS: u32 = u32::from_be_bytes(*b"GURL");
    const AE_GET_URL: u32 = u32::from_be_bytes(*b"GURL");

    unsafe {
        shared.set_event_handler(
            delegate,
            sel!(event:replyEvent:),
            INTERNET_EVENT_CLASS,
            AE_GET_URL,
        )
    };
}
