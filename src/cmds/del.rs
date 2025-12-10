use std::{fs::{self, File}, io::Read};

use dialoguer::{Confirm};

use crate::config;

pub fn delete_profile(slug: String) {
    let mut conf_dir = config::get_config_dir().unwrap();
    conf_dir.push(format!("{}.toml", slug));
    if conf_dir.exists() {
        let mut f: File = File::open(&conf_dir).expect("Failed to open profile.");
        let mut contents: String = String::new();
        f.read_to_string(&mut contents).expect("Failed to read profile.");
        
        println!("{}", contents);
        let confirm = Confirm::new()
            .with_prompt("Are you sure you want to delete this profile? This action cannot be undone!")
            .default(false)
            .interact();
        
        if confirm.unwrap() {
            let _ = fs::remove_file(&conf_dir);
        }
    }
}
