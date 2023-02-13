use std::path::Path;

pub fn is_running_in_container() -> bool {
    Path::new("/.dockerenv").exists()
}
