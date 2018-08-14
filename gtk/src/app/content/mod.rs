mod devices;
mod error;
mod flashing;
mod images;
mod summary;
mod view;

pub use self::devices::{DeviceList, DevicesView};
pub use self::error::ErrorView;
pub use self::flashing::FlashView;
pub use self::images::ImageView;
pub use self::summary::SummaryView;
pub use self::view::View;

use gtk::*;

use super::set_margins;

pub struct Content {
    pub container:    Stack,
    pub image_view:   ImageView,
    pub devices_view: DevicesView,
    pub error_view:   ErrorView,
    pub flash_view:   FlashView,
    pub summary_view: SummaryView,
}

impl Content {
    pub fn new() -> Content {
        let image_view = ImageView::new();
        let devices_view = DevicesView::new();
        let flash_view = FlashView::new();
        let summary_view = SummaryView::new();
        let error_view = ErrorView::new();

        let container = cascade! {
            container: Stack::new();
            ..add_named(&image_view.view.container, "image");
            ..add_named(&devices_view.view.container, "devices");
            ..add_named(&flash_view.view.container, "flash");
            ..add_named(&summary_view.view.container, "summary");
            ..add_named(&error_view.view.container, "error");
            ..set_visible_child_name("image");
            | set_margins(&container, 12);
        };

        Content {
            container,
            image_view,
            devices_view,
            flash_view,
            summary_view,
            error_view,
        }
    }
}
