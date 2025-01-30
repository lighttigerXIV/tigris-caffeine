use std::path::PathBuf;

use tigris_rs::features::extensions::get_extension_dir;

pub fn get_icon(name: &str) -> PathBuf {
    get_extension_dir("caffeine")
        .unwrap()
        .join(format!("src/icons/{name}.svg"))
}
