use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub back:      Button,
    pub next:      Button,
}

impl Header {
    pub fn new() -> Header {
        let back = cascade! {
            Button::new_with_label("Cancel");
            ..get_style_context().map(|c| c.add_class("back"));
        };

        let next = cascade! {
            Button::new_with_label("Next");
            ..set_sensitive(false);
            ..get_style_context().map(|c| c.add_class(&STYLE_CLASS_SUGGESTED_ACTION));
        };

        // Returns the header and all of it's state
        Header {
            container: cascade! {
                HeaderBar::new();
                ..set_title("USB Flasher");
                ..pack_start(&back);
                ..pack_end(&next);
            },
            back,
            next,
        }
    }
}
