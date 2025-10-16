use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use std::time::Duration;

pub mod badge;
pub mod cpu;
pub mod helper;
pub mod interface;
pub mod system;

fn main() {
    if std::env::args().any(|arg| arg == "--helper") {
        std::process::exit(helper::helper_loop());
    }

    let (mut helper_stdin, helper_stdout, _helper_handle) = match helper::spawn_helper() {
        Ok((stdin, stdout, handle)) => (Some(stdin), Some(stdout), Some(handle)),
        Err(e) => {
            println!("Didn't spawn escalated helper");
            (None, None, None)
        }
    };

    let app = Application::builder().build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Pulse")
            .resizable(false)
            .build();

        let cpu_count = cpu::get_cpu_count();

        let mut cpu_infos = cpu::init_infos(cpu_count);

        cpu::get_cpu_stats(&mut cpu_infos);

        for i in 0..cpu_infos.len() {
            cpu::get_cpu_usage(&mut cpu_infos[i]);
        }

        let mut interfaces = interface::init_interface(&window, &cpu_infos);

        glib::timeout_add_local(Duration::new(1, 0), move || {
            interface::update_usage_handler(&mut interfaces, &mut cpu_infos);

            return glib::ControlFlow::Continue;
        });

        window.present();
    });

    app.run();
}
