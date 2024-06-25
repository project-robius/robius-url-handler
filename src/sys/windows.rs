// TODO: Test

use std::mem;

use windows::{
    ApplicationModel::{
        Activation::{ActivationKind, IActivatedEventArgs, ProtocolActivatedEventArgs},
        Core::{CoreApplication, CoreApplicationView},
    },
    Foundation::TypedEventHandler,
};

pub(crate) fn register_handler(handler: fn(&str)) {
    let view = CoreApplication::GetCurrentView().unwrap();

    let event_handler: TypedEventHandler<CoreApplicationView, IActivatedEventArgs> =
        TypedEventHandler::new(
            move |_: &Option<CoreApplicationView>, args: &Option<IActivatedEventArgs>| {
                if let Some(args) = args {
                    if let Ok(ActivationKind::Protocol) = args.Kind() {
                        let _: &IActivatedEventArgs = args;
                        // FIXME: This cannot be ok.
                        let casted_args: &ProtocolActivatedEventArgs =
                            unsafe { mem::transmute(args) };

                        // TODO: Call handler with error if not ok?
                        if let Ok(hstring) = casted_args.Uri().and_then(|uri| uri.RawUri()) {
                            handler(&hstring.to_string())
                        }
                    }
                }
                Ok(())
            },
        );
    let _token = view.Activated(&event_handler).unwrap();
}
