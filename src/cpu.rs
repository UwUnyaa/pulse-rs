use sscanf::scanf;
use std::collections::HashMap;
use std::fs;

use crate::system;

pub const MAX_CPUS: u32 = 256;

#[derive(Clone, Debug)]
pub struct CPUStat {
    pub user: u32,
    pub nice: u32,
    pub system: u32,
    pub idle: u32,
    pub iowait: u32,
    pub irq: u32,
    pub softirq: u32,
    pub steal: u32,
    pub guest: u32,
    pub guest_nice: u32,
}

#[derive(Clone, Debug)]
pub struct CPUInfo {
    pub prev_stat: CPUStat,
    pub curr_stat: CPUStat,
    pub enabled: bool,
    pub usage: f64,
}

pub fn get_cpu_count() -> u32 {
    let mut count: u32 = 1;

    loop {
        let dir_name = format!("/sys/devices/system/cpu/cpu{}", count);
        if system::directory_exists(&dir_name) {
            count += 1;
        } else {
            break;
        }
    }

    if count > MAX_CPUS {
        panic!("Too many processors, something is wrong");
    }

    return count;
}

pub fn get_cpu_max_frequency() -> u32 {
    let contents = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .expect("Cannot read cpu frequency file");

    let line = contents.lines().next();

    if let Some(frequency) = line {
        return frequency.parse::<u32>().unwrap();
    } else {
        panic!("Malformed frequency");
    }
}

pub fn parse_cpuinfo() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let contents = fs::read_to_string("/proc/cpuinfo").expect("Cannot read cpuinfo file.");

    for line in contents.lines() {
        // Ignore info about CPUs other than the first one
        if line == "" {
            break;
        }

        let parsed = scanf!(line, "{/[^\\t]+/}{/\\t+: /}{}", String, String, String);

        if let Some((key, _, value)) = parsed {
            map.insert(key, value);
        }
    }

    return map;
}

pub fn get_cpu_enable_state(nth_cpu: u32) -> bool {
    let file_name = format!("/sys/devices/system/cpu/cpu{}/online", nth_cpu);
    let contents = match fs::read_to_string(&file_name) {
        Ok(content) => content,
        // Assume CPU is enabled if online file doesn't exist
        // FIXME: match a proper error here
        Err(_) => return true,
    };

    return contents.chars().nth(0) == Some('1');
}

pub fn get_cpu_stats(cpu_infos: &mut Vec<CPUInfo>) {
    let stat_contents =
        fs::read_to_string("/proc/stat").expect("Couldn't read processor stat file.");

    let mut lines = stat_contents.lines();

    // skip the first line
    lines.next();

    for cpu_info in cpu_infos {
        cpu_info.prev_stat = cpu_info.curr_stat.clone();

        let curr_stat = &mut cpu_info.curr_stat;
        let line = lines.next();

        let results = scanf!(
            line.unwrap(),
            "cpu{} {} {} {} {} {} {} {} {} {} {}",
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        );

        let _first_field;

        (
            _first_field,
            curr_stat.user,
            curr_stat.nice,
            curr_stat.system,
            curr_stat.idle,
            curr_stat.iowait,
            curr_stat.irq,
            curr_stat.softirq,
            curr_stat.steal,
            curr_stat.guest,
            curr_stat.guest_nice,
        ) = results.unwrap();
    }
}

pub fn get_cpu_usage(cpu_info: &mut CPUInfo) -> f64 {
    let prev = &cpu_info.prev_stat;
    let curr = &cpu_info.curr_stat;

    let work: f64 = f64::from(
        (curr.user
            + curr.nice
            + curr.system
            + curr.irq
            + curr.softirq
            + curr.guest
            + curr.guest_nice)
            - (prev.user
                + prev.nice
                + prev.system
                + prev.irq
                + prev.softirq
                + prev.guest
                + prev.guest_nice),
    );

    let total: f64 = work
        + f64::from(
            (curr.idle - prev.idle) + (curr.steal - prev.steal) + (curr.iowait - prev.iowait),
        );

    let usage = work / total;

    cpu_info.usage = usage;
    return usage;
}

pub fn init_infos(cpu_count: u32) -> Vec<CPUInfo> {
    let mut cpu_infos = Vec::with_capacity(MAX_CPUS as usize);

    for cpu in 0..cpu_count {
        cpu_infos.push(CPUInfo {
            prev_stat: CPUStat {
                user: 0,
                nice: 0,
                system: 0,
                idle: 0,
                iowait: 0,
                irq: 0,
                softirq: 0,
                steal: 0,
                guest: 0,
                guest_nice: 0,
            },
            curr_stat: CPUStat {
                user: 0,
                nice: 0,
                system: 0,
                idle: 0,
                iowait: 0,
                irq: 0,
                softirq: 0,
                steal: 0,
                guest: 0,
                guest_nice: 0,
            },
            enabled: get_cpu_enable_state(cpu),
            usage: 0.0,
        });
    }

    return cpu_infos;
}
