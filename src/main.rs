use gtk::glib::source;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::time::Duration;

pub mod badge;
pub mod cpu;
pub mod interface;
pub mod system;

fn main() {
    let app = Application::builder().build();

    app.connect_activate(|app| {
        // create the window
        // FIXME: move constants to some sort of config
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Pulse")
            .resizable(false)
            // .border_size(4)
            .build();

        let cpu_count = cpu::get_cpu_count();

        let mut cpu_infos = cpu::init_infos(cpu_count);

        cpu::get_cpu_stats(&mut cpu_infos);

        for i in 0..cpu_infos.len() {
            cpu::get_cpu_usage(&mut cpu_infos[i]);
        }

        let mut interfaces = interface::init_interface(&window, &cpu_infos);

        source::timeout_add_local(Duration::new(1, 0), move || {
            interface::update_usage_handler(&mut interfaces, &mut cpu_infos);

            return source::Continue(true);
        });

        // show the window
        window.show_all();
    });

    app.run();
}
