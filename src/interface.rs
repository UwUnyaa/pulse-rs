use gtk;
use gtk::prelude::{BoxExt, ContainerExt, ProgressBarExt, ToggleButtonExt, WidgetExt};
use gtk::{Align, ApplicationWindow, Box, Image, Orientation, ProgressBar, ToggleButton};

use crate::badge;
use crate::cpu;

const BORDER_SIZE: i32 = 4;

// struct CPUInterface {
//     toggle: Widget,
//     usage_bar: Widget,
// }

pub fn init_interface(window: &ApplicationWindow, cpu_infos: Vec<cpu::CPUInfo>) {
    let top_hbox = Box::new(Orientation::Horizontal, BORDER_SIZE);

    window.add(&top_hbox);

    let image = Image::from_pixbuf(badge::create_badge_image().as_ref());
    image.set_halign(Align::Start);
    image.set_valign(Align::Start);

    top_hbox.pack_start(&image, false, false, 0);

    let cpus_vbox = Box::new(Orientation::Vertical, 2 * BORDER_SIZE);
    top_hbox.pack_start(&cpus_vbox, true, true, 0);

    for num_cpu in 0..cpu_infos.len() {
        let cpu_info = &cpu_infos[num_cpu];

        let hbox = Box::new(Orientation::Horizontal, BORDER_SIZE);
        cpus_vbox.pack_start(&hbox, true, true, 0);

        let button = ToggleButton::with_label(&format!("{}", num_cpu));
        button.set_active(cpu_info.enabled);

        // FIXME: progressbar doesn't have proper height
        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(cpu_info.usage);

        hbox.pack_start(&button, true, true, 0);
        hbox.pack_start(&progress_bar, true, true, 0);
    }
}
