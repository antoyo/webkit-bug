#[macro_use]
extern crate webkit2gtk_webextension;

use glib::Cast;
use glib::variant::Variant;
use webkit2gtk_webextension::{DOMDocumentExt, DOMEventTargetExt, DOMMouseEvent, DOMMouseEventExt, WebExtension};

web_extension_init!();

#[no_mangle]
pub fn web_extension_initialize(extension: WebExtension, user_data: Variant) {
    let _string = user_data.get_str();

    extension.connect_page_created(|_, page| {
        page.connect_document_loaded(|page| {
            let document = page.get_dom_document().unwrap();
            if let Some(link) = document.get_element_by_id("link") {
                let event = document.create_event("MouseEvent").unwrap();
                let window = document.get_default_view().unwrap();
                let event = event.downcast::<DOMMouseEvent>().unwrap();
                event.init_mouse_event("click", true, true, &window, 0, 0, 0, 0, 0, false, false, false, false, 0, &link);
                link.dispatch_event(&event).unwrap();
            }
        });
    });
}
