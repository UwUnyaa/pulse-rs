// use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow};

pub mod cpu;
pub mod system;

fn main() {
    // let app = Application::builder()
    //     .application_id("org.uwunyaa.pulse")
    //     .build();

    // app.connect_activate(|app| {
    //     // create the window
    //     let window = ApplicationWindow::builder()
    //         .application(app)
    //         .title("Pulse")
    //         .build();

    //     // show the window
    //     window.show_all();
    // });

    let cpu_count = cpu::get_cpu_count();

    let mut cpu_infos = cpu::init_infos(cpu_count);

    // FIXME: remove debug code once it's not needed
    println!("CPU count: {}", cpu_count);

    cpu::get_cpu_stats(&mut cpu_infos);

    for i in 0..cpu_infos.len() {
        // println!("CPU{} enabled: {}", i, cpu_infos[i].enabled);
        cpu::get_cpu_usage(&mut cpu_infos[i]);
        // println!("CPU{} usage: {}", i, cpu_infos[i].usage);
        // dbg!(&cpu_infos[i].curr_stat);
    }

    dbg!(cpu::parse_cpuinfo());

    // app.run();
}
