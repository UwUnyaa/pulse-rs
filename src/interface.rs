use gtk;
use gtk::glib;
use gtk::prelude::{BoxExt, ContainerExt, ProgressBarExt, ToggleButtonExt, WidgetExt};
use gtk::{Align, ApplicationWindow, Box, Orientation, ProgressBar, ToggleButton};

use crate::badge;
use crate::cpu;

const BORDER_SIZE: i32 = 4;

pub struct CPUInterface {
    toggle: ToggleButton,
    usage_bar: ProgressBar,
}

pub fn update_usage_handler(
    interfaces: &Vec<CPUInterface>,
    infos: &mut Vec<cpu::CPUInfo>,
) -> glib::source::Continue {
    cpu::get_cpu_stats(infos);

    for i in 0..infos.len() {
        let usage = cpu::get_cpu_usage(&mut infos[i]).clamp(0.0, 1.0);

        interfaces[i].usage_bar.set_fraction(usage);
    }

    return glib::source::Continue(true);
}

pub fn init_interface(
    window: &ApplicationWindow,
    cpu_infos: &Vec<cpu::CPUInfo>,
) -> Vec<CPUInterface> {
    let top_hbox = Box::new(Orientation::Horizontal, BORDER_SIZE);

    window.add(&top_hbox);

    let image = badge::create_badge_image();
    image.set_halign(Align::Start);
    image.set_valign(Align::Start);

    top_hbox.pack_start(&image, false, false, 0);

    let cpus_vbox = Box::new(Orientation::Vertical, 2 * BORDER_SIZE);
    top_hbox.pack_start(&cpus_vbox, true, true, 0);

    let mut interfaces = Vec::with_capacity(cpu::MAX_CPUS as usize);

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

        interfaces.push(CPUInterface {
            toggle: button,
            usage_bar: progress_bar,
        });
    }

    return interfaces;
}
