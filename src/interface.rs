use gtk4;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Align, ApplicationWindow, Box, Orientation, ProgressBar, ToggleButton};

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
) -> glib::ControlFlow {
    cpu::get_cpu_stats(infos);

    for i in 0..infos.len() {
        let usage = cpu::get_cpu_usage(&mut infos[i]).clamp(0.0, 1.0);

        interfaces[i].usage_bar.set_fraction(usage);
    }

    return glib::ControlFlow::Continue;
}

pub fn init_interface(
    window: &ApplicationWindow,
    cpu_infos: &Vec<cpu::CPUInfo>,
) -> Vec<CPUInterface> {
    let top_hbox = Box::new(Orientation::Horizontal, BORDER_SIZE);

    window.set_child(Some(&top_hbox));

    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    let display = gtk4::gdk::Display::default().expect("Couldn't get default display");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let image = badge::create_badge_image();
    image.set_halign(Align::Start);
    image.set_valign(Align::Start);

    top_hbox.append(&image);

    let cpus_vbox = Box::new(Orientation::Vertical, 2 * BORDER_SIZE);
    top_hbox.append(&cpus_vbox);

    let mut interfaces = Vec::with_capacity(cpu::MAX_CPUS as usize);

    for num_cpu in 0..cpu_infos.len() {
        let cpu_info = &cpu_infos[num_cpu];

        let hbox = Box::new(Orientation::Horizontal, BORDER_SIZE);
        cpus_vbox.append(&hbox);

        let button = ToggleButton::with_label(&format!("{}", num_cpu));
        button.set_active(cpu_info.enabled);

        // FIXME: progressbar doesn't have proper height
        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(cpu_info.usage);

        hbox.append(&button);
        hbox.append(&progress_bar);

        interfaces.push(CPUInterface {
            toggle: button,
            usage_bar: progress_bar,
        });
    }

    return interfaces;
}
