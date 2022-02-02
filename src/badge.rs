const CPU_FREQUENCY_POWERS: [char; 4] = ['k', 'M', 'G', 'T'];

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
