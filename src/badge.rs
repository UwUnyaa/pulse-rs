use gdk_pixbuf::{Colorspace, Pixbuf};

const BADGE_SIZE: i32 = 128;
const CPU_FREQUENCY_POWERS: [char; 4] = ['k', 'M', 'G', 'T'];

pub fn create_badge_image() -> Option<Pixbuf> {
    let pixbuf = Pixbuf::new(Colorspace::Rgb, true, 8, BADGE_SIZE, BADGE_SIZE).unwrap();
    // FIXME: implement badge drawing

    pixbuf.fill(0xffffffff);

    return Some(pixbuf);
}

// FIXME: this shouldn't be public, fix after debugging
pub fn normalize_cpu_frequency(frequency: u32) -> String {
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
