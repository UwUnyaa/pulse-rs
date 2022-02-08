use crate::cpu;

use std::collections::HashMap;

use gtk::cairo::{Context as CairoContext, ImageSurface};
use gtk::prelude::WidgetExt;
use gtk::{Image, Inhibit};
use pango;
use pango::{Alignment, FontDescription};
use pangocairo;
use std::fs::File;

const CPU_FREQUENCY_POWERS: [char; 4] = ['k', 'M', 'G', 'T'];
const BADGE_SIZE: i32 = 128;

fn draw_badge_text(cr: &CairoContext, label: &String, ypos: f64, font_size: i32) {
    let layout = pangocairo::create_layout(&cr).unwrap();
    layout.set_text(&label);
    layout.set_font_description(Some(&FontDescription::from_string(&format!(
        "sans {}",
        font_size
    ))));
    layout.set_alignment(Alignment::Center);
    layout.set_width(pango::units_from_double((BADGE_SIZE - 16).into()));
    pangocairo::update_layout(cr, &layout);
    cr.set_source_rgb(0.89453125, 0.89453125, 0.89453125);
    cr.move_to(8.0, ypos);
    pangocairo::show_layout(cr, &layout);
}

pub fn create_badge_image() -> Image {
    let image = Image::builder()
        .width_request(BADGE_SIZE)
        .height_request(BADGE_SIZE)
        .build();

    let cpu_stats = cpu::parse_cpuinfo();

    image.connect_draw(move |_image, cr| {
        // TODO: fix path handling, consider embedding the image directly into
        // the binary
        let image_surface =
            ImageSurface::create_from_png(&mut File::open("src/badge.png").unwrap()).unwrap();

        // Draw the background image onto the surface
        cr.set_source_surface(&image_surface, 0.0, 0.0)
            .expect("Couldn't set source surface for the badge");
        cr.rectangle(0.0, 0.0, BADGE_SIZE.into(), BADGE_SIZE.into());
        cr.paint()
            .expect("Painting the image onto the badge failed");

        // draw text
        draw_badge_text(
            cr,
            &normalize_vendor_name(&cpu_stats.get("vendor_id").unwrap().to_string()).unwrap(),
            8.0,
            20,
        );
        draw_badge_text(cr, cpu_stats.get("model name").unwrap(), 40.0, 8);
        draw_badge_text(
            cr,
            &normalize_cpu_frequency(cpu::get_cpu_max_frequency()),
            108.0,
            8,
        );

        return Inhibit(true);
    });

    return image;
}

fn normalize_vendor_name(vendor_name: &String) -> Option<String> {
    // FIXME: make this cleaner, an external crate might be necessary to store
    // a compile-time hash table
    let vendor_map = HashMap::from([
        ("AMDisbetter!".to_string(), "AMD".to_string()),
        ("AuthenticAMD".to_string(), "AMD".to_string()),
        ("CentaurHauls".to_string(), "Centaur".to_string()),
        ("CyrixInstead".to_string(), "Cyrix".to_string()),
        ("GenuineIntel".to_string(), "Intel".to_string()),
        ("TransmetaCPU".to_string(), "Transmeta".to_string()),
        ("GenuineTMx86".to_string(), "Transmeta".to_string()),
        (
            "Geode by NSC".to_string(),
            "National Semiconductor".to_string(),
        ),
        ("NexGenDriven".to_string(), "NexGen".to_string()),
        ("RiseRiseRise".to_string(), "Rise".to_string()),
        ("SiS SiS SiS ".to_string(), "SiS".to_string()),
        ("UMC UMC UMC ".to_string(), "UMC".to_string()),
        ("VIA VIA VIA ".to_string(), "VIA".to_string()),
        ("Vortex86 SoC".to_string(), "DM&P Vortex86".to_string()),
        ("  Shanghai  ".to_string(), "Zhaoxin".to_string()),
        ("HygonGenuine".to_string(), "Hygon".to_string()),
        ("E2K MACHINE".to_string(), "MCST Elbrus".to_string()),
        ("MiSTer AO486".to_string(), "ao486".to_string()),
        ("bhyve bhyve ".to_string(), "bhyve".to_string()),
        (" KVMKVMKVM  ".to_string(), "KVM".to_string()),
        ("TCGTCGTCGTCG".to_string(), "QEMU".to_string()),
        ("Microsoft Hv".to_string(), "Hyper-V".to_string()),
        (" lrpepyh  vr".to_string(), "Parallels".to_string()),
        ("VMwareVMware".to_string(), "VMware".to_string()),
        ("XenVMMXenVMM".to_string(), "Xen HVM".to_string()),
        ("ACRNACRNACRN".to_string(), "ACRN".to_string()),
        (" QNXQVMBSQG ".to_string(), "QNX".to_string()),
    ]);

    let normalized = vendor_map.get(vendor_name);

    if let Some(name) = normalized {
        return Some(name.to_string());
    }

    return None;
}

fn normalize_cpu_frequency(frequency: u32) -> String {
    let mut powers = 0;
    let mut result_frequency = f64::from(frequency);

    while result_frequency > 1000.0 {
        result_frequency /= 1000.0;
        powers += 1;
    }

    if powers > CPU_FREQUENCY_POWERS.len() {
        result_frequency = f64::from(frequency);
        powers = 0;
    }

    return format!("{:.2} {}Hz", result_frequency, CPU_FREQUENCY_POWERS[powers]);
}
