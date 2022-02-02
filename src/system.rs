use libc;
use std::path::Path;

pub fn directory_exists(path: &String) -> bool {
    return Path::new(path).is_dir();
}

pub fn user_is_root() -> bool {
    return unsafe { libc::getuid() } == 0;
}
